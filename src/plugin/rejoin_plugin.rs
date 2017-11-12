use std::thread;
use std::time::Duration;
use irc::client::prelude::*;
use plugin::{Plugin, PluginContext};
use errors::*;

pub struct RejoinPlugin;
impl Plugin for RejoinPlugin {
    fn will_handle(&self, command: Command) -> bool {
        match command {
            Command::KICK(_, _, _) => true,
            _ => false,
        }
    }

    fn handle(&self, context: &PluginContext) -> Result<()> {
        let server = context.server.clone();
        if let Command::KICK(channel, _, _) = context.message.command.clone()  {
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(2000));
                server.send_join(&channel).unwrap();
            });
        }

        Ok(())
    }
}
