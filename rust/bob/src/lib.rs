pub fn reply(message: &str) -> &str {
    let message = message.trim();

    match (is_shout(message), is_ask(message), is_silence(message)) {
        (_   , _   , true) => "Fine. Be that way!",
        (true, true, _   ) => "Calm down, I know what I'm doing!",
        (true, _   , _   ) => "Whoa, chill out!",
        (_   , true, _   ) => "Sure.",
        _                  => "Whatever."
    }
}

fn is_shout(message: &str) -> bool {
    message == message.to_uppercase() &&
    message != message.to_lowercase()
}

fn is_ask(message: &str) -> bool {
    message.ends_with("?")
}

fn is_silence(message: &str) -> bool {
    message.is_empty()
}
