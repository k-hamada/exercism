pub fn abbreviate(text: &str) -> String {
    text.chars()
        .scan(' ', |prev, word| {
            let ret;
            if !prev.is_ascii_alphabetic() && word.is_ascii_alphabetic()
                || prev.is_lowercase() && word.is_uppercase()
            {
                ret = Some(Some(word.to_ascii_uppercase()));
            } else {
                ret = Some(None);
            }
            *prev = word;
            ret
        })
        .flat_map(|o| o)
        .collect()
}
