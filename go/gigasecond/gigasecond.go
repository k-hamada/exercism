package gigasecond

import "time"

// AddGigasecond Add 10^9 (1,000,000,000) seconds
func AddGigasecond(t time.Time) time.Time {
	a := t.Add(1000000000 * time.Second)
	return a
}
