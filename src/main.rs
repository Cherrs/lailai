#![feature(let_chains)]
mod config;
mod message_handler;
//mod messagebuild;
mod sendreport;

use fflogsv1::FF14;
use log::error;
use qrcode::QrCode;
use ricq::{
    client::Token,
    device::Device,
    ext::common::after_login,
    version::{get_version, Protocol},
    Client,
};
use simplelog::*;
use std::{env, path::Path, sync::Arc, time::Duration};
use tokio::{net::TcpStream, task::JoinHandle};

use crate::message_handler::MyHandler;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //初始化配置
    config::init().await;
    initlog();
    let (_handle, client) = initbot().await;
    loop {
        //获取logs数据，检测更新发送到群
        let cli = client.clone();
        tokio::spawn(async move {
            match sendreport::trysendmessageorinit(&cli).await {
                Ok(_) => {}
                Err(e) => error!("{:?}", e),
            }
        });
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
    //handle.await.unwrap();
    //Ok(())
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
            tokio::fs::write("device.json", serde_json::to_string(&d).unwrap())
                .await
                .expect("failed to write device info to file");
            d
        }
    };
    let token: Option<Token> = match Path::new("session.key").exists() {
        true => {
            serde_json::from_str(&tokio::fs::read_to_string("session.key").await.unwrap()).unwrap()
        }
        false => None,
    };
    let myh = MyHandler {
        ff14client: FF14::new(env::var("logskey").unwrap().as_str()),
    };
    let client = Arc::new(Client::new(device, get_version(Protocol::IPad), myh));
    let stream = TcpStream::connect(client.get_address())
        .await
        .expect("failed to connect");
    let c = client.clone();
    let handle = tokio::spawn(async move { c.start(stream).await });
    tokio::task::yield_now().await; // 等一下，确保连上了
    if token.is_none() {
        let resp = client.fetch_qrcode().await.expect("failed to fetch qrcode");
        use ricq::ext::login::auto_query_qrcode;
        match resp {
            ricq::QRCodeState::ImageFetch(x) => {
                let img = image::load_from_memory(&x.image_data).unwrap();
                let decoder = bardecoder::default_decoder();
                let results = decoder.decode(&img);
                let qrstr = results[0].as_ref().unwrap();
                let code = QrCode::new(qrstr).unwrap();
                let image = code
                    .render::<char>()
                    .quiet_zone(false)
                    .module_dimensions(2, 1)
                    .build();
                println!("{}", image);
                if let Err(err) = auto_query_qrcode(&client, &x.sig).await {
                    panic!("登录失败 {}", err)
                };
            }
            _ => {
                panic!("resp error")
            }
        }
    } else {
        client.token_login(token.unwrap()).await.unwrap();
    }

    after_login(&client).await;
    {
        let token = client.gen_token().await;
        let tokenstr = serde_json::to_vec(&token).unwrap();
        tokio::fs::write("session.key", tokenstr).await.unwrap();
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
