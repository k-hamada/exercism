class Matrix {
    rows: number[][];
    columns: number[][];

    constructor(input: string) {
        this.rows = input.split(/\n/).map(i => i.split(/\s/).map(t => parseInt(t)))
        this.columns = this.transpose(this.rows)
    }

    transpose(array: number[][]): number[][] {
        return array[0].map((_, c) => array.map(r => r[c]))
    }
}

export default Matrix
