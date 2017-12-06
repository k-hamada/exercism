package hamming

import (
	"fmt"
)

// Distance : Hammint distance
func Distance(a, b string) (int, error) {
	if len(a) != len(b) {
		return -1, fmt.Errorf("invalid strings len(a) == %d, len(b) == %d", len(a), len(b))
	}

	if a == b {
		return 0, nil
	}

	var res int
	var rb = []rune(b)
	for i, ar := range a {
		if rb[i] != ar {
			res++
		}
	}

	return res, nil
}
