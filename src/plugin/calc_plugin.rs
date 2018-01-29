use regex::Regex;
use irc::client::prelude::*;
use plugin::{Plugin, PluginContext};
use calc;
use calc::eval as calculate;
use errors::*;

pub struct CalcPlugin;
impl Plugin for CalcPlugin {
    fn handle(&mut self, context: &PluginContext) -> Result<()> {
        if let Command::PRIVMSG(target, msg) = context.message.command.clone() {
            let re = Regex::new(r"^\.(cc|calc) (?P<expression>.*)")?;

            let captures = match re.captures(&msg) {
                Some(captures) => captures,
                None => return Ok(()), // bail, there was no search term
            };

            let result = calculate(&captures["expression"])
                .unwrap_or(calc::Value::Dec(0));

            context.server.send_privmsg(&target, &format!("{}", &result))?;
        }

        Ok(())
    }
}
