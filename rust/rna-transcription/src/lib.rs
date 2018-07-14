#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Nucleotide {
    G, C, T, A, U,
    Error
}

impl From<char> for Nucleotide {
    fn from(c: char) -> Self {
        match c.to_ascii_uppercase() {
            'A' => Nucleotide::A,
            'C' => Nucleotide::C,
            'G' => Nucleotide::G,
            'T' => Nucleotide::T,
            'U' => Nucleotide::U,
            _   => Nucleotide::Error,
        }
    }
}

impl Nucleotide {
    fn is_dna(self) -> Option<Self> {
        match self {
            Nucleotide::A | Nucleotide::C | Nucleotide::G | Nucleotide::T => Some(self),
            _ => None
        }
    }

    fn is_rna(self) -> Option<Self> {
        match self {
            Nucleotide::A | Nucleotide::C | Nucleotide::G | Nucleotide::U => Some(self),
            _ => None
        }
    }

    fn to_rna(self) -> Option<Self> {
        match self {
            Nucleotide::A => Some(Nucleotide::U),
            Nucleotide::C => Some(Nucleotide::G),
            Nucleotide::G => Some(Nucleotide::C),
            Nucleotide::T => Some(Nucleotide::A),
            _ => None
        }
    }
}

#[derive(Debug)]
pub struct DNA {
    pub nucleotides: Vec<Nucleotide>
}

#[derive(Debug)]
pub struct RNA {
    pub nucleotides: Vec<Nucleotide>
}

impl DNA {
    pub fn new(rna: &str) -> Result<Self, ()> {
        Self::parse(rna)
            .map(|nucleotides| Self::create(nucleotides) )
            .ok_or(())
    }

    fn parse(nucleotides: &str) -> Option<Vec<Nucleotide>> {
        nucleotides.chars()
            .map(|c| Nucleotide::from(c) )
            .map(|c| Self::verify(c) )
            .collect::<Option<Vec<_>>>()
    }

    fn verify(n: Nucleotide) -> Option<Nucleotide> {
        n.is_dna()
    }

    fn create(nucleotides: Vec<Nucleotide>) -> Self {
        DNA { nucleotides: nucleotides }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<Self, ()> {
        Self::parse(rna)
            .map(|nucleotides| Self::create(nucleotides) )
            .ok_or(())
    }

    fn parse(nucleotides: &str) -> Option<Vec<Nucleotide>> {
        nucleotides.chars()
            .map(|c| Nucleotide::from(c) )
            .map(|c| Self::verify(c) )
            .collect::<Option<Vec<_>>>()
    }

    fn verify(n: Nucleotide) -> Option<Nucleotide> {
        n.is_rna()
    }

    fn create(nucleotides: Vec<Nucleotide>) -> Self {
        RNA { nucleotides: nucleotides }
    }
}

impl PartialEq for DNA {
    fn eq(&self, other: &Self) -> bool {
        self.nucleotides == other.nucleotides
    }
}

impl PartialEq for RNA {
    fn eq(&self, other: &Self) -> bool {
        self.nucleotides == other.nucleotides
    }
}

impl DNA {
    pub fn to_rna(&self) -> RNA {
        self.nucleotides
            .iter()
            .map(|n| n.to_rna() )
            .collect::<Option<Vec<_>>>()
            .map(|nucleotides| RNA { nucleotides: nucleotides } )
            .unwrap()
    }
}
