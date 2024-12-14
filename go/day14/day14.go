package day14

import (
	"regexp"
	"strconv"
	"strings"
)

type Robot struct {
	posX int
	posY int
	velX int
	velY int
}

func find(robots []Robot, posX, posY int) bool {
	for _, r := range robots {
		if r.posX == posX && r.posY == posY {
			return true
		}
	}
	return false
}

func Run(input string) (int, int) {
	parse := func(a string) int {
		n, _ := strconv.Atoi(a)
		return n
	}

	w := 101
	h := 103

	rem := func(a int, b int) int {
		return (a%b + b) % b
	}

	lines := strings.Split(input, "\n")
	var q1, q2, q3, q4 int
	robots := make([]Robot, len(lines))
	for i, line := range lines {
		re, _ := regexp.Compile(`p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)`)
		match := re.FindStringSubmatch(line)[1:]
		px, py, vx, vy := parse(match[0]), parse(match[1]), parse(match[2]), parse(match[3])
		robots[i] = Robot{posX: px, posY: py, velX: vx, velY: vy}

		// after 100 steps:
		nx := rem(px+100*vx, w)
		ny := rem(py+100*vy, h)

		if nx < w/2 && ny < h/2 {
			q1++
		} else if nx > w/2 && ny < h/2 {
			q2++
		} else if nx < w/2 && ny > h/2 {
			q3++
		} else if nx > w/2 && ny > h/2 {
			q4++
		}
	}

	i := 0
	for true {
		i++
		for j := 0; j < len(robots); j++ {
			robots[j].posX += robots[j].velX
			robots[j].posY += robots[j].velY
			robots[j].posX = rem(robots[j].posX, w)
			robots[j].posY = rem(robots[j].posY, h)
		}
		// check if 10 robots in a row
		found := true
		for j := 0; j < len(robots); j++ {
			px := robots[j].posX
			py := robots[j].posY

			found = true
			for k := 1; k < 10; k++ {
				if !find(robots, px+k, py) {
					found = false
					break
				}
			}
			if found {
				break
			}
		}
		if found {
			break
		}
	}

	return q1 * q2 * q3 * q4, i
}
