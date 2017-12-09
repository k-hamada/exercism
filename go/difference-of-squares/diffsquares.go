package diffsquares

// SquareOfSums (1 + ... + n)²
func SquareOfSums(n int) int {
	s := (n * (1 + n)) / 2
	return s * s
}

// SumOfSquares (1² + ... + n²)
func SumOfSquares(n int) int {
	return (n * (n + 1) * (2*n + 1)) / 6
}

// Difference SquareOfSums - SumOfSquares
func Difference(n int) int {
	return SquareOfSums(n) - SumOfSquares(n)
}
