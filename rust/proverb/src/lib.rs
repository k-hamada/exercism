pub fn build_proverb(list: Vec<&str>) -> String {
    let len = list.len();
    if len <= 0 {
        return String::new();
    }

    let iter = list.iter();
    iter.clone().take(len - 1).zip(iter.clone().skip(1))
        .map(|(x, y)| format!("For want of a {} the {} was lost.", x, y))
        .chain(vec![format!("And all for the want of a {}.", iter.clone().next().unwrap())])
        .collect::<Vec<_>>()
        .join("\n")
}
