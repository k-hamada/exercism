package bob

import (
	"regexp"
	"strings"
)

func say(s string) bool {
	return len(s) == 0
}

func yell(s string) bool {
	re := regexp.MustCompile("[A-Za-z]+")
	ws := re.FindAllString(s, -1)
	y := len(ws) != 0
	for _, w := range ws {
		y = y && strings.ToUpper(w) == w
	}
	return y
}

func ask(s string) bool {
	return strings.HasSuffix(s, "?")
}

// Hey bob
func Hey(remark string) string {
	remark = strings.TrimSpace(remark)

	switch {
	case say(remark):
		return "Fine. Be that way!"
	case yell(remark):
		return "Whoa, chill out!"
	case ask(remark):
		return "Sure."
	default:
		return "Whatever."
	}
}
