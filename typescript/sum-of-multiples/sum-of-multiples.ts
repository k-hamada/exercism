class SumOfMultiples {
    constructor(readonly numbers: number[]) { }

    to(limit: number): number {
        const multiple_numbers = this.numbers.map((n) => this.multiple_numbers(n, limit))
        return this.sum([...new Set(this.flat(multiple_numbers))])
    }

    private multiple_numbers(n: number, limit: number): number[] {
        if (limit <= 1) { return [] }

        const size = (limit % n === 0) ? limit / n - 1 : Math.floor(limit / n)
        return [...Array(size)].map((_, i) => n * (i + 1))
    }

    private flat(args: number[][]): number[] {
        return args.reduce((acc, val) => acc.concat(val), [])
    }

    private sum(args: number[]): number {
        return args.reduce((acc, cur) => acc + cur, 0)
    }
}

export default function (numbers: number[]): SumOfMultiples {
    return new SumOfMultiples(numbers)
}