export default class AtbashCipher {
    readonly plain: string = 'abcdefghijklmnopqrstuvwxyz1234567890'
    readonly cipher: string = 'zyxwvutsrqponmlkjihgfedcba1234567890'

    encode(input: string): string {
        return this.chunk(this.translate(input, this.plain, this.cipher))
            .map((words) => words.join('')).join(' ')
    }

    decode(input: string): string {
        return this.translate(input, this.cipher, this.plain)
            .join('')
    }

    private translate(input: string, plain_map: string, cipher_map: string): string[] {
        return [... input]
            .filter((char) => /[A-Za-z0-9]/.test(char))
            .map((char) => cipher_map[plain_map.indexOf(char.toLowerCase())])
    }

    private chunk(input: string[], chunk_size: number = 5): string[][] {
        const result = []
        const length = input.length
        let r_index = 0
        let c_index = 0
        while (c_index < length) {
            result[r_index++] = input.slice(c_index, c_index + chunk_size)
            c_index += chunk_size
        }
        return result
    }
}