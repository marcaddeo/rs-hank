use regex::Regex;
use serde_json;
use reqwest;
use std::io::Read;
use serde_json::Value;
use irc::client::prelude::*;
use plugin::{Plugin, PluginContext};
use errors::*;

pub struct BtcPlugin;
impl Plugin for BtcPlugin {
    fn will_handle(&self, command: Command) -> bool {
        match command {
            Command::PRIVMSG(_, _) => true,
            _ => false,
        }
    }

    fn handle(&mut self, context: &PluginContext) -> Result<()> {
        if let Command::PRIVMSG(target, msg) = context.message.command.clone() {
            let re = Regex::new(r"^\.btc$")?;

            if !re.is_match(&msg) {
                return Ok(());
            }

            let mut res = reqwest::get("https://blockchain.info/ticker")?;
            let mut body = String::new();
            res.read_to_string(&mut body)?;
            let json: Value = serde_json::from_str(&body)?;

            let message = format!(
                "{}{:.3}",
                json["USD"]["symbol"].as_str().ok_or("")?,
                json["USD"]["last"],
            );

            context.server.send_privmsg(&target, &message)?;
        }

        Ok(())
    }
}
