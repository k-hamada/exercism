const DefaultNucleotideCount = { A: 0, C: 0, G: 0, T: 0 }
type NucleotideCountResult = { [key in keyof typeof DefaultNucleotideCount]: number }

class NucleotideCount {
  static nucleotideCounts(dna: string): NucleotideCountResult {
    let result = Object.assign({}, DefaultNucleotideCount)
    for (const nucleotide of dna) {
      switch (nucleotide) {
        case 'A': result.A += 1; break;
        case 'C': result.C += 1; break;
        case 'G': result.G += 1; break;
        case 'T': result.T += 1; break;
        default: throw new Error('Invalid nucleotide in strand')
      }
    }
    return result
  }
}

export default NucleotideCount
