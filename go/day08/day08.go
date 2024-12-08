package day08

import "strings"

func Run(input string) (int, int) {

	type position struct {
		i int
		j int
	}

	antenna := make(map[rune][]position)

	for i, line := range strings.Split(input, "\n") {
		for j, char := range line {
			if char != '.' {
				antenna[char] = append(antenna[char], position{i, j})
			}
		}
	}

	pts1 := make(map[position]struct{})
	pts2 := make(map[position]struct{})

	withinBounds := func(p position) bool {
		return p.i >= 0 && p.i < len(strings.Split(input, "\n")) && p.j >= 0 && p.j < len(strings.Split(input, "\n")[0])
	}

	for _, locs := range antenna {
		for _, loc1 := range locs {
			for _, loc2 := range locs {
				if loc1 != loc2 {
					loc3 := position{
						loc1.i + (loc2.i-loc1.i)*2,
						loc1.j + (loc2.j-loc1.j)*2,
					}
					loc4 := position{
						loc2.i + (loc1.i-loc2.i)*2,
						loc2.j + (loc1.j-loc2.j)*2,
					}
					if withinBounds(loc3) {
						pts1[loc3] = struct{}{}
					}
					if withinBounds(loc4) {
						pts1[loc4] = struct{}{}
					}

					di := loc2.i - loc1.i
					dj := loc2.j - loc1.j
					loc := position{loc2.i, loc2.j}
					for withinBounds(loc) {
						pts2[loc] = struct{}{}
						loc = position{loc.i + di, loc.j + dj}
					}
					loc = position{loc1.i, loc1.j}
					for withinBounds(loc) {
						pts2[loc] = struct{}{}
						loc = position{loc.i - di, loc.j - dj}
					}
				}
			}
		}
	}

	return len(pts1), len(pts2)
}
