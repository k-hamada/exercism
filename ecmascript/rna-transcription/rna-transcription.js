class Transcriptor {
  constructor() {
    this.COMPLEMENT_TABLE = {
      G: 'C',
      C: 'G',
      T: 'A',
      A: 'U',
    };
  }

  toRna(dna) {
    return Array.from(dna).map(this.lookup, this).join('');
  }

  lookup(dna) {
    const transcribed = this.COMPLEMENT_TABLE[dna];
    if (!transcribed) {
      throw new Error('Invalid input DNA.');
    }
    return transcribed;
  }
}

export default Transcriptor;
