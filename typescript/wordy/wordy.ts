export class ArgumentError implements Error {
    public name = "ArgumentError"

    constructor(public message: string) {
    }

    toString() {
        return this.name + ': ' + this.message;
    }
}

class Result {
    constructor(public value: number) {
    }
}

class Operation {
    readonly value: number
    readonly length: number

    constructor(input: string, pattern: RegExp) {
        const re = input.match(pattern)
        if (re !== null) {
            this.value = parseInt(re[1], 10)
            this.length = re[0].length
        }
    }

    public get isValid(): boolean {
        return this.value !== undefined
    }
}

class Num extends Operation {
    constructor(input: string) {
        super(input, /^What is (-?\d+)/)
    }
}

class Add extends Operation {
    constructor(input: string) {
        super(input, /^ plus (-?\d+)\??/)
    }
}

class Subtract extends Operation {
    constructor(input: string) {
        super(input, /^ minus (-?\d+)\??/)
    }
}

class Multiply extends Operation {
    constructor(input: string) {
        super(input, /^ multiplied by (-?\d+)\??/)
    }
}

class Divide extends Operation {
    constructor(input: string) {
        super(input, /^ divided by (-?\d+)\??/)
    }
}

export class WordProblem {
    constructor(readonly question: string) {
    }

    answer(): number {
        return this.parse(this.question)
            .reduce(this.calc, new Result(0))
            .value
    }

    private parse(question: string): Operation[] {
        const result = []
        while (question.length) {
            const word = [
                new Num(question),
                new Add(question),
                new Subtract(question),
                new Multiply(question),
                new Divide(question),
            ].find((op) => op.isValid)

            if (word === undefined) {
                throw new ArgumentError(`parse error: ${ question }`)
            }

            question = question.slice(word.length)
            result.push(word)
        }
        return result
    }

    private calc(acc: Result, cur: Operation, idx: number): Result {
        if (cur instanceof Num && idx === 0) {
            return new Result(cur.value)
        }
        if (cur instanceof Add) {
            return new Result(acc.value + cur.value)
        }
        if (cur instanceof Subtract) {
            return new Result(acc.value - cur.value)
        }
        if (cur instanceof Multiply) {
            return new Result(acc.value * cur.value)
        }
        if (cur instanceof Divide) {
            return new Result(acc.value / cur.value)
        }

        throw new ArgumentError(`calc error: ${ acc }, ${ cur }`)
    }
}