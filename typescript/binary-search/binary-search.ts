export default class BinarySearch<T> {
    static readonly NOT_FOUND: number = -1

    readonly array: T[]

    constructor(array: T[]) {
        if (!this.validate(array)) return

        this.array = array
    }

    indexOf(key: T): number {
        return this.bsearch(this.array, key, 0, this.array.length - 1)
    }

    private validate(array: T[]): boolean {
        for (let i = 0, l = array.length - 1; i < l; i++) {
            if (array[i] > array[i + 1]) {
                return false
            }
        }
        return true
    }

    private bsearch(array: T[], key: T, min: number, max: number): number {
        if (max < min) {
            return BinarySearch.NOT_FOUND
        }
        
        const mid = Math.floor((max + min) / 2)
        if (array[mid] > key) {
            return this.bsearch(array, key, min, mid - 1)
        } else if (array[mid] < key) {
            return this.bsearch(array, key, mid + 1, max)
        } else {
            return mid
        }
    }
}
