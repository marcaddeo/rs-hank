error_chain! {
    foreign_links {
        IrcError(::irc::error::Error);
        RegexError(::regex::Error);
        EnvVarError(::std::env::VarError);
        StringUtf8Error(::std::string::FromUtf8Error);
        SerdeJsonError(::serde_json::Error);
        TimeError(::time::ParseError);
        ParseIntError(::std::num::ParseIntError);
        UrlParseError(::url::ParseError);
        IoError(::std::io::Error);
        ReqwestError(::reqwest::Error);
        ParseFloatError(::std::num::ParseFloatError);
    }

    errors {
        InvalidDuration {
            description("Could not find a valid ISO 8601 duration."),
            display("Could not find a valid ISO 8601 duration."),
        }
    }
}
