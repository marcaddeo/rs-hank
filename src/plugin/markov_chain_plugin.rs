use std::env;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use regex::Regex;
use markov::Chain;
use rand;
use rand::Rng;
use irc::client::prelude::*;
use plugin::{Plugin, PluginContext};
use errors::*;

pub struct MarkovChainPlugin {
    chain: Chain<String>,
}

impl MarkovChainPlugin {
    pub fn new() -> Result<MarkovChainPlugin> {
        let path = get_log_file()?;
        let file = File::open(path)?;
        let mut chain: Chain<String> = Chain::new();

        for line in BufReader::new(file).lines() {
            let line = match line {
                Ok(line) => line,
                Err(_) => continue,
            };
            chain.feed_str(line.as_str());
        }

        Ok(MarkovChainPlugin {
            chain: chain,
        })
    }

    fn process_message(&mut self, message: &str) -> Result<()> {
        let message = message.to_lowercase();
        let path = get_log_file()?;
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(path)?;

        writeln!(file, "{}", message)?;
        self.chain.feed_str(&message);

        Ok(())
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
            self.process_message(&msg)?;

            let re = Regex::new(r"(?i)^Hank.? (?P<message>.*)$")?;
            let captures = match re.captures(&msg) {
                Some(captures) => captures,
                None => return Ok(()), // bail, not a message to Hank
            };
            let message = &captures["message"].to_lowercase();
            let words: Vec<&str> = message.split(' ').collect();
            let word = match rand::thread_rng().choose(&words) {
                Some(word) => word,
                _ => return Ok(()),
            };
            let mut attempts = 0;
            while attempts < 100 {
                let response = self.chain.generate_str_from_token(word);
                let words: Vec<&str> = response
                    .split(' ')
                    .filter(|word| !word.is_empty())
                    .collect();

                if words.len() >= 3 {
                    context.server.send_privmsg(&target, &response)?;
                    break;
                }

                attempts += 1;
            }
        }

        Ok(())
    }
}

fn get_log_file() -> Result<String> {
    Ok(env::var("HANK_MARKOV_CHAIN_FILE")?)
}
