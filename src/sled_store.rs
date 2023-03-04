use async_trait::async_trait;
use sled::Db;

use crate::store::Store;

pub struct SledStore {
    db: Db,
}

impl SledStore {
    pub fn create(path: &str) -> Box<dyn Store> {
        Box::new(SledStore {
            db: sled::open(path).expect("本地存储无法使用"),
        })
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

#[test]
fn delete() {
    let db = sled::open("db/test").expect("本地存储无法使用");
    db.remove(1653815603808_i64.to_be_bytes()).unwrap();
    db.flush().unwrap();
}

#[test]
fn insert() {
    let db = sled::open("db/test").expect("本地存储无法使用");
    for i in 1_i64..100_i64 {
        db.insert(i.to_be_bytes(), i.to_be_bytes().to_vec())
            .unwrap();
    }
    db.flush().unwrap();
}
#[test]
fn insert_new() {
    let db = sled::open("db/openai").unwrap();
    let db = db.open_tree(15i64.to_be_bytes()).unwrap();
    for i in 0..50 {
        db.insert(
            format!("真的吗_{}", i).as_str(),
            format!("真的_{}", i).as_str(),
        )
        .unwrap();
    }
}

#[test]
fn sled_iter() {
    let db = sled::open("db/openai").unwrap();
    let db = db.open_tree(16i64.to_be_bytes()).unwrap();
    for ele in db.iter() {
        let e = ele.unwrap();
        println!(
            "k:{} v:{}",
            String::from_utf8(e.0.to_vec()).unwrap(),
            String::from_utf8(e.1.to_vec()).unwrap()
        );
    }
}

#[tokio::test]
async fn query() {
    let db = SledStore::create("db/test");
    let mm = db.query_by_start_time(50).await;
    println!("{mm:?}");
}
