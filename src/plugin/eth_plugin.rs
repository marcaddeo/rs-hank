use regex::Regex;
use serde_json;
use reqwest;
use std::io::Read;
use serde_json::Value;
use irc::client::prelude::*;
use plugin::{Plugin, PluginContext};
use errors::*;

pub struct EthPlugin;
impl Plugin for EthPlugin {
    fn will_handle(&self, command: Command) -> bool {
        match command {
            Command::PRIVMSG(_, _) => true,
            _ => false,
        }
    }

    fn handle(&mut self, context: &PluginContext) -> Result<()> {
        if let Command::PRIVMSG(target, msg) = context.message.command.clone() {
            let re = Regex::new(r"^\.eth$")?;

            if !re.is_match(&msg) {
                return Ok(());
            }

            let mut res = reqwest::get("https://api.gdax.com/products/ETH-USD/ticker")?;
            let mut body = String::new();
            res.read_to_string(&mut body)?;
            let json: Value = serde_json::from_str(&body)?;

            let message = format!(
                "${:.2}",
                json["price"].as_str().ok_or("")?.parse::<f64>()?,
            );

            context.server.send_privmsg(&target, &message)?;
        }

        Ok(())
    }
}
