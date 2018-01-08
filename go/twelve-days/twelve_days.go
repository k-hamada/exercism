package twelve

type Songs struct {
	Number int
}

func (s Songs) Count() string {
	var t string
	switch s.Number {
	case 1:
		t = "first"
	case 2:
		t = "second"
	case 3:
		t = "third"
	case 4:
		t = "fourth"
	case 5:
		t = "fifth"
	case 6:
		t = "sixth"
	case 7:
		t = "seventh"
	case 8:
		t = "eighth"
	case 9:
		t = "ninth"
	case 10:
		t = "tenth"
	case 11:
		t = "eleventh"
	case 12:
		t = "twelfth"
	}
	return t
}

func (s Songs) Song() string {
	var t string
	for i := s.Number; 0 < i; i-- {
		switch i {
		case 1:
			t += "a Partridge in a Pear Tree."
		case 2:
			t += "two Turtle Doves, and "
		case 3:
			t += "three French Hens, "
		case 4:
			t += "four Calling Birds, "
		case 5:
			t += "five Gold Rings, "
		case 6:
			t += "six Geese-a-Laying, "
		case 7:
			t += "seven Swans-a-Swimming, "
		case 8:
			t += "eight Maids-a-Milking, "
		case 9:
			t += "nine Ladies Dancing, "
		case 10:
			t += "ten Lords-a-Leaping, "
		case 11:
			t += "eleven Pipers Piping, "
		case 12:
			t += "twelve Drummers Drumming, "
		}
	}
	return t
}

func Song() string {
	var s string
	for i := 1; i <= 12; i++ {
		s += Verse(i) + "\n"
	}
	return s
}

func Verse(i int) string {
	songs := Songs{Number: i}
	return "On the " + songs.Count() + " day of Christmas my true love gave to me, " + songs.Song()
}
