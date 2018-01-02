extern crate hank;
extern crate irc;

use std::env;
use std::default::Default;
use irc::client::prelude::*;
use hank::plugin::*;
use hank::errors::*;

fn main() {
    if let Err(error) = run() {
        print_error_chain(error, true);
        ::std::process::exit(1);
    }
}

fn print_error_chain(error: Error, backtrace: bool) {
    use std::io::Write;
    let stderr = &mut ::std::io::stderr();
    let error_message = "Error writing to stderr";

    writeln!(stderr, "Error: {}", error).expect(error_message);

    for error in error.iter().skip(1) {
        writeln!(stderr, "Caused by: {}", error).expect(error_message);
    }

    if backtrace {
        if let Some(backtrace) = error.backtrace() {
            writeln!(stderr, "Backtrace: {:?}", backtrace)
                .expect(error_message);
        }
    }
}

fn run() -> Result<()> {
    let mut plugins: Vec<Box<Plugin>> = vec![
        Box::new(rejoin_plugin::RejoinPlugin),
        Box::new(youtube_plugin::YoutubePlugin),
        Box::new(nop_plugin::NopPlugin),
        Box::new(hi_plugin::HiPlugin),
        Box::new(nm_plugin::NmPlugin),
        Box::new(maize_plugin::MaizePlugin),
        Box::new(btc_plugin::BtcPlugin),
        Box::new(ltc_plugin::LtcPlugin),
        Box::new(lmgtfy_plugin::LmgtfyPlugin),
        Box::new(calc_plugin::CalcPlugin),
        Box::new(markov_chain_plugin::MarkovChainPlugin::new()?),
        Box::new(eth_plugin::EthPlugin),
    ];

    let config = Config {
        nickname: Some(env::var("HANK_NICK").unwrap_or(format!("Hank"))),
        nick_password: Some(env::var("HANK_PASS")?),
        server: Some(format!("irc.rizon.net")),
        channels: Some(vec![env::var("HANK_CHANNEL")?]),
        .. Default::default()
    };
    let server = IrcServer::from_config(config)?;

    server.identify()?;
    server.for_each_incoming(|message| {
        print!("{}", message);

        let context = PluginContext::new(&server, &message);
        for plugin in plugins.iter_mut() {
            if plugin.will_handle(message.command.clone()) {
                match plugin.handle(&context) {
                    Ok(()) => (),
                    Err(error) => print_error_chain(error, false),
                }
            }
        }
    })?;

    Ok(())
}
