package isogram

import (
	"strings"
)

var ignore = map[rune]struct{}{
	'-': struct{}{},
	' ': struct{}{},
}

// IsIsogram isogram is a word or phrase without a repeating letter
func IsIsogram(s string) bool {
	set := make(map[rune]struct{})
	for _, r := range strings.ToLower(s) {
		if _, skip := ignore[r]; skip {
			continue
		}
		if _, ok := set[r]; ok {
			return false
		}
		set[r] = struct{}{}
	}
	return true
}
