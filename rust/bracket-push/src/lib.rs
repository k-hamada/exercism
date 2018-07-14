struct Bracket(char);

impl Bracket {
    pub fn is_open(&self) -> bool {
        match self.0 {
            '[' => true,
            '{' => true,
            '(' => true,
            _ => false,
        }
    }
    pub fn is_close(&self) -> bool {
        match self.0 {
            ']' => true,
            '}' => true,
            ')' => true,
            _ => false,
        }
    }
    pub fn is_pair(&self, close: &Bracket) -> bool {
        match (self.0, close.0) {
            ('[', ']') => true,
            ('{', '}') => true,
            ('(', ')') => true,
            _ => false
        }
    }
}


pub struct Brackets(Vec<Bracket>);

impl From<&'static str> for Brackets {
    fn from(sentence: &str) -> Self {
        Brackets(sentence.chars().map(|c| Bracket(c)).collect())
    }
}

impl Brackets {
    pub fn are_balanced(&self) -> bool {
        let mut stack = Vec::new();

        for bracket in self.0.iter() {
            if bracket.is_open() {
                stack.push(bracket);
            }
            else if bracket.is_close() {
                let is_pair = {
                    if let Some(last_open) = stack.last() {
                        last_open.is_pair(bracket)
                    } else { false }
                };
                if !is_pair { return false; }

                stack.pop();
            }
        }
        stack.is_empty()
    }
}
