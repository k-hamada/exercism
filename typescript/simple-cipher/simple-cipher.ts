class SimpleCipher {
    KEYSET = 'abcdefghijklmnopqrstuvwxyz'

    readonly key: string
    readonly offsets: number[]

    constructor(key?: string) {
        if (key != null) {
            if (key.length === 0 || Array.from(key).some(c => !this.KEYSET.includes(c))) {
                throw new Error('Bad key')
            }
            this.key = key
        } else {
            this.key = this.generateKey(100)
        }

        this.offsets = Array.from(this.key).map(c => this.KEYSET.indexOf(c))
    }

    generateKey(length: number) {
        const randomString = () => this.KEYSET.charAt(Math.floor(Math.random() * this.KEYSET.length))
        return [...Array(length)].map(randomString, this).join('')
    }

    codepoints(text: string) {
        return Array.from(text).map(c => c.charCodeAt(0))
    }

    cipher(code: number, base: number, offset: number) {
        return String.fromCharCode(base + ((code - base + offset) % 26))
    }

    encode(text: string) {
        return this.codepoints(text)
            .map((code, index) => this.cipher(code, 97, this.getShift(index)))
            .join('')
    }

    decode(text: string) {
        return this.codepoints(text)
            .map((code, index) => this.cipher(code, 97, this.getUnshift(index)))
            .join('')
    }

    getShift(index: number) {
        return this.offsets[index % this.offsets.length]
    }

    getUnshift(index: number) {
        return 26 - (this.getShift(index) % 26)
    }
}

export default SimpleCipher
