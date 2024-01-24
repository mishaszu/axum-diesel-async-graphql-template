pub mod cookie;
pub mod crypt;
pub mod ctx;
pub mod error;

pub use error::{Error, Result};

pub const AUTH_TOKEN: &str = "auth-token";
