#[macro_use]
extern crate error_chain;

extern crate regex;
extern crate rand;
extern crate time;
extern crate curl;
extern crate serde_json;
extern crate irc;

pub mod errors;
pub mod handlers;
pub mod duration;
