package acronym

import (
	"regexp"
	"strings"
)

// Abbreviate split space and hyphen
func Abbreviate(s string) string {
	var a string
	for _, w := range regexp.MustCompile("\\s|\\-").Split(s, 5) {
		a = a + strings.ToUpper(string([]rune(w)[0]))
	}
	return a
}
