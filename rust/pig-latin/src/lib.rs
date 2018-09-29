pub fn translate(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            let words = word.to_string();
            let (first, last) = words.split_at(split_size(word));
            format!("{}{}ay", last, first)
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn split_size(word: &str) -> usize {
    let rules = vec![
        ("squ", 3),
        ("thr", 3),
        ("sch", 3),
        ("ch", 2),
        ("qu", 2),
        ("th", 2),
        ("a", 0),
        ("o", 0),
        ("i", 0),
        ("u", 0),
        ("e", 0),
        ("xr", 0),
        ("yt", 0),
    ];

    for rule in rules {
        if word.starts_with(rule.0) {
            return rule.1;
        }
    }

    return 1;
}
