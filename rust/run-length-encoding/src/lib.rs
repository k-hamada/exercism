struct Char {
    c: char,
    i: usize,
}

impl Char {
    pub fn new(c: char, i: usize) -> Char {
        Char { c: c, i: i }
    }

    pub fn same(&self, c: char) -> bool {
        self.c == c
    }

    pub fn increment(&mut self) -> () {
        self.i += 1
    }

    pub fn encode(&self) -> String {
        if self.i <= 1 {
            format!("{}", self.c)
        } else {
            format!("{}{}", self.i, self.c)
        }
    }

    pub fn decode(&self) -> String {
        format!("{}", self.c).repeat(self.i)
    }
}

pub fn encode(code: &str) -> String {
    if code.len() == 0 {
        return String::new();
    }

    let codes = code.chars().collect::<Vec<char>>();
    let mut chars: Vec<Char> = vec![];
    let mut last = Char::new(codes[0], 1);

    for c in codes.into_iter().skip(1) {
        if last.same(c) {
            last.increment();
        } else {
            chars.push(last);
            last = Char::new(c, 1);
        }
    }
    chars.push(last);

    chars.iter().map(|c| c.encode()).collect()
}

pub fn decode(code: &str) -> String {
    if code.len() == 0 {
        return String::new();
    }

    let codes = code.chars().collect::<Vec<char>>();
    let mut chars: Vec<Char> = vec![];
    let mut n = String::new();

    for c in codes.into_iter() {
        if c.is_digit(10) {
            n.push(c);
        } else {
            chars.push(Char::new(c, n.parse().unwrap_or(1)));
            n.clear();
        }
    }

    chars.iter().map(|c| c.decode()).collect()
}
