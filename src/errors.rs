error_chain! {
    foreign_links {
        IrcError(::irc::error::Error);
        RegexError(::regex::Error);
        EnvVarError(::std::env::VarError);
        CurlError(::curl::Error);
        StringUtf8Error(::std::string::FromUtf8Error);
        SerdeJsonError(::serde_json::Error);
        TimeError(::time::ParseError);
    }
}
