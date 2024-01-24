use derive_more::Display;
use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, Display, Clone)]
pub enum Error {
    #[display(fmt = "FailToCreatePool: {}", _0)]
    FailToCreatePool(String),
}
