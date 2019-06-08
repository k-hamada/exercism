const Events = {
    'wink': 0b00001,
    'double blink': 0b00010,
    'close your eyes': 0b00100,
    'jump': 0b01000,
}

export default class HandShake {
    readonly n: number;
    readonly reverse: boolean;

    constructor(n: number) {
        this.n = n
        this.reverse = this.match(n, 0b10000)
    }

    commands(): string[] {
        return Object.entries(Events)
            .filter(([_, value]) => this.match(this.n, value))
            .map(([key, _]) => key)
            .sort(_ => this.reverse ? 1 : 0)
    }
    
    match(lhs: number, rhs: number): boolean {
        return (lhs & rhs) !== 0
    }
}