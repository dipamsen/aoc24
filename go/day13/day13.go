package day13

import (
	"regexp"
	"strconv"
	"strings"
)

func parse(s string) int64 {
	n, _ := strconv.ParseInt(s, 10, 64)
	return n
}

func Run(input string) (int64, int64) {
	machines := strings.Split(input, "\n\n")

	var ans1, ans2 int64

	for _, m := range machines {
		re, _ := regexp.Compile(`Button A: X\+(\d+), Y\+(\d+)\sButton B: X\+(\d+), Y\+(\d+)\sPrize: X=(\d+), Y=(\d+)`)
		matches := re.FindStringSubmatch(m)[1:]
		var x1, y1, x2, y2, x, y int64
		x1 = parse(matches[0])
		y1 = parse(matches[1])
		x2 = parse(matches[2])
		y2 = parse(matches[3])
		x = parse(matches[4])
		y = parse(matches[5])

		xm := x + 10000000000000
		ym := y + 10000000000000

		d := x1*y2 - x2*y1
		na := x*y2 - x2*y
		nb := x1*y - x*y1

		na2 := xm*y2 - x2*ym
		nb2 := x1*ym - xm*y1

		if d == 0 {
			panic("Found degenerate case")
		}
		if na%d == 0 && nb%d == 0 {
			t1 := na / d
			t2 := nb / d
			if t1 >= 0 && t2 >= 0 {
				ans1 += 3*t1 + t2
			}
		}
		if na2%d == 0 && nb2%d == 0 {
			t1 := na2 / d
			t2 := nb2 / d
			if t1 >= 0 && t2 >= 0 {
				ans2 += 3*t1 + t2
			}
		}

	}

	return ans1, ans2
}
