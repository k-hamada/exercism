export default class ComplexNumber {
    constructor(public readonly real: number, public readonly imag: number) {}

    public get abs(): number {
        return Math.sqrt(this.real ** 2 + this.imag ** 2)
    }

    public get conj(): ComplexNumber {
        const imag = (this.imag === 0) ? 0 : this.imag * -1
        return new ComplexNumber(this.real, imag)
    }

    public get exp(): ComplexNumber {
        const ex = Math.E ** this.real
        const real = ex * Math.cos(this.imag)
        const imag = ex * Math.sin(this.imag)
        return new ComplexNumber(real, imag)
    }

    public add(other: ComplexNumber): ComplexNumber {
        return new ComplexNumber(this.real + other.real, this.imag + other.imag)
    }

    public sub(other: ComplexNumber): ComplexNumber {
        return new ComplexNumber(this.real - other.real, this.imag - other.imag)
    }

    public mul(other: ComplexNumber): ComplexNumber {
        const real = this.real * other.real - this.imag * other.imag
        const imag = this.imag * other.real + this.real * other.imag
        return new ComplexNumber(real, imag)
    }

    public div(other: ComplexNumber): ComplexNumber {
        const denom = other.real ** 2 + other.imag ** 2
        const real = (this.real * other.real + this.imag * other.imag) / denom
        const imag = (this.imag * other.real - this.real * other.imag) / denom
        return new ComplexNumber(real, imag)
    }
}