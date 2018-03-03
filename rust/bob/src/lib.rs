pub fn reply(message: &str) -> &str {
    let chars = message.chars().filter(|&c| !c.is_whitespace()).collect::<Vec<char>>();
    let shout = is_shout(&chars);
    let ask = is_ask(&chars);
    let silence = is_silence(&chars);

    if shout && ask {
        return "Calm down, I know what I'm doing!"
    }
    if shout {
        return "Whoa, chill out!"
    }
    if ask {
        return "Sure."
    }
    if silence {
        return "Fine. Be that way!"
    }
    "Whatever."
}

fn is_shout(chars: &Vec<char>) -> bool {
    chars == &chars.iter().map(|c| c.to_ascii_uppercase()).collect::<Vec<char>>() &&
    chars != &chars.iter().map(|c| c.to_ascii_lowercase()).collect::<Vec<char>>()
}

fn is_ask(chars: &Vec<char>) -> bool {
    chars.last().map_or(false, |&s| s == '?')
}

fn is_silence(chars: &Vec<char>) -> bool {
    chars.len() == 0
}
