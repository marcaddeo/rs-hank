use regex::Regex;
use serde_json;
use reqwest;
use std::collections::HashMap;
use std::io::Read;
use serde_json::Value;
use irc::client::prelude::*;
use plugin::{Plugin, PluginContext};
use separator::FixedPlaceSeparatable;
use errors::*;

#[derive(Debug)]
pub enum PriceChange {
    Increase,
    Decrease,
    NoChange,
    NoHistory,
}

#[derive(Debug)]
pub struct CryptoPlugin {
    watchlist: Vec<String>,
    coinlist: HashMap<String, String>,
    prices: HashMap<String, f64>,
}
impl CryptoPlugin {
    pub fn new(watchlist: Vec<String>) -> Result<CryptoPlugin> {
        let mut res = reqwest::get(
            "https://min-api.cryptocompare.com/data/all/coinlist"
        )?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;
        let json: Value = serde_json::from_str(&body)?;

        let mut coinlist: HashMap<String, String> = HashMap::new();
        if let Some(data) = json["Data"].as_object() {
            for (symbol, coin) in data {
                let full_name = coin["FullName"].as_str()
                    .ok_or("Error converting FullName to str")?
                    .to_string();

                coinlist.insert(
                    symbol.to_string().to_uppercase(),
                    full_name,
                );
            }
        }

        Ok(CryptoPlugin {
            watchlist,
            coinlist,
            prices: HashMap::new(),
        })
    }

    fn get_price(&mut self, symbol: &str) -> Result<(f64, PriceChange)> {
        let symbol = symbol.to_string().to_uppercase();

        if !self.coinlist.contains_key(&symbol) {
            bail!("Invalid coin symbol");
        }

        let mut res = reqwest::get(&format!(
            "https://min-api.cryptocompare.com/data/price?fsym={}&tsyms=USD",
            symbol
        ))?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;
        let json: Value = serde_json::from_str(&body)?;
        let price = json["USD"].as_f64()
            .ok_or("Could not convert value to f64")?;

        let change = match self.prices.clone().get(&symbol) {
            Some(last_price) => {
                self.prices.insert(symbol, price);
                if price > *last_price {
                    PriceChange::Increase
                } else if price < *last_price {
                    PriceChange::Decrease
                } else {
                    PriceChange::NoChange
                }
            },
            None => {
                self.prices.insert(symbol, price);
                PriceChange::NoHistory
            }
        };

        Ok((price, change))
    }
}
impl Plugin for CryptoPlugin {
    fn will_handle(&self, command: Command) -> bool {
        match command {
            Command::PRIVMSG(_, _) => true,
            _ => false,
        }
    }

    fn handle(&mut self, context: &PluginContext) -> Result<()> {
        if let Command::PRIVMSG(target, msg) = context.message.command.clone() {
            let re = Regex::new(&format!(
                r#"(?i)^\.(?P<command>crypto|{})( (?P<symbol>.*))?"#,
                self.watchlist.join("|"),
            ))?;

            let captures = match re.captures(&msg) {
                Some(captures) => captures,
                None => return Ok(()), // bail, there was no command
            };

            let symbol = match &captures["command"] {
                "crypto" => match captures.name("symbol") {
                    Some(symbol) => symbol.as_str(),
                    None => return Ok(()), // bail, there was no symbol
                },
                _ => &captures["command"],
            }.to_uppercase();

            let coinlist = self.coinlist.clone();
            let full_name = coinlist.get(&symbol)
                .ok_or("Error retrieving coin name")?;
            let (price, change) = self.get_price(&symbol)?;
            let change_symbol = match change {
                PriceChange::Increase => "\x0303▲\x0F",
                PriceChange::Decrease => "\x0304▼\x0F",
                PriceChange::NoChange => "\x0314—\x0F",
                _ => "",
            };
            context.server.send_privmsg(&target, &format!(
                "{}: ${} {}",
                full_name,
                price.separated_string_with_fixed_place(2),
                change_symbol,
            ))?;
        }

        Ok(())
    }
}
