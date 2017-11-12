extern crate hank;
extern crate irc;
extern crate regex;
extern crate time;

use std::default::Default;
use irc::client::prelude::*;
use std::env;
use hank::errors::*;
use hank::handlers::*;

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
    let config = Config {
        nickname: Some(env::var("HANK_NICK").unwrap_or(format!("Hank"))),
        nick_password: Some(env::var("HANK_PASS")?),
        server: Some(format!("irc.rizon.net")),
        channels: Some(vec![env::var("HANK_CHANNEL")?]),
        .. Default::default()
    };
    let server = IrcServer::from_config(config)?;

    server.identify()?;

    let privmsg_handlers: Vec<fn (&HandlerContext) -> Result<()>> = vec![
        maize_handler,
        nop_handler,
        nm_handler,
        hi_handler,
        youtube_handler,
        btc_handler,
    ];

    server.for_each_incoming(|message| {
        print!("{}", message);

        match message.command {
            Command::PRIVMSG(ref target, ref msg) => {
                for handler in privmsg_handlers.iter() {
                    let context = HandlerContext::new(
                        &server,
                        &message.source_nickname().unwrap(),
                        &target,
                        &msg
                    );
                    match handler(&context) {
                        Ok(()) => (),
                        Err(error) => print_error_chain(error, false),
                    }
                }
            },
            Command::KICK(ref channel, ref target, _) => {
                let context = HandlerContext::new(
                    &server,
                    &message.source_nickname().unwrap(),
                    &target,
                    &channel
                );
                match rejoin_handler(&context) {
                    Ok(()) => (),
                    Err(error) => print_error_chain(error, false),
                }
            },
            _ => (),
        }
    })?;

    Ok(())
}
