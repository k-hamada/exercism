package grains

import (
	"fmt"
	"math"
)

func Square(n int) (uint64, error) {
	switch {
	case n < 0:
		return 0, fmt.Errorf("negative square returns")
	case n == 0:
		return 0, fmt.Errorf("square 0 returns")
	case n == 1:
		return 1, nil
	default:
		return uint64(math.Pow(2, float64(n-1))), nil
	case 64 < n:
		return 0, fmt.Errorf("square greater than 64 returns")
	}
}

func Total() uint64 {
	var r uint64
	for i := 1; i <= 64; i++ {
		if n, err := Square(i); err == nil {
			r += n
		}
	}
	return r
}
