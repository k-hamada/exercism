type Grade = string
type Students = string[]
type Roster = Map<Grade, Students>

export default class GradeSchool {
    readonly roster: Roster = new Map()

    studentRoster(): Roster {
        const result = new Map()
        for (const [key, value] of this.roster.entries()) {
            result.set(key, [... value])
        }
        return result
    }

    addStudent(student: string, grade: number) {
        const value = [... this.studentsInGrade(grade), student]
        value.sort()
        this.roster.set(this.toKey(grade), value)
    }

    studentsInGrade(grade: number): Students {
        return [... this.roster.get(this.toKey(grade)) || []]
    }

    toKey(grade: number): Grade {
        return grade.toString()
    }
}