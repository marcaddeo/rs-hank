use std::env;
use serde_json;
use serde_json::Value;
use time;
use regex::Regex;
use curl::easy::Easy;
use irc::client::prelude::*;
use plugin::{Plugin, PluginContext};
use duration::parse_duration;
use errors::*;

pub struct YoutubePlugin;
impl Plugin for YoutubePlugin {
    fn will_handle(&self, command: Command) -> bool {
        match command {
            Command::PRIVMSG(_, _) => true,
            _ => false,
        }
    }

    fn handle(&self, context: &PluginContext) -> Result<()> {
        if let Command::PRIVMSG(target, msg) = context.message.command.clone() {
            let re = Regex::new(
                r"^.*((youtu.be/)|(v/)|(/u/\w/)|(embed/)|(watch\?))\??v?=?(?P<video_id>[^#\&\?\s]*).*"
            )?;

            let captures = match re.captures(&msg) {
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

            let duration = parse_duration(&video_duration)?;
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

            context.server.send_privmsg(&target, &message)?;
        }

        Ok(())
    }
}
