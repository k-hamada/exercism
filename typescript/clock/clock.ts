export default class Clock {
    readonly total_minute: number

    constructor(hour: number, minute: number = 0) {
        this.total_minute = hour * 60 + minute
    }

    private get hour(): number {
        const h = Math.floor(this.total_minute / 60 % 24)
        return h >= 0 ? h : 24 + h
    }

    private get minute(): number {
        const m = this.total_minute % 60
        return m >= 0 ? m : 60 + m
    }

    private padTwo(n: number): string {
        return n.toString().padStart(2, '0')
    }

    toString(): string {
        return `${this.padTwo(this.hour)}:${this.padTwo(this.minute)}`
    }

    plus(minute: number): Clock {
        return new Clock(0, this.total_minute + minute)
    }

    minus(minute: number): Clock {
        return new Clock(0, this.total_minute - minute)
    }

    equals(other: Clock): boolean {
        return this.toString() === other.toString()
    }
}