#![feature(let_chains)]
#![feature(path_try_exists)]
mod config;
mod message_handler;
mod pgstore;
mod sendreport;
mod sledstore;
mod store;
use crate::message_handler::MyHandler;
use config::GROUP_CONF;
use dialoguer::{console::Term, theme::ColorfulTheme, Input, Password, Select};
use fflogsv1::FF14;
use log::{debug, error, info};
use ricq::{
    client::Token,
    device::Device,
    ext::common::after_login,
    version::{get_version, Protocol},
    Client, LoginNeedCaptcha, LoginResponse, LoginSuccess, LoginUnknownStatus,
};
use simplelog::*;
use std::{env, path::Path, sync::Arc, time::Duration};
use tokio::{net::TcpStream, task::JoinHandle};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //初始化配置
    config::init().await;
    initlog();
    let (handle, client) = initbot().await;
    match GROUP_CONF.get() {
        Some(_) => {
            loop {
                //获取logs数据，检测更新发送到群
                match sendreport::trysendmessageorinit(&client).await {
                    Ok(_) => {}
                    Err(e) => error!("{:?}", e),
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

    handle.await.unwrap();
    Ok(())
}
///初始化机器人
pub async fn initbot() -> (JoinHandle<()>, Arc<Client>) {
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
    let client = Arc::new(Client::new(device, get_version(Protocol::IPad), myh));
    let stream = TcpStream::connect(client.get_address())
        .await
        .expect("failed to connect");
    let c = client.clone();
    let handle = tokio::spawn(async move { c.start(stream).await });
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
                let upwd = QQandPassword {
                    qq: Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("QQ号")
                        .interact()
                        .unwrap(),
                    password: Password::with_theme(&ColorfulTheme::default())
                        .with_prompt("密码")
                        .interact()
                        .unwrap(),
                };
                let mut resp = client
                    .password_login(upwd.qq, &upwd.password)
                    .await
                    .unwrap();
                loop {
                    match resp {
                        LoginResponse::Success(LoginSuccess {
                            ref account_info, ..
                        }) => {
                            info!("login success: {:?}", account_info);
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
                            let ticket: String = Input::with_theme(&ColorfulTheme::default())
                                .with_prompt("请输入ticket")
                                .interact()
                                .unwrap();
                            resp = client
                                .submit_ticket(&ticket)
                                .await
                                .expect("failed to submit ticket");
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
                        let qrstr = results[0].as_ref().unwrap();
                        qr2term::print_qr(qrstr).unwrap();
                        println!("扫码打印出的二维码，若无法扫描打开程序目录下qrcode.jpg");
                        if let Err(err) = auto_query_qrcode(&client, &x.sig).await {
                            panic!("登录失败，请重试 {}", err)
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
        client.token_login(token.unwrap()).await.unwrap();
    }

    after_login(&client).await;
    {
        let token = client.gen_token().await;
        let tokenstr = serde_json::to_vec(&token).unwrap();
        tokio::fs::write("session.key", tokenstr)
            .await
            .expect("无法写入session.key，请检查");
    }
    (handle, client)
}

///初始化日志
fn initlog() {
    let logconfig = ConfigBuilder::new()
        .set_time_format_rfc3339()
        .add_filter_ignore("sqlx".to_string())
        .add_filter_ignore_str("mio::poll")
        .add_filter_ignore_str("want")
        .set_thread_mode(ThreadLogMode::IDs)
        .set_thread_padding(ThreadPadding::Left(0))
        .build();
    let level;
    if let Ok(debug) = env::var("debug") && debug == "1" {
        level = LevelFilter::Debug;
    } else {
        level = LevelFilter::Info;
    }
    CombinedLogger::init(vec![TermLogger::new(
        level,
        logconfig,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();
}

struct QQandPassword {
    qq: i64,
    password: String,
}
