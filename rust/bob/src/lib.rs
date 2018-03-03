pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if is_silence(message) { return "Fine. Be that way!" }
    match (is_shout(message), is_ask(message)) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, _   ) => "Whoa, chill out!",
        (_   , true) => "Sure.",
        _            => "Whatever."
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
