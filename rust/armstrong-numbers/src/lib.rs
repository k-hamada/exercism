pub fn is_armstrong_number(num: u32) -> bool {
    let number = num.to_string();
    let len = number.len() as u32;
    num == number.chars().map(|i| i.to_digit(10).unwrap().pow(len)).sum()
}
