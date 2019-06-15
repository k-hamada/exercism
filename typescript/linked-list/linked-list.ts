class Node<T> {
    value: T
    next: Node<T> | undefined

    constructor(value: T, next: Node<T> | undefined) {
        this.value = value
        this.next = next
    }
}

export default class LinkedList<T> {
    head: Node<T> | undefined = undefined

    push(value: T) {
        this.head = new Node(value, this.head)
    }

    pop(): T {
        const head = this.head
        if (head === undefined) { throw new Error() }

        this.head = head.next
        return head.value
    }

    shift(): T {
        let head = this.head
        if (!head) { throw new Error() }

        if (!head.next) {
            if (!head.value) { throw new Error() }

            this.head = undefined
            return head.value
        }

        let tail = head.next
        while (tail.next !== undefined) {
            head = tail
            tail = tail.next
        }
        head.next = undefined
        return tail.value
    }

    unshift(value: T) {
        const node = new Node(value, undefined)

        let head = this.head
        if (head === undefined) {
            this.head = node
            return
        }

        while (head.next !== undefined) {
            head = head.next
        }
        head.next = node
    }

    delete(value: T) {
        let head = this.head
        if (head === undefined) { return }

        if (head.value === value) {
            this.head = head.next
        }

        while (head.next !== undefined) {
            if (head.next.value !== value) {
                head = head.next
            } else {
                head.next = head.next.next
            }
        }
    }

    count(): number {
        let count = 0

        let head = this.head
        while (head !== undefined) {
            head = head.next
            count++
        }

        return count
    }
}