export default class Triplet {
    constructor(readonly a: number, readonly b: number, readonly c: number) {
    }

    sum(): number {
        return this.a + this.b + this.c
    }

    product(): number {
        return this.a * this.b * this.c
    }

    isPythagorean(): boolean {
        return this.a ** 2 + this.b ** 2 === this.c ** 2
    }

    static where(max: number, min: number = 1, sum?: number): Triplet[] {
        const triplets = []
        for (let a = min; a <= max; a++) {
            for (let b = a + 1; b <= max; b++) {
                for (let c = b + 1; c <= max; c++) {
                    const triplet = new Triplet(a, b, c)
                    if (triplet.isPythagorean()) {
                        triplets.push(triplet)
                    }
                }
            }
        }

        if (sum) {
            return triplets.filter((triplet) => sum === triplet.sum())
        }

        return triplets
    }
}