pub mod channel;
pub mod message;
pub mod roles;
pub mod user;

use whirlpool::{Whirlpool, Digest};
use std::borrow::Cow;
use std::string::FromUtf8Error;

pub fn hash_password(password: String) -> String {
    format!("{:x}", Whirlpool::new().chain(password.as_bytes()).finalize())
}