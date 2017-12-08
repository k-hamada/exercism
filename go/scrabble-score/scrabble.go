package scrabble

import "strings"

type scrabble struct {
	letterScore map[string]int
}

func (s *scrabble) register(lv letterValue) {
	for _, l := range strings.Split(lv.letter, ", ") {
		s.letterScore[strings.ToLower(l)] = lv.value
	}
}

func (s *scrabble) score(letter string) int {
	return s.letterScore[strings.ToLower(letter)]
}

type letterValue struct {
	letter string
	value  int
}

// Score scrabble score
func Score(words string) int {
	s := &scrabble{make(map[string]int)}

	letterValues := []letterValue{
		{"A, E, I, O, U, L, N, R, S, T", 1},
		{"D, G", 2},
		{"B, C, M, P", 3},
		{"F, H, V, W, Y", 4},
		{"K", 5},
		{"J, X", 8},
		{"Q, Z", 10},
	}
	for _, lv := range letterValues {
		s.register(lv)
	}

	totalscore := 0
	for _, r := range words {
		totalscore += s.score(string(r))
	}
	return totalscore
}
