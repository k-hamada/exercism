pub type Error = ();

pub struct Scale {
    use_flats: bool,
    start: usize,
    steps: Vec<usize>,
}

const SHARPS: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];
const FLATS: [&str; 12] = [
    "Ab", "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G",
];
const USE_FLATS: [&str; 12] = [
    "F", "Bb", "Eb", "Ab", "Db", "Gb", "d", "g", "c", "f", "bb", "eb",
];

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let steps = intervals.chars().map(Self::interval).collect::<Vec<_>>();
        Self::generate(tonic, steps)
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        let steps = vec![0; 11];
        Self::generate(tonic, steps)
    }

    pub fn enumerate(&self) -> Vec<String> {
        let pitches = if self.use_flats { FLATS } else { SHARPS };
        let mut iter = pitches.iter().cycle().skip(self.start).take(pitches.len());

        vec![0]
            .iter()
            .chain(self.steps.iter())
            .copied()
            .filter_map(|n| iter.nth(n))
            .copied()
            .map(String::from)
            .collect::<Vec<_>>()
    }

    fn interval(scale: char) -> usize {
        match scale {
            'A' => 2,
            'M' => 1,
            'm' => 0,
            _ => 0,
        }
    }

    fn generate(tonic: &str, steps: Vec<usize>) -> Result<Scale, Error> {
        let use_flats = USE_FLATS.contains(&tonic);
        let pitches = if use_flats { FLATS } else { SHARPS };
        let start = pitches
            .iter()
            .position(|pitche| pitche.to_uppercase() == tonic.to_uppercase())
            .ok_or(())?;

        Ok(Scale {
            use_flats,
            start,
            steps,
        })
    }
}
