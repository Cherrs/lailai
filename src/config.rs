use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use std::env;
use tokio::fs;
use tracing::debug;

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
pub struct DotaConfigoption {
    pub qq: i64,
    pub steam_id: u64,
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

pub static GROUP_CONF: OnceCell<HashMap<&'static str, Configoption>> = OnceCell::new();
pub static GROUP_CONF_BYQQ: OnceCell<HashMap<i64, Configoption>> = OnceCell::new();
pub static DOTA_GROUP_CONF_BYQQ: OnceCell<HashMap<i64, DotaConfigoption>> = OnceCell::new();
pub static GROUP_CONF_BYGROUPID: OnceCell<HashMap<i64, Vec<Configoption>>> = OnceCell::new();

pub async fn init() {
    if let Some(con) = get_config_file().await {
        GROUP_CONF.get_or_init(|| {
            let mut conf: HashMap<&'static str, Configoption> = HashMap::new();
            for i in &con {
                let name = i.name.clone();
                let server = i.server.clone();
                conf.insert(
                    Box::leak(format!("{name}/{server}").into_boxed_str()),
                    i.clone(),
                );
            }
            conf
        });
        GROUP_CONF_BYQQ.get_or_init(|| {
            let mut conf: HashMap<i64, Configoption> = HashMap::new();
            //let con = get_config_file().await;
            for i in &con {
                if i.qq != 0 {
                    conf.insert(i.qq, i.clone());
                }
            }
            conf
        });
        GROUP_CONF_BYGROUPID.get_or_init(|| {
            let mut conf: HashMap<i64, Vec<Configoption>> = HashMap::new();
            //let con = get_config_file().await;
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
        });
    }

    //载入dota2 group config
    if let Some(con) = get_dota_config_file().await {
        DOTA_GROUP_CONF_BYQQ.get_or_init(|| {
            let mut conf: HashMap<i64, DotaConfigoption> = HashMap::new();
            for i in &con {
                if i.qq != 0 {
                    conf.insert(i.qq, i.clone());
                }
            }
            conf
        });
    }

    match fs::read_to_string("config/config.yaml").await {
        Ok(f) => match serde_yaml::from_str(&f) {
            Ok(x) => {
                let con: BTreeMap<String, String> = x;
                for i in con {
                    env::set_var(i.0, i.1);
                }
                if env::var("interval").is_err() {
                    env::set_var("interval", "60");
                }
                if env::var("historydays").is_err() {
                    env::set_var("historydays", "1");
                }
            }
            Err(e) => debug!("配置错误，使用环境变量{}", e),
        },
        Err(e) => debug!("没有读取到配置,使用环境变量{}", e),
    }
}

async fn get_config_file() -> Option<Vec<Configoption>> {
    let mut path = "config/group_config.yaml";
    if let Ok(t) = std::fs::try_exists("group_config.dev.yaml") && t {
        path = "group_config.dev.yaml";
    }
    if let Ok(file) = fs::read_to_string(path).await {
        let con: Vec<Configoption> = serde_yaml::from_str(&file).expect("配置文件格式不正确");
        Some(con)
    } else {
        None
    }
}
// 载入dota_group_config
async fn get_dota_config_file() -> Option<Vec<DotaConfigoption>> {
    let mut path = "config/dota_group_config.yaml";
    if let Ok(t) = std::fs::try_exists("dota_group_config.dev.yaml") && t {
        path = "dota_group_config.dev.yaml";
    }
    if let Ok(file) = fs::read_to_string(path).await {
        let con: Vec<DotaConfigoption> = serde_yaml::from_str(&file).expect("配置文件格式不正确");
        Some(con)
    } else {
        None
    }
}

#[tokio::test]
async fn test() {
    init().await;
    let g = GROUP_CONF.get().unwrap();
    println!("{g:?}");
}
