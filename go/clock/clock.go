package clock

import "fmt"

// Clock hour and minute
type Clock struct {
	Hour   int
	Minute int
}

func calcDiffHourAndFixedMinute(minute int) (diffHour, fixedMinute int) {
	if minute >= 0 {
		diffHour = minute / 60
		fixedMinute = minute % 60
	} else if (minute % -60) == 0 {
		diffHour = -(minute / -60)
		fixedMinute = 0
	} else {
		diffHour = -1 - (minute / -60)
		fixedMinute = 60 + minute%60
	}
	return
}

func calcFixedHour(hour, diffHour int) (fixedHour int) {
	fixedHour = hour + diffHour
	if fixedHour < 0 {
		fixedHour = 24 + fixedHour%24
	}
	return fixedHour % 24
}

// New is constructor
func New(hour, minute int) Clock {
	diffHour, fixedMinute := calcDiffHourAndFixedMinute(minute)
	fixedHour := calcFixedHour(hour, diffHour)
	return Clock{Hour: fixedHour, Minute: fixedMinute}
}

// String is stringer
func (c Clock) String() string {
	return fmt.Sprintf("%02d:%02d", c.Hour, c.Minute)
}

// Add is calculation
func (c Clock) Add(minutes int) Clock {
	return New(c.Hour, c.Minute+minutes)
}
