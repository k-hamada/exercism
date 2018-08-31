enum Calculate {
    Integer(i32),
    Plus,
    Minus,
    Multiplied,
    Divided
}

impl Calculate {
    fn new(command: &str) -> Option<Self> {
        match (command, command.parse()) {
            (_, Ok(n)) => Some(Calculate::Integer(n)),
            ("plus", _) => Some(Calculate::Plus),
            ("minus", _) => Some(Calculate::Minus),
            ("multiplied", _) => Some(Calculate::Multiplied),
            ("divided", _) => Some(Calculate::Divided),
            _ => None,
        }
    }
}

pub struct WordProblem {
    commands: Vec<Calculate>
}

impl WordProblem {
    pub fn new(command: &str) -> Self {
        WordProblem {
            commands: command.trim_left_matches("What is ")
                .trim_right_matches('?')
                .split_whitespace()
                .filter_map(|s| Calculate::new(s))
                .collect::<Vec<_>>()
        }
    }

    pub fn answer(&self) -> Option<i32> {
        match self.commands.split_first() {
            Some((_, [])) => None,
            Some((&Calculate::Integer(i), commands)) =>
                commands.chunks(2).fold(Some(i), |acc, chunk| {
                    match chunk {
                        [Calculate::Plus, Calculate::Integer(n)] => acc.map(|m| m + n),
                        [Calculate::Minus, Calculate::Integer(n)] => acc.map(|m| m - n),
                        [Calculate::Multiplied, Calculate::Integer(n)] => acc.map(|m| m * n),
                        [Calculate::Divided, Calculate::Integer(n)] => acc.map(|m| m / n),
                        _ => None
                    }
                }),
            _ => None
        }
    }
}
