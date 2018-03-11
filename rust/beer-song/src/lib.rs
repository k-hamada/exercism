pub fn verse(n: i32) -> String {
    format!(
        "{}, {}.\n{}, {}.\n",
        bottle_of_bear_on_the_wall(n),
        bottle_of_bear(n).to_lowercase(),
        take_beer(n),
        bottle_of_bear_on_the_wall(n - 1).to_lowercase()
    )
}

pub fn sing(start: i32, end: i32) -> String {
    (end .. start + 1).rev()
        .map(|n| verse(n))
        .collect::<Vec<_>>()
        .join("\n")
}

fn bottle(n: i32) -> String {
    match n {
        -1 => bottle(99),
        0 => "No more bottles".to_string(),
        1 => "1 bottle".to_string(),
        _ => format!("{} bottles", n)
    }
}

fn take_beer(n: i32) -> String {
    match n {
        0 => "Go to the store and buy some more".to_string(),
        1 => "Take it down and pass it around".to_string(),
        _ => "Take one down and pass it around".to_string()
    }
}

fn bottle_of_bear(n: i32) -> String {
    format!("{} of beer", bottle(n))
}

fn bottle_of_bear_on_the_wall(n: i32) -> String {
    format!("{} on the wall", bottle_of_bear(n))
}
