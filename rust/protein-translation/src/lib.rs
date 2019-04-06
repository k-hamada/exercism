pub struct CodonsInfo<'a> {
    pairs: Vec<(&'a str, &'a str)>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.pairs
            .iter()
            .find(|(p_codon, _)| p_codon == &codon)
            .map(|&(_, p_name)| p_name)
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna.chars()
            .collect::<Vec<_>>()
            .chunks_exact(3)
            .map(|chunk| chunk.iter().collect::<String>())
            .map(|codon| self.name_for(&codon))
            .take_while(|name| name.map_or(false, |name| name != "stop codon"))
            .into_iter()
            .collect::<Option<Vec<_>>>()
            .filter(|x| !x.is_empty())
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo { pairs }
}
