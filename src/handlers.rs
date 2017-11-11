use regex::Regex;
use time;
use std::env;
use std::time::Duration;
use std::thread;
use rand;
use rand::Rng;
use curl::easy::Easy;
use serde_json;
use serde_json::Value;
use irc::client::prelude::*;
use duration::parse_duration;
use errors::*;

pub struct HandlerContext<'a> {
    server: &'a IrcServer,
    sender: &'a str,
    target: &'a String,
    message: &'a String,
}

impl<'a> HandlerContext<'a> {
    pub fn new(
        server: &'a IrcServer,
        sender: &'a str,
        target: &'a String,
        message: &'a String,
    ) -> HandlerContext<'a> {
        HandlerContext {
            server: server,
            sender: sender,
            target: target,
            message: message,
        }
    }
}

pub fn nop_handler(context: &HandlerContext) -> Result<()> {
    let re = Regex::new(r"^.nop$")?;

    if re.is_match(context.message) {
        context.server.send_privmsg(context.target, "nop pls")?;
    }

    Ok(())
}

pub fn nm_handler(context: &HandlerContext) -> Result<()> {
  let re = Regex::new(r"^nmu$")?;

  if re.is_match(context.message) {
    context.server.send_privmsg(context.target, "nm")?;
  }

  Ok(())
}

pub fn maize_handler(context: &HandlerContext) -> Result<()> {
    let re = Regex::new(r"^[o]+[h]+$")?;

    if re.is_match(context.message) {
        context.server.send_privmsg(context.target, "maize")?;
    }

    Ok(())
}

pub fn hi_handler(context: &HandlerContext) -> Result<()> {
    let re = Regex::new(
        &format!(r"(?i)h(i?) {}", context.server.config().nickname())
    )?;
    let greetings = vec![
        "hi",
        "h",
        "bonjour",
        "sup",
        "ni hao",
        "fuck off",
        "piss off",
        "get fucked",
    ];

    if re.is_match(context.message) {
        let greeting = rand::thread_rng().choose(&greetings).ok_or("")?;
        context.server.send_privmsg(
            context.target,
            &format!("{} {}", greeting, context.sender)
        )?;
    }

    Ok(())
}

pub fn youtube_handler(context: &HandlerContext) -> Result<()> {
    let re = Regex::new(
        r"^.*((youtu.be/)|(v/)|(/u/\w/)|(embed/)|(watch\?))\??v?=?(?P<video_id>[^#\&\?\s]*).*"
    )?;

    let captures = match re.captures(context.message) {
        Some(captures) => captures,
        None => return Ok(()), // bail, there was no youtube video found in the message
    };

    let mut data = Vec::new();
    let mut easy = Easy::new();
    let url = format!(
        "https://www.googleapis.com/youtube/v3/videos?part=contentDetails,snippet,statistics&id={video_id}&key={api_key}",
        video_id = &captures["video_id"],
        api_key = env::var("HANK_YOUTUBE_API_KEY")?,
    );
    easy.url(&url)?;
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        })?;
        transfer.perform()?;
    }
    let body = String::from_utf8(data.to_vec())?;
    let json: Value = serde_json::from_str(&body)?;
    let video = &json["items"][0];
    let video_title: String = serde_json::from_value(video["snippet"]["title"].clone())?;
    let video_duration: String = serde_json::from_value(video["contentDetails"]["duration"].clone())?;
    let video_definition: String = serde_json::from_value(video["contentDetails"]["definition"].clone())?;
    let video_views: String = serde_json::from_value(video["statistics"]["viewCount"].clone())?;
    let video_likes: String = serde_json::from_value(video["statistics"]["likeCount"].clone())?;
    let video_dislikes: String = serde_json::from_value(video["statistics"]["dislikeCount"].clone())?;

    let duration = parse_duration(&video_duration).unwrap();
    let tm = time::empty_tm() + duration;

    let message = format!(
        "{title} [{duration}] {definition} {views} views (+{likes}|-{dislikes}) {permalink}",
        title = format!("\x02{}\x0F", video_title),
        duration = tm.strftime("%X")?,
        definition = format!("\x02\x0304{}\x0F", video_definition.to_uppercase()),
        views = video_views,
        likes = format!("\x0303{}\x0F", video_likes),
        dislikes = format!("\x0304{}\x0F", video_dislikes),
        permalink = format!("\x02https://youtu.be/{}\x0F", &captures["video_id"]),
    );

    context.server.send_privmsg(context.target, &message)?;

    Ok(())
}

pub fn rejoin_handler(context: &HandlerContext) -> Result<()> {
    let server = context.server.clone();
    let channel = context.message.clone();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(2000));
        server.send_join(&channel).unwrap();
    });

    Ok(())
}

pub fn btc_handler(context: &HandlerContext) -> Result<()> {
    let re = Regex::new(r"^.btc$")?;

    if !re.is_match(context.message) {
        return Ok(());
    }

    let mut data = Vec::new();
    let mut easy = Easy::new();
    easy.url("https://blockchain.info/ticker")?;
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        })?;
        transfer.perform()?;
    }
    let body = String::from_utf8(data.to_vec())?;
    let json: Value = serde_json::from_str(&body)?;

    let message = format!(
        "{}{:.3}",
        json["USD"]["symbol"].as_str().ok_or("")?,
        json["USD"]["last"],
    );

    context.server.send_privmsg(context.target, &message)?;

    Ok(())
}
