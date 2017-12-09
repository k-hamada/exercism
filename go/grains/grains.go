package grains

import (
	"fmt"
	"math/bits"
)

// Square - how many grains were on each square
func Square(n int) (uint64, error) {
	switch {
	case n < 0:
		return 0, fmt.Errorf("negative square")
	case n == 0:
		return 0, fmt.Errorf("square 0")
	case n == 1:
		return 1, nil
	default:
		return bits.RotateLeft64(1, n-1), nil
	case 64 < n:
		return 0, fmt.Errorf("square greater than 64")
	}
}

// Total - total number of grains
func Total() uint64 {
	return 1<<64 - 1
}
