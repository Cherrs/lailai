#![feature(fs_try_exists)]
#![feature(let_chains)]
#![allow(clippy::redundant_async_block)]
#[cfg(any(target_os = "macos", target_os = "windows"))]
mod captcha_window;
mod config;
mod log;
mod message_handler;
mod openai;
mod pg_store;
mod report_send;
mod sled_store;
mod store;
use crate::message_handler::MyHandler;
use ::tracing::{debug, error, info};
use config::GROUP_CONF;
use dialoguer::{console::Term, theme::ColorfulTheme, Input, Password, Select};
use fflogsv1::FF14;
use ricq::{
    client::{Connector, DefaultConnector, Token},
    device::Device,
    ext::common::after_login,
    version::{get_version, Protocol},
    Client, LoginNeedCaptcha, LoginResponse, LoginSuccess, LoginUnknownStatus,
};

use std::{env, path::Path, sync::Arc, time::Duration};
use tokio::task::JoinHandle;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //初始化配置
    config::init().await;
    log::init();
    let (handle, client) = bot_init().await;
    let logs_loop = tokio::spawn(async move {
        match GROUP_CONF.get() {
            Some(_) => {
                loop {
                    //获取logs数据，检测更新发送到群
                    if let Err(e) = report_send::send_message_init(&client).await {
                        error!("{:?}", e);
                    }
                    let interval = env::var("interval")
                        .unwrap_or_else(|_| "60".to_string())
                        .parse::<u64>()
                        .unwrap();
                    debug!("{}秒后重新查询", interval);
                    tokio::time::sleep(Duration::from_secs(interval)).await;
                }
            }
            None => {
                info!("没有读取到群配置，禁用logs警察功能");
            }
        }
    });

    handle.await.unwrap();
    logs_loop.await.unwrap();
    Ok(())
}
///初始化机器人
pub async fn bot_init() -> (JoinHandle<()>, Arc<Client>) {
    let device = match Path::new("device.json").exists() {
        true => serde_json::from_str(
            &tokio::fs::read_to_string("device.json")
                .await
                .expect("failed to read device.json"),
        )
        .expect("failed to parse device info"),
        false => {
            let d = Device::random();
            tokio::fs::write(
                "device.json",
                serde_json::to_string(&d).expect("device.json写入失败，请检查权限"),
            )
            .await
            .expect("device.json写入失败，请检查权限");
            d
        }
    };
    let token: Option<Token> = match Path::new("session.key").exists() {
        true => serde_json::from_str(
            &tokio::fs::read_to_string("session.key")
                .await
                .expect("无法读取session.key，请检查权限"),
        )
        .unwrap(),
        false => None,
    };
    let myh = MyHandler {
        ff14client: FF14::new(
            env::var("logskey")
                .unwrap_or_else(|_| "none".to_string())
                .as_str(),
        ),
    };
    let client = Arc::new(Client::new(
        device,
        get_version(Protocol::AndroidWatch),
        myh,
    ));
    let handle = tokio::spawn({
        let client = client.clone();
        let stream = DefaultConnector.connect(&client).await.unwrap();
        async move { client.start(stream).await }
    });
    tokio::task::yield_now().await; // 等一下，确保连上了
    let term = Term::stdout();
    if token.is_none() {
        let login_type = vec!["账号密码+短信验证码", "二维码"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("选择登录方式")
            .items(&login_type)
            .default(0)
            .interact_on_opt(&term)
            .unwrap()
            .unwrap();
        match login_type[selection] {
            "账号密码+短信验证码" => {
                let pwd = QQandPassword {
                    qq: Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("QQ号")
                        .interact()
                        .unwrap(),
                    password: Password::with_theme(&ColorfulTheme::default())
                        .with_prompt("密码")
                        .interact()
                        .unwrap(),
                };
                let mut resp = client.password_login(pwd.qq, &pwd.password).await.unwrap();
                loop {
                    match resp {
                        LoginResponse::Success(LoginSuccess {
                            ref account_info, ..
                        }) => {
                            info!("登录成功！🎉 {:?}", account_info);
                            break;
                        }
                        LoginResponse::DeviceLocked(x) => {
                            if let Some(message) = x.message {
                                info!("{}", message);
                            }
                            resp = client.request_sms().await.expect("failed to request sms");
                        }
                        LoginResponse::NeedCaptcha(LoginNeedCaptcha {
                            ref verify_url,
                            // 图片应该没了
                            image_captcha: ref _image_captcha,
                            ..
                        }) => {
                            term.write_line(&format!(
                                "滑块URL: {:?}",
                                verify_url.as_ref().unwrap()
                            ))
                            .unwrap();
                            #[cfg(any(target_os = "macos", target_os = "windows"))]
                            if let Some(ticket) =
                                captcha_window::ticket(verify_url.as_ref().unwrap())
                            {
                                resp = client
                                    .submit_ticket(&ticket)
                                    .await
                                    .expect("failed to submit ticket");
                            }
                            #[cfg(not(any(target_os = "macos", target_os = "windows")))]
                            {
                                let ticket: String = Input::with_theme(&ColorfulTheme::default())
                                    .with_prompt("请输入ticket")
                                    .interact()
                                    .unwrap();
                                resp = client
                                    .submit_ticket(&ticket)
                                    .await
                                    .expect("failed to submit ticket");
                            }
                        }
                        LoginResponse::DeviceLockLogin { .. } => {
                            resp = client
                                .device_lock_login()
                                .await
                                .expect("failed to login with device lock");
                        }
                        LoginResponse::TooManySMSRequest => {
                            let code: String = Input::with_theme(&ColorfulTheme::default())
                                .with_prompt("输入短信验证码")
                                .interact()
                                .unwrap();
                            resp = client.submit_sms_code(&code).await.unwrap();
                        }
                        LoginResponse::UnknownStatus(LoginUnknownStatus {
                            ref message, ..
                        }) => {
                            error!("{}", message);
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                }
            }
            "二维码" => {
                let resp = client.fetch_qrcode().await.expect("failed to fetch qrcode");
                use ricq::ext::login::auto_query_qrcode;
                match resp {
                    //登录二维码展示
                    ricq::QRCodeState::ImageFetch(x) => {
                        let img = image::load_from_memory(&x.image_data).unwrap();
                        tokio::fs::write("qrcode.jpg", &x.image_data)
                            .await
                            .expect("二维码保存失败");
                        let decoder = bardecoder::default_decoder();
                        let results = decoder.decode(&img);
                        let qr_str = results[0].as_ref().unwrap();
                        qr2term::print_qr(qr_str).unwrap();
                        println!("扫码打印出的二维码，若无法扫描打开程序目录下qrcode.jpg");
                        if let Err(err) = auto_query_qrcode(&client, &x.sig).await {
                            panic!("登录失败，请重试 {err}")
                        };
                    }
                    _ => {
                        panic!("resp error")
                    }
                }
            }
            _ => {}
        }
    } else {
        let resp = client.token_login(token.unwrap()).await.unwrap();
        match resp {
            LoginResponse::Success(LoginSuccess {
                ref account_info, ..
            }) => {
                info!("登录成功！🎉 {:?}", account_info);
            }
            _ => {
                info!("{:?}", resp);
            }
        }
    }

    after_login(&client).await;
    {
        let token = client.gen_token().await;
        let token_str = serde_json::to_vec(&token).unwrap();
        tokio::fs::write("session.key", token_str)
            .await
            .expect("无法写入session.key，请检查");
    }
    (handle, client)
}

struct QQandPassword {
    qq: i64,
    password: String,
}
