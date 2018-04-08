pub fn encode(n : u64) -> String {
    match n {
        0 => String::from("zero"),
        _ => do_encode(n).trim().to_string()
    }
}

fn do_encode(n : u64) -> String {
    match n {
        1 ... 9 =>
            a_digit(n),
        11 ... 19 =>
            teen_digit(n),
        10 ... 90 if n % 10 == 0 =>
            ty_digit(n / 10),
        20 ... 99 =>
            format!("{}-{}", ty_digit(n / 10), do_encode(n % 10)),
        100 ... 999 =>
            format!("{} hundred {}", do_encode(n / 100), do_encode(n % 100)),
        1_000 ... 999_999 =>
            format!("{} thousand {}", do_encode(n / 1_000), do_encode(n % 1_000)),
        1_000_000 ... 999_999_999 =>
            format!("{} million {}", do_encode(n / 1_000_000), do_encode(n % 1_000_000)),
        1_000_000_000 ... 999_999_999_999 =>
            format!("{} billion {}", do_encode(n / 1_000_000_000), do_encode(n % 1_000_000_000)),
        1_000_000_000_000 ... 999_999_999_999_999 =>
            format!("{} trillion {}", do_encode(n / 1_000_000_000_000), do_encode(n % 1_000_000_000_000)),
        1_000_000_000_000_000 ... 999_999_999_999_999_999 =>
            format!("{} quadrillion {}", do_encode(n / 1_000_000_000_000_000), do_encode(n % 1_000_000_000_000_000)),
        1_000_000_000_000_000_000 ... 9_999_999_999_999_999_999 =>
            format!("{} quintillion {}", do_encode(n / 1_000_000_000_000_000_000), do_encode(n % 1_000_000_000_000_000_000)),
        _ =>
            String::new()
    }
}

fn a_digit(n: u64) -> String {
    let word = match n {
      1 => "one",
      2 => "two",
      3 => "three",
      4 => "four",
      5 => "five",
      6 => "six",
      7 => "seven",
      8 => "eight",
      9 => "nine",
      _ => ""
  };
  String::from(word)
}

fn teen_digit(n: u64) -> String {
    match n {
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        15 => "fifteen".to_string(),
        18 => "eighteen".to_string(),
        _ => format!("{}teen", a_digit(n % 10))
    }
}

fn ty_digit(n: u64) -> String {
    match n {
      1 => "ten".to_string(),
      2 => "twenty".to_string(),
      3 => "thirty".to_string(),
      4 => "forty".to_string(),
      5 => "fifty".to_string(),
      8 => "eighty".to_string(),
      _ => format!("{}ty", a_digit(n))
    }
}
