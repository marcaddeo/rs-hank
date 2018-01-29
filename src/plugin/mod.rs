use irc::client::prelude::*;
use errors::*;

pub mod rejoin_plugin;
pub mod youtube_plugin;
pub mod hi_plugin;
pub mod maize_plugin;
pub mod lmgtfy_plugin;
pub mod calc_plugin;
pub mod markov_chain_plugin;
pub mod crypto_plugin;

pub trait Plugin: 'static {
    fn will_handle(&self, command: Command) -> bool;
    fn handle(&mut self, context: &PluginContext) -> Result<()>;
}

pub struct PluginContext<'a> {
    server: &'a IrcServer,
    message: &'a Message,
}

impl<'a> PluginContext<'a> {
    pub fn new(
        server: &'a IrcServer,
        message: &'a Message,
    ) -> PluginContext<'a> {
        PluginContext {
            server: server,
            message: message,
        }
    }
}
