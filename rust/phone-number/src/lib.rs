pub fn number(user_number: &str) -> Option<String> {
    let input_number = user_number.chars()
        .filter_map(|c| if c.is_ascii_digit() { c.to_digit(10) } else { None } )
        .collect::<Vec<_>>();

    let phone_number =
        if input_number.len() == 10 {
            Some(input_number.as_slice())
        } else if input_number.len() == 11 && input_number.get(0) == Some(&1) {
            input_number.get(1..)
        } else {
            None
        };

    if let Some(number) = phone_number {
        match number {
            [n1, _, _, n2, _, _, _, _, _, _] if n1 >= &2 && n2 >= &2 =>
                number.into_iter().cloned().filter_map(|i| std::char::from_digit(i, 10)).collect::<String>().into(),
            _ => None
        }
    } else {
        None
    }
}
