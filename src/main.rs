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

    if let Some(captures) = re.captures(context.message) {
        let video = match Rafy::new(&captures["video_id"]) {
            Ok(video) => video,
            Err(_) => return (),
        };

        let duration = Duration::seconds(i64::from(video.length));
        let tm = time::empty_tm() + duration;

        let message = format!(
            "\x02{title}\x0F [{duration}] {views} views (+\x0303{likes}\x0F|-\x0304{dislikes}\x0F) \x02{permalink}\x0F",
            title = video.title,
            duration = tm.strftime("%X").unwrap(),
            views = video.viewcount,
            likes = video.likes,
            dislikes = video.dislikes,
            permalink = format!("https://youtu.be/{id}", id = video.videoid),
        );

        context.server.send_privmsg(context.target, &message).unwrap();
    }
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
            _ => (),
        }
    }).unwrap();
}
