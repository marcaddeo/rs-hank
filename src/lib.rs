#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;

extern crate regex;
extern crate rand;
extern crate time;
extern crate reqwest;
extern crate serde_json;
extern crate irc;
extern crate url;
extern crate calc;
extern crate markov;

pub mod errors;
pub mod duration;
pub mod plugin;
