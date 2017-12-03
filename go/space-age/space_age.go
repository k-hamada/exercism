package space

const EarthSeconds = 31557600

type Planet string

var planets = map[Planet]float64{
	"Earth":   EarthSeconds,
	"Mercury": 0.2408467 * EarthSeconds,
	"Venus":   0.6151972 * EarthSeconds,
	"Mars":    1.8808158 * EarthSeconds,
	"Jupiter": 11.862615 * EarthSeconds,
	"Saturn":  29.447498 * EarthSeconds,
	"Uranus":  84.016846 * EarthSeconds,
	"Neptune": 164.79132 * EarthSeconds,
}

func Age(seconds float64, planet Planet) float64 {
	v, ok := planets[planet]
	if ok == true {
		return seconds / v
	}
	return 0.0
}
