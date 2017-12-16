package letter

type FreqMap map[rune]int

func Frequency(s string) FreqMap {
	m := FreqMap{}
	for _, r := range s {
		m[r]++
	}
	return m
}

func frequencies(ss []string) <-chan FreqMap {
	out := make(chan FreqMap, len(ss))
	go func() {
		defer close(out)
		for _, s := range ss {
			out <- Frequency(s)
		}
	}()
	return out
}

func sum(in <-chan FreqMap) <-chan FreqMap {
	out := make(chan FreqMap, 1)
	go func() {
		defer close(out)
		res := FreqMap{}
		for f := range in {
			for k, v := range f {
				res[k] += v
			}
		}
		out <- res
	}()
	return out
}

func ConcurrentFrequency(ss []string) FreqMap {
	return <-sum(frequencies(ss))
}
