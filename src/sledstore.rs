use async_trait::async_trait;
use sled::Db;

use crate::store::Store;

pub struct SledStore {
    db: Db,
}

impl SledStore {
    pub fn new(path: &str) -> SledStore {
        SledStore {
            db: sled::open(path).expect("本地存储无法使用"),
        }
    }
}

#[async_trait]
impl Store for SledStore {
    async fn query_by_start_time(&self, start_time: i64) -> Vec<i64> {
        let result = self.db.range(start_time.to_be_bytes()..);
        let rs: Vec<i64> = result
            .map(|x| {
                let d: [u8; 8] = (x.unwrap().0)[0..8].try_into().expect("err");
                i64::from_be_bytes(d)
            })
            .collect();
        rs
    }

    async fn init(&self, datas: &std::collections::HashMap<i64, Vec<fflogsv1::parses::Parses>>) {
        for i in datas {
            let data = bincode::serialize(i.1.first().unwrap()).unwrap();
            self.db.insert(i.0.to_be_bytes(), data).unwrap();
        }
        self.db.flush_async().await.unwrap();
    }

    async fn is_empty(&self) -> bool {
        self.db.is_empty()
    }

    async fn add_cache(&self, parse: &fflogsv1::parses::Parses) {
        let data = bincode::serialize(parse).unwrap();
        self.db
            .insert(parse.start_time.to_be_bytes(), data)
            .unwrap();
    }
}
