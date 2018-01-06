package tournament

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"sort"
	"strings"
)

type Teams []*Team

func (t Teams) Len() int {
	return len(t)
}

func (t Teams) Swap(i, j int) {
	t[i], t[j] = t[j], t[i]
}

func (t Teams) Less(i, j int) bool {
	if t[i].Points() == t[j].Points() {
		return t[i].Name < t[j].Name
	}
	return t[i].Points() > t[j].Points()
}

func (t Teams) String() string {
	var s string
	for _, v := range t {
		s += fmt.Sprint(v)
	}
	return s
}

type Header struct{}

func (h *Header) String() string {
	return fmt.Sprintf("%-30s | %2s | %2s | %2s | %2s | %2s\n", "Team", "MP", "W", "D", "L", "P")
}

type Team struct {
	Name                     string
	Played, Won, Drawn, Lost int
}

func (t *Team) Play() {
	t.Played++
}

func (t *Team) Win() {
	t.Won++
}

func (t *Team) Lose() {
	t.Lost++
}

func (t *Team) Draw() {
	t.Drawn++
}

func (t *Team) Points() int {
	return t.Won*3 + t.Drawn*1
}

func (t *Team) String() string {
	return fmt.Sprintf("%-30s | %2d | %2d | %2d | %2d | %2d\n", t.Name, t.Played, t.Won, t.Drawn, t.Lost, t.Points())
}

var teams = make(map[string]*Team)

func clear() {
	teams = make(map[string]*Team)
}

func Tally(in io.Reader, out io.Writer) error {
	defer clear()

	reader := bufio.NewReader(in)
	for {
		line, _, err := reader.ReadLine()
		if err != nil {
			break
		}
		if len(line) == 0 {
			continue
		}

		parse := strings.Split(string(line), ";")
		if len(parse) != 3 {
			continue
		}

		apply0, apply1, err := GetAcrion(parse[2])
		if err != nil {
			continue
		}
		PlayedTeam(parse[0], apply0)
		PlayedTeam(parse[1], apply1)
	}

	if (len(teams)) == 0 {
		return errors.New("empty")
	}

	fmt.Fprint(out, new(Header))

	results := Teams{}
	for _, v := range teams {
		results = append(results, v)
	}
	sort.Sort(results)
	fmt.Fprint(out, results)

	return nil
}

func GetAcrion(result string) (apply0, apply1 func(*Team), err error) {
	switch result {
	case "win":
		apply0 = func(t *Team) { t.Win() }
		apply1 = func(t *Team) { t.Lose() }
	case "loss":
		apply0 = func(t *Team) { t.Lose() }
		apply1 = func(t *Team) { t.Win() }
	case "draw":
		apply0 = func(t *Team) { t.Draw() }
		apply1 = func(t *Team) { t.Draw() }
	default:
		err = errors.New(result)
	}
	return
}

func PlayedTeam(name string, fn func(*Team)) {
	team, ok := teams[name]
	if ok == true {
		team.Play()
		fn(team)
	} else {
		newTeam := &Team{Name: name}
		teams[name] = newTeam
		newTeam.Play()
		fn(newTeam)
	}
}
