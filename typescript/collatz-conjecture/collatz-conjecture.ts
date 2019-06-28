class CollatzConjecture {
    static steps(n: number): number {
        return CollatzConjecture.stepsImpl(0, n)
    }

    static stepsImpl(step: number, n: number): number {
        if (n <= 0) throw new Error('Only positive numbers are allowed')
        if (n === 1) return step
        if (n % 2 === 0) return this.stepsImpl(step + 1, n / 2)
        return this.stepsImpl(step + 1, (n * 3) + 1)
    }
}

export default CollatzConjecture