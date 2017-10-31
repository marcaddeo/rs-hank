extern crate irc;
extern crate regex;
extern crate rand;
extern crate rafy;
extern crate time;

use std::default::Default;
use irc::client::prelude::*;
use regex::Regex;
use rand::Rng;
use rafy::Rafy;
use time::Duration;
use std::env;
use std::time::Duration as StdDuration;
use std::thread;

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

fn maize_handler(context: &HandlerContext) {
    let re = Regex::new(r"^[o]+[h]+$").unwrap();

    if re.is_match(context.message) {
        context.server.send_privmsg(context.target, "maize").unwrap();
    }
}

fn hi_handler(context: &HandlerContext) {
    let re = Regex::new(
        &format!(r"(?i)h(i?) {}", context.server.config().nickname())
    ).unwrap();
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
        let greeting = rand::thread_rng().choose(&greetings).unwrap();
        context.server.send_privmsg(
            context.target,
            &format!("{} {}", greeting, context.sender)
        ).unwrap();
    }
}

fn youtube_handler(context: &HandlerContext) {
    let re = Regex::new(
        r"^.*((youtu.be/)|(v/)|(/u/\w/)|(embed/)|(watch\?))\??v?=?(?P<video_id>[^#\&\?\s]*).*"
    ).unwrap();

    let captures = match re.captures(context.message) {
        Some(captures) => captures,
        None => return (), // bail, there was no youtube video found in the message
    };

    let video = match Rafy::new(&captures["video_id"]) {
        Ok(video) => video,
        Err(_) => return (), // bail, failed to get video information
    };

    let duration = Duration::seconds(i64::from(video.length));
    let tm = time::empty_tm() + duration;

    let message = format!(
        "{title} [{duration}] {views} views (+{likes}|-{dislikes}) {permalink}",
        title = format!("\x02{}\x0F", video.title),
        duration = tm.strftime("%X").unwrap(),
        views = video.viewcount,
        likes = format!("\x0303{}\x0F", video.likes),
        dislikes = format!("\x0304{}\x0F", video.dislikes),
        permalink = format!("\x02https://youtu.be/{}\x0F", video.videoid),
    );

    context.server.send_privmsg(context.target, &message).unwrap();
}

fn main() {
    let config = Config {
        nickname: Some(format!("Hank")),
        nick_password: Some(env::var("HANK_PASS").unwrap()),
        server: Some(format!("irc.rizon.net")),
        channels: Some(vec![env::var("HANK_CHANNEL").unwrap()]),
        .. Default::default()
    };
    let server = IrcServer::from_config(config).unwrap();

    server.identify().unwrap();

    let privmsg_handlers: Vec<fn (&HandlerContext)> = vec![
        maize_handler,
        hi_handler,
        youtube_handler,
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
                    handler(&context);
                }
            },
            Command::KICK(ref channel, _, _) => {
                thread::sleep(StdDuration::from_millis(2000));
                server.send_join(channel).unwrap();
            },
            _ => (),
        }
    }).unwrap();
}
