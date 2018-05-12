use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

fn is_valid(dna: &str) -> bool {
    dna.to_string().chars().all(|n| NUCLEOTIDES.contains(&n) )
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, ()> {
    if !NUCLEOTIDES.contains(&nucleotide) || !is_valid(dna) {
        return Err(())
    };

    let count = dna.match_indices(nucleotide).count();

    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, ()> {
    if !is_valid(dna) {
        return Err(())
    }

    let mut hashmap = HashMap::new();
    for nucleotide in NUCLEOTIDES.iter() {
        hashmap.insert(*nucleotide, 0);
    }
    for nucleotide in dna.to_string().chars() {
        *hashmap.entry(nucleotide).or_insert(0) += 1;
    }

    Ok(hashmap)
}
