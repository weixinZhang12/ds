use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStructErr {
    #[error("无效的索引")]
    InvalidIndex,
}