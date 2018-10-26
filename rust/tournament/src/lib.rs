use std::collections::HashMap;

struct Match<'a> {
    name: &'a str,
    won: u32,
    drawn: u32,
    lost: u32,
}

impl<'a> Match<'a> {
    fn new(name: &'a str) -> Self {
        Match {
            name,
            won: 0,
            drawn: 0,
            lost: 0,
        }
    }

    fn played(&self) -> u32 {
        self.won + self.drawn + self.lost
    }

    fn point(&self) -> u32 {
        self.won * 3 + self.drawn
    }

    fn output(&self) -> String {
        format!(
            "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
            self.name,
            self.played(),
            self.won,
            self.drawn,
            self.lost,
            self.point()
        )
    }
}

pub fn tally(input: &str) -> String {
    let mut map: HashMap<&str, Match> = HashMap::new();

    for line in input.lines() {
        match &line.split(';').collect::<Vec<_>>()[..] {
            [a, b, "win"] |
            [b, a, "loss"] => {
                map.entry(a).or_insert_with(|| Match::new(a)).won += 1;
                map.entry(b).or_insert_with(|| Match::new(b)).lost += 1;
            }
            [a, b, "draw"] => {
                map.entry(a).or_insert_with(|| Match::new(a)).drawn += 1;
                map.entry(b).or_insert_with(|| Match::new(b)).drawn += 1;
            }
            _ => continue,
        }
    }

    let mut output = String::from(
        format!("{:30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}", "Team", "MP", "W", "D", "L", "P")
    );

    let mut values = map.values().collect::<Vec<_>>();
    values.sort_by(|a, b| b.point().cmp(&a.point()).then(a.name.cmp(&b.name)));
    for value in values {
        output.push_str("\n");
        output.push_str(&value.output());
    }

    output
}
