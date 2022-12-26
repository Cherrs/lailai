use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpenDotaErr {
    #[error("请求opendota api错误")]
    ReqwestErr(#[from] reqwest::Error),
}
