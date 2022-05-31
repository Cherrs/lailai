use super::store::Store;
use async_trait::async_trait;
use fflogsv1::parses::Parses;
use futures::executor::block_on;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::collections::HashMap;

pub struct PgStore {
    pool: Pool<Postgres>,
}
impl PgStore {
    pub async fn new(str: &str) -> Result<PgStore, Box<dyn std::error::Error>> {
        let pool = PgPoolOptions::new().max_connections(5).connect(str).await?;
        Ok(PgStore { pool })
    }
}

impl Drop for PgStore {
    fn drop(&mut self) {
        block_on(self.pool.close());
    }
}

#[async_trait]
impl Store for PgStore {
    async fn add_cache(&self, parse: &Parses) {
        let _ = sqlx::query(
            "INSERT INTO public.cache(
                code, datetime)
                VALUES ($1, $2);",
        )
        .bind(format!("{}#fight={}", &parse.report_id, &parse.fight_id))
        .bind(&parse.start_time)
        .execute(&self.pool)
        .await
        .unwrap();
    }
    async fn init(&self, datas: &HashMap<i64, Vec<Parses>>) {
        let mut p = Vec::new();
        let mut datas = datas.iter().map(|(k, v)| {
            (
                format!(
                    "{}#fight={}",
                    v.first().unwrap().report_id,
                    v.first().unwrap().fight_id
                ),
                k,
            )
        });
        for x in datas.by_ref() {
            let code = x.0;
            if p.contains(&code) {
                continue;
            }
            let datetime = x.1;
            p.push(code.clone());
            sqlx::query(
                "INSERT INTO public.cache(
                code, datetime)
                VALUES ($1, $2);",
            )
            .bind(code)
            .bind(datetime)
            .fetch_all(&self.pool)
            .await
            .unwrap();
        }
    }
    async fn query_by_start_time(&self, start_time: i64) -> Vec<i64> {
        let row: Vec<(i64,)> = sqlx::query_as("select datetime from cache where datetime > $1")
            .bind(start_time)
            .fetch_all(&self.pool)
            .await
            .unwrap();
        row.iter().map(|x| x.0).collect()
    }
    async fn is_empty(&self) -> bool {
        let count: (i64,) = sqlx::query_as("select count(0) from cache")
            .fetch_one(&self.pool)
            .await
            .unwrap();
        count.0 == 0
    }
}
