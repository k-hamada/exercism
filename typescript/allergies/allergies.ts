enum Allergen {
    eggs         = 1 << 0,
    peanuts      = 1 << 1,
    shellfish    = 1 << 2,
    strawberries = 1 << 3,
    tomatoes     = 1 << 4,
    chocolate    = 1 << 5,
    pollen       = 1 << 6,
    cats         = 1 << 7,
}

type AllergenStrings = keyof typeof Allergen;

export default class Allergies {
    constructor(readonly score: number) {
    }

    allergicTo(input: AllergenStrings): boolean {
        const allergen = Allergen[input]
        return (this.score & allergen) === allergen
    }

    list(): AllergenStrings[] {
        return Object.values(Allergen)
            .filter(value => typeof value === 'string')
            .filter(value => this.allergicTo(value))
    }
}