use regex::Regex;
use irc::client::prelude::*;
use plugin::{Plugin, PluginContext};
use errors::*;

pub struct MaizePlugin;
impl Plugin for MaizePlugin {
    fn handle(&mut self, context: &PluginContext) -> Result<()> {
        if let Command::PRIVMSG(target, msg) = context.message.command.clone() {
            let re = Regex::new(r"^[o]+[h]+$")?;

            if re.is_match(&msg) {
                context.server.send_privmsg(&target, "maize")?;
            }
        }

        Ok(())
    }
}
