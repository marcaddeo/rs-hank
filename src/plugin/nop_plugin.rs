use regex::Regex;
use irc::client::prelude::*;
use plugin::{Plugin, PluginContext};
use errors::*;

pub struct NopPlugin;
impl Plugin for NopPlugin {
    fn will_handle(&self, command: Command) -> bool {
        match command {
            Command::PRIVMSG(_, _) => true,
            _ => false,
        }
    }

    fn handle(&self, context: &PluginContext) -> Result<()> {
        if let Command::PRIVMSG(target, msg) = context.message.command.clone() {
            let re = Regex::new(r"^\.nop$")?;

            if re.is_match(&msg) {
                context.server.send_privmsg(&target, "nop pls")?;
            }
        }

        Ok(())
    }
}
