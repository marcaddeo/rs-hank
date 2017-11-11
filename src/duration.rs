use time::Duration;
use regex::Regex;
use errors::*;

pub fn parse_duration(period: &str) -> Result<Duration> {
    let re = Regex::new(r"(?x)
        ^(-|\+)?P
        (?:(?P<years>[-+]?[0-9,.]*)Y)?
        (?:(?P<months>[-+]?[0-9,.]*)M)?
        (?:(?P<weeks>[-+]?[0-9,.]*)W)?
        (?:(?P<days>[-+]?[0-9,.]*)D)?
        (?:T(?:(?P<hours>[-+]?[0-9,.]*)H)?
        (?:(?P<minutes>[-+]?[0-9,.]*)M)?
        (?:(?P<seconds>[-+]?[0-9,.]*)S)?)?$
    ")?;

    let captures = match re.captures(period) {
        Some(captures) => captures,
        None => bail!(ErrorKind::InvalidDuration),
    };

    let mut seconds: i64 = 0;
    for name in re.capture_names() {
        let capture_name = match name {
            Some(capture_name) => capture_name,
            None => continue,
        };
        let value = match captures.name(capture_name) {
            Some(value) => value.as_str().parse::<i64>()?,
            None => continue,
        };
        let multiplier = match capture_name {
            "years" => 31536000,
            "months" => 2592000,
            "weeks" => 604800,
            "days" => 86400,
            "hours" => 3600,
            "minutes" => 60,
            "seconds" => 1,
            _ => 0,
        };

        seconds += value * multiplier;
    }

    Ok(Duration::seconds(seconds))
}
