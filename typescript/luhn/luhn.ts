export default class Luhn {
    static valid(input: string): boolean {
        if (!/\d|\s/.test(input)) return false
        if (input.trim().length <= 1) return false

        const sum = input.replace(/\s/g, '').split('')
            .reduceRight((acc, val, idx, arr) => acc + this.calc(Number(val), arr.length - idx), 0)
        return sum % 10 === 0
    }

    static calc(n: number, rnum: number): number {
        if (rnum % 2 === 0) {
            const res = n * 2
            return (res >= 9) ? res - 9 : res
        }
        return n
    }
}