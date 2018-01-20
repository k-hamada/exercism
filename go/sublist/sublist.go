package sublist

type Relation string

const (
	IsEqual     Relation = "equal"
	IsSublist   Relation = "sublist"
	IsSuperlist Relation = "superlist"
	IsUnequal   Relation = "unequal"
)

func Sublist(listOne, listTwo []int) Relation {
	lenOne, lenTwo := len(listOne), len(listTwo)
	if (lenOne == lenTwo) && compare(listOne, listTwo) {
		return IsEqual
	}
	if (lenOne < lenTwo) && isSublist(listOne, listTwo, lenOne, lenTwo) {
		return IsSublist
	}
	if (lenOne > lenTwo) && isSublist(listTwo, listOne, lenTwo, lenOne) {
		return IsSuperlist
	}
	return IsUnequal
}

func isSublist(listOne, listTwo []int, lenOne, lenTwo int) bool {
	for i := 0; i <= (lenTwo - lenOne); i++ {
		if compare(listOne, listTwo[i:i+lenOne]) {
			return true
		}
	}
	return false
}

func compare(a, b []int) bool {
	b = b[:len(a)]
	for i, v := range a {
		if v != b[i] {
			return false
		}
	}
	return true
}
