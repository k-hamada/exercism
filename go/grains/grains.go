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
	r, ch := uint64(0), make(chan uint64)
	for i := 1; i <= 64; i++ {
		go func(i int, c chan<- uint64) {
			if n, err := Square(i); err == nil {
				c <- n
			} else {
				c <- 0
			}
		}(i, ch)
	}
	for i := 1; i <= 64; i++ {
		r += <-ch
	}
	return r
}
