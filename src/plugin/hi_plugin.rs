use regex::Regex;
use rand;
use rand::Rng;
use irc::client::prelude::*;
use plugin::{Plugin, PluginContext};
use errors::*;

pub struct HiPlugin;
impl Plugin for HiPlugin {
    fn will_handle(&self, command: Command) -> bool {
        match command {
            Command::PRIVMSG(_, _) => true,
            _ => false,
        }
    }

    fn handle(&self, context: &PluginContext) -> Result<()> {
        if let Command::PRIVMSG(target, msg) = context.message.command.clone() {
            let re = Regex::new(
                &format!(r"(?i)h(i?) {}", context.server.config().nickname())
            )?;
            let greetings = vec![
                "hi",
                "h",
                "bonjour",
                "sup",
                "ni hao",
                "fuck off",
                "piss off",
                "get fucked",
            ];

            if re.is_match(&msg) {
                let greeting = rand::thread_rng().choose(&greetings).ok_or("")?;
                context.server.send_privmsg(
                    &target,
                    &format!("{} {}", greeting, context.message.source_nickname().ok_or("")?)
                )?;
            }
        }

        Ok(())
    }
}
