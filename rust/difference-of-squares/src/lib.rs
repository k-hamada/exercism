pub fn square_of_sum(n: usize) -> usize {
	let s = (n * (1 + n)) / 2;
	s * s
}

pub fn sum_of_squares(n: usize) -> usize {
	(n * (n + 1) * (2 * n + 1)) / 6
}

pub fn difference(n: usize) -> usize {
	square_of_sum(n) - sum_of_squares(n)
}
