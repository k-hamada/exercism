package luhn

import (
	"strconv"
	"strings"
)

// Valid check at luhn algorithm
func Valid(input string) bool {
	r := []rune(strings.Trim(input, " "))
	if len(r) <= 1 {
		return false
	}

	total := 0
	for i, alt := len(r)-1, false; i >= 0; i-- {
		s := string(r[i])
		if n, err := strconv.Atoi(s); err == nil {
			if alt {
				n *= 2
				if n > 9 {
					n -= 9
				}
			}
			total += n
			alt = !alt
		} else if s != " " {
			return false
		}
	}
	return total%10 == 0
}
