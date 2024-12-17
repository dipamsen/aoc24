package day15

import (
	"strings"
)

type Box struct {
	i int
	j int
}

type Wall struct {
	i int
	j int
}

func Run(input string) (int, int) {

	p := strings.Split(input, "\n\n")
	lines := p[0]
	steps := strings.Split(p[1], "\n")

	boxes := make([]Box, 0)
	walls := make([]Wall, 0)
	var robot Box

	for i, line := range strings.Split(lines, "\n") {
		for j, c := range line {
			if c == '#' {
				walls = append(walls, Wall{i, j})
			} else if c == 'O' {
				boxes = append(boxes, Box{i, j})
			} else if c == '@' {
				robot = Box{i, j}
			}
		}
	}

	for _, row := range steps {
		for _, step := range row {
			if step == '^' {
				// find all boxes connected on top of robot
				i := robot.i - 1
				j := robot.j
				for {
					for _, box := range boxes {
						if box.i == i && box.j == j {
							// move box
							box.i--
						}
					}
					i--
				}
			} else if step == 'v' {
				// move down
			} else if step == '<' {
				// move left
			} else if step == '>' {
				// move right
			}
		}
	}

	return 0, 0
}
