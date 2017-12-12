use regex::Regex;
use irc::client::prelude::*;
use plugin::{Plugin, PluginContext};
use url::Url;
use errors::*;

pub struct LmgtfyPlugin;
impl Plugin for LmgtfyPlugin {
    fn will_handle(&self, command: Command) -> bool {
        match command {
            Command::PRIVMSG(_, _) => true,
            _ => false,
        }
    }

    fn handle(&self, context: &PluginContext) -> Result<()> {
        if let Command::PRIVMSG(target, msg) = context.message.command.clone() {
            let re = Regex::new(r"^\.g (?P<search_term>.*)")?;

            let captures = match re.captures(&msg) {
                Some(captures) => captures,
                None => return Ok(()), // bail, there was no search term
            };

            let url = Url::parse(&format!(
                "http://lmgtfy.com/?q={search_term}",
                search_term = &captures["search_term"],
            ));

            context.server.send_privmsg(&target, &format!(
                "{bold}lmgtfy:{reset} {url}",
                bold = "\x02",
                reset = "\x0F",
                url = url?,
            ))?;
        }

        Ok(())
    }
}
