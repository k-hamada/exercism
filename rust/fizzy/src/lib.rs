pub struct Matcher<T> {
    matcher: Box<dyn Fn(T) -> bool>,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: ToString,
    {
        Matcher {
            matcher: Box::new(matcher),
            subs: subs.to_string(),
        }
    }
}

pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T> Fizzy<T> {
    pub fn new() -> Self {
        Fizzy { matchers: vec![] }
    }

    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        T: ToString + Copy,
        I: Iterator<Item = T>,
    {
        iter.map(move |n| {
            let subs = self
                .matchers
                .iter()
                .filter(|m| (m.matcher)(n))
                .map(|m| m.subs.to_string())
                .collect::<String>();

            return if subs.is_empty() { n.to_string() } else { subs };
        })
    }
}

pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: std::ops::Rem<Output = T> + PartialEq + From<u8>,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n| n % 5.into() == 0.into(), "buzz"))
}
