package robotname

import (
	crand "crypto/rand"
	"encoding/binary"
	"math/rand"
)

const alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
const number = "0123456789"

var set = make(map[string]struct{})

type Robot struct {
	rand *rand.Rand
	name string
}

func (robot *Robot) Name() string {
	if robot.rand == nil {
		robot.Reset()
	}
	return robot.name
}

func (robot *Robot) Reset() {
again:
	robot.rand = rand.New(rand.NewSource(randSeed()))
	robot.name = robot.randomStr(2, alphabet) + robot.randomStr(3, number)

	if _, exist := set[robot.name]; exist {
		goto again
	}
	set[robot.name] = struct{}{}
}

func (robot *Robot) randomStr(n int, charset string) string {
	b := make([]byte, n)
	for i := range b {
		b[i] = charset[robot.rand.Intn(len(charset))]
	}
	return string(b)
}

func randSeed() int64 {
	var seed int64
	err := binary.Read(crand.Reader, binary.LittleEndian, &seed)
	if err != nil {
		panic(err)
	}
	return seed
}
