use regex::Regex;
use curl::easy::Easy;
use serde_json;
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

    fn handle(&self, context: &PluginContext) -> Result<()> {
        if let Command::PRIVMSG(target, msg) = context.message.command.clone() {
            let re = Regex::new(r"^.btc$")?;

            if !re.is_match(&msg) {
                return Ok(());
            }

            let mut data = Vec::new();
            let mut easy = Easy::new();
            easy.url("https://blockchain.info/ticker")?;
            {
                let mut transfer = easy.transfer();
                transfer.write_function(|new_data| {
                    data.extend_from_slice(new_data);
                    Ok(new_data.len())
                })?;
                transfer.perform()?;
            }
            let body = String::from_utf8(data.to_vec())?;
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
