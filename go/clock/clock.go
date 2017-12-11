package clock

import "fmt"

// Clock have minutes
type Clock struct {
	minutes int
}

// MinutesPerDay 1 day == 24 hour * 60 minute
const MinutesPerDay = 24 * 60

// New is constructor
func New(hour, minute int) Clock {
	newMinute := hour*60 + minute
	for newMinute < 0 {
		newMinute += MinutesPerDay
	}
	newMinute %= MinutesPerDay
	return Clock{minutes: newMinute}
}

func (c Clock) hour() int {
	return c.minutes / 60 % 24
}

func (c Clock) minute() int {
	return c.minutes % 60
}

// String is stringer
func (c Clock) String() string {
	return fmt.Sprintf("%02d:%02d", c.hour(), c.minute())
}

// Add is calculation
func (c Clock) Add(minutes int) Clock {
	return New(c.hour(), c.minute()+minutes)
}
