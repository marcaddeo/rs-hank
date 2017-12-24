use std::io::{BufReader, BufRead};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use regex::Regex;
use markov::Chain;
use irc::client::prelude::*;
use plugin::{Plugin, PluginContext};
use errors::*;

pub struct MarkovChainPlugin {
    chain: Chain<String>,
}

impl MarkovChainPlugin {
    pub fn new() -> Result<MarkovChainPlugin> {
        let file = File::open("~/hank_chains.txt")?;
        let mut chain: Chain<String> = Chain::new();

        for line in BufReader::new(file).lines() {
            chain.feed_str(line?.as_str());
        }

        Ok(MarkovChainPlugin {
            chain: chain,
        })
    }
}

impl Plugin for MarkovChainPlugin {
    fn will_handle(&self, command: Command) -> bool {
        match command {
            Command::PRIVMSG(_, _) => true,
            _ => false,
        }
    }

    fn handle(&mut self, context: &PluginContext) -> Result<()> {
        if let Command::PRIVMSG(target, msg) = context.message.command.clone() {
            let re = Regex::new(r"(?i)^Hank (?P<message>.*)$")?;

            let captures = match re.captures(&msg) {
                Some(captures) => captures,
                None => return Ok(()), // bail, not a message to Hank
            };

            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("~/hank_chains.txt")?;

            writeln!(file, "{}", &captures["message"])?;
            self.chain.feed_str(&captures["message"]);

            context.server.send_privmsg(&target, &self.chain.generate_str())?;
        }

        Ok(())
    }
}
