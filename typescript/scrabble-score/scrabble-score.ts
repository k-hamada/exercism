const SCORE_MAP = {
   AEIOULNRST: 1,
   DG: 2,
   BCMP: 3,
   FHVWY: 4,
   K: 5,
   JX: 8,
   QZ: 10,
}

class ScoreMap {
    readonly _map = new Map<string, number>()

    constructor(source: { [n: string]: number } = SCORE_MAP) {
        for (const [k, v] of Object.entries(source)) {
            for (const c of Array.from(k)) {
                this._map.set(c, v)
            }
        }
    }

    public score(input: string): number {
        return Array.from(input.toUpperCase())
            .map((c) => this.get(c))
            .reduce((prev, current) => prev + current, 0)
    }

    get(key: string): number {
        return this._map.get(key) || 0
    }
}

export default function score(input: string | undefined): number {
    if (input === undefined) { return 0 }

    return new ScoreMap().score(input)
}