#![feature(plugin, custom_derive,libc,core)]
#![plugin(serde_macros)]
#![allow(deprecated)]
extern crate num;
extern crate serde;
extern crate hyper;
extern crate rand;
extern crate time;
extern crate libc;
extern crate openssl;
#[macro_use] extern crate log;
extern crate env_logger;
mod error;
mod action;
mod protocol;
mod game;
mod dict;
mod httpconnector;

pub use game::*;
pub use error::*;
pub use dict::*;
pub use httpconnector::*;
