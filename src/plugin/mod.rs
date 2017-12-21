use irc::client::prelude::*;
use errors::*;

pub mod rejoin_plugin;
pub mod youtube_plugin;
pub mod nop_plugin;
pub mod hi_plugin;
pub mod nm_plugin;
pub mod maize_plugin;
pub mod btc_plugin;
pub mod ltc_plugin;
pub mod lmgtfy_plugin;

pub trait Plugin: 'static {
    fn will_handle(&self, command: Command) -> bool;
    fn handle(&self, context: &PluginContext) -> Result<()>;
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
