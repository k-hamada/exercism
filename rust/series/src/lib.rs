pub fn series(digits: &str, len: usize) -> Vec<String> {
    match digits.len() {
        l if l == 0 =>
            vec!["".to_string(); 6],
        l if l >= len =>
            (0 .. l - len + 1).into_iter()
                .map(|i| digits[i .. i + len].to_string())
                .collect::<Vec<_>>(),
        _  =>
            vec![]
    }
}
