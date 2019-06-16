export default class Rational {
    constructor(public readonly numerator: number, public readonly denominator: number) {
    }

    add(other: Rational): Rational {
        const numerator = this.numerator * other.denominator + other.numerator * this.denominator
        const denominator = this.denominator * other.denominator
        return this.reduced_rational(numerator, denominator)
    }

    sub(other: Rational): Rational {
        const numerator = this.numerator * other.denominator - other.numerator * this.denominator
        const denominator = this.denominator * other.denominator
        return this.reduced_rational(numerator, denominator)
    }

    mul(other: Rational): Rational {
        const numerator = this.numerator * other.numerator
        const denominator = this.denominator * other.denominator
        return this.reduced_rational(numerator, denominator)
    }

    div(other: Rational): Rational {
        const numerator = this.numerator * other.denominator
        const denominator = this.denominator * other.numerator
        if (denominator === 0) { throw new Error('zero divide') }
        return this.reduced_rational(numerator, denominator)
    }

    abs(): Rational {
        const numerator = Math.abs(this.numerator)
        const denominator = Math.abs(this.denominator)
        return this.reduced_rational(numerator, denominator)
    }

    exprational(n: number): Rational {
        if (n >= 0) {
            const numerator = this.numerator ** n
            const denominator = this.denominator ** n
            return this.reduced_rational(numerator, denominator)
        } else {
            const m = Math.abs(n)
            const numerator = this.denominator ** m
            const denominator = this.numerator ** m
            return this.reduced_rational(numerator, denominator)
        }
    }

    expreal(n: number): number {
        return 10 ** (Math.log10(n ** this.numerator) / this.denominator)
    }

    reduce(): Rational {
        return this.reduced_rational(this.numerator, this.denominator)
    }

    private reduced_rational(numerator: number, denominator: number): Rational {
        const sign = denominator >= 0 ? 1 : -1
        const gcd = this.gcd(Math.abs(numerator), Math.abs(denominator))
        return new Rational(sign * numerator / gcd, sign * denominator / gcd)
    }

    private gcd(x: number, y: number): number {
        return y === 0 ? x : this.gcd(y, x % y)
    }
}