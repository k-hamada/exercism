class Cipher {
  constructor(key) {
    this.KEY_SET = 'abcdefghijklmnopqrstuvwxyz';
    this.constructor.validate(key, this.KEY_SET);
    this.key = key || this.constructor.generateKey(100, this.KEY_SET);
    this.offsets = this.constructor.offsets(this.key, this.KEY_SET);
  }

  static validate(key, keyset) {
    if (key === undefined) return;

    if ((key.length === 0) ||
        (Array.from(key).some(k => !keyset.includes(k)))) {
      throw new Error('Bad key');
    }
  }

  static generateKey(length, keyset) {
    const randomString = () => keyset.charAt(Math.floor(Math.random() * keyset.length));
    return [...Array(length)].map(randomString, this).join('');
  }

  static offsets(key, keyset) {
    return Array.from(key).map(k => keyset.indexOf(k));
  }

  static codepoints(text) {
    return Array.from(text).map(word => word.charCodeAt(0));
  }

  static cipher(code, base, offset) {
    return String.fromCharCode(base + (((code - base) + offset) % 26));
  }

  encode(text) {
    return this.constructor.codepoints(text)
      .map((code, index) => this.constructor.cipher(code, 97, this.getShift(index)), this)
      .join('');
  }

  decode(text) {
    return this.constructor.codepoints(text)
      .map((code, index) => this.constructor.cipher(code, 97, this.getUnshift(index)), this)
      .join('');
  }

  getShift(index) {
    return this.offsets[index % this.offsets.length];
  }

  getUnshift(index) {
    return 26 - (this.getShift(index) % 26);
  }
}

export default Cipher;
