use async_trait::async_trait;
use fflogsv1::parses::Parses;
use std::collections::HashMap;

#[async_trait]
pub trait Store {
    async fn query_by_start_time(&self, start_time: i64) -> Vec<i64>;
    async fn init(&self, datas: &HashMap<i64, Vec<Parses>>);
    async fn is_empty(&self) -> bool;
    async fn add_cache(&self, parse: &Parses);
}
