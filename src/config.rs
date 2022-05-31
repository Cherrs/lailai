use log::debug;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use std::env;
use tokio::fs;
use tokio::sync::OnceCell;

#[derive(Deserialize, Clone, Debug)]
pub struct Configoption {
    pub qq: i64,
    pub name: String,
    pub server: String,
    #[serde(default = "default_region")]
    pub region: String,
    pub group: Vec<Group>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Group {
    pub id: i64,
    pub xdspec: Option<String>,
}

fn default_region() -> String {
    "CN".to_string()
}

pub static GROUP_CONF: OnceCell<HashMap<&'static str, Configoption>> = OnceCell::const_new();
pub static GROUP_CONF_BYQQ: OnceCell<HashMap<i64, Configoption>> = OnceCell::const_new();
pub static GROUP_CONF_BYGROUPID: OnceCell<HashMap<i64, Vec<Configoption>>> = OnceCell::const_new();

pub async fn init() {
    GROUP_CONF
        .get_or_init(|| async {
            let mut conf: HashMap<&'static str, Configoption> = HashMap::new();
            let con = get_config_file().await;
            for i in con {
                let name = i.name.clone();
                let server = i.server.clone();
                conf.insert(Box::leak(format!("{name}/{server}").into_boxed_str()), i);
            }
            conf
        })
        .await;
    GROUP_CONF_BYQQ
        .get_or_init(|| async {
            let mut conf: HashMap<i64, Configoption> = HashMap::new();
            let con = get_config_file().await;
            for i in con {
                if i.qq != 0 {
                    conf.insert(i.qq, i);
                }
            }
            conf
        })
        .await;
    GROUP_CONF_BYGROUPID
        .get_or_init(|| async {
            let mut conf: HashMap<i64, Vec<Configoption>> = HashMap::new();
            let con = get_config_file().await;
            for i in con {
                for ii in &i.group {
                    match conf.get_mut(&ii.id) {
                        Some(users) => {
                            let x = i.clone();
                            users.push(x);
                        }
                        None => {
                            conf.insert(ii.id, vec![i.clone()]);
                        }
                    }
                }
            }
            conf
        })
        .await;
    match fs::read_to_string("config.yaml").await {
        Ok(f) => match serde_yaml::from_str(&f) {
            Ok(x) => {
                let con: BTreeMap<String, String> = x;
                for i in con {
                    env::set_var(i.0, i.1);
                }
                if env::var("interval").is_err() {
                    env::set_var("interval", "60");
                }
            }
            Err(e) => debug!("配置错误，使用环境变量{}", e),
        },
        Err(e) => debug!("没有读取到配置,使用环境变量{}", e),
    }
}

async fn get_config_file() -> Vec<Configoption> {
    let mut path = "group_config.yaml";
    if std::fs::try_exists("group_config.dev.yaml").unwrap() {
        path = "group_config.dev.yaml";
    }
    let file = fs::read_to_string(path).await.expect("读取配置失败");
    let con: Vec<Configoption> = serde_yaml::from_str(&file).expect("配置文件格式不正确");
    con
}
