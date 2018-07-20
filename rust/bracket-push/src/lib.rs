struct Bracket(char);

impl Bracket {
    pub fn to_close(&self) -> Option<char> {
        match self.0 {
            '[' => Some(']'),
            '{' => Some('}'),
            '(' => Some(')'),
            _ => None,
        }
    }

    pub fn is_close(&self) -> bool {
        match self.0 {
            ']' | '}' | ')' => true,
            _ => false,
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
            if let Some(close) = bracket.to_close() {
                stack.push(close);
            } else if bracket.is_close() {
                if Some(bracket.0) != stack.pop() {
                    return false;
                }
            }
        }

        stack.is_empty()
    }
}
