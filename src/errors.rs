error_chain! {
    foreign_links {
        IrcError(::irc::error::Error);
        RegexError(::regex::Error);
        EnvVarError(::std::env::VarError);
        CurlError(::curl::Error);
        StringUtf8Error(::std::string::FromUtf8Error);
        SerdeJsonError(::serde_json::Error);
        TimeError(::time::ParseError);
        ParseIntError(::std::num::ParseIntError);
    }

    errors {
        InvalidDuration {
            description("Could not find a valid ISO 8601 duration."),
            display("Could not find a valid ISO 8601 duration."),

        }
    }
}
