package day12

import (
	"sort"
	"strings"
)

type Pos struct {
	i, j int
}

type Fence struct {
	i, j  int
	align int // 0: right, 1: down, 2: left, 3: up
}

type FenceType struct {
	coord int
	align int
}

func (f Fence) getType() (FenceType, int) {
	if f.align == 0 { // right
		return FenceType{f.j, 0}, f.i
	} else if f.align == 1 { // down
		return FenceType{f.i, 1}, f.j
	} else if f.align == 2 { // left
		return FenceType{f.j, 2}, f.i
	} else { // up
		return FenceType{f.i, 3}, f.j
	}
}

func Run(input string) (int, int) {
	// 	input = `RRRRIICCFF
	// RRRRIICCCF
	// VVRRRCCFFF
	// VVRCCCJFFF
	// VVVVCJJCFE
	// VVIVCCJJEE
	// VVIIICJJEE
	// MIIIIIJJEE
	// MIIISIJEEE
	// MMMISSJEEE`

	// 	input = `AAAAAA
	// AAABBA
	// AAABBA
	// ABBAAA
	// ABBAAA
	// AAAAAA`

	garden := make([][]rune, 0)
	for _, line := range strings.Fields(input) {
		garden = append(garden, []rune(line))
	}
	m := len(garden)
	n := len(garden[0])

	visited := make([][]bool, m)
	for i := 0; i < m; i++ {
		visited[i] = make([]bool, n)
	}

	var dfs func(i, j int) (int, int, []Fence)
	dfs = func(i, j int) (int, int, []Fence) {
		dirns := []Pos{{0, 1}, {1, 0}, {0, -1}, {-1, 0}}
		visited[i][j] = true

		fences := []Fence{}
		sides := 4
		count := 1

		createFence := func(i, j int, d Pos) {
			if d == dirns[0] {
				fences = append(fences, Fence{i, j, 0})
			} else if d == dirns[1] {
				fences = append(fences, Fence{i, j, 1})
			} else if d == dirns[2] {
				fences = append(fences, Fence{i, j, 2})
			} else if d == dirns[3] {
				fences = append(fences, Fence{i, j, 3})
			}
		}

		for _, d := range dirns {
			if i+d.i >= 0 && i+d.i < m && j+d.j >= 0 && j+d.j < n && garden[i+d.i][j+d.j] == garden[i][j] {
				if !visited[i+d.i][j+d.j] {
					a, p, f := dfs(i+d.i, j+d.j)
					sides += p
					count += a
					fences = append(fences, f...)
				}
				sides--
			} else {
				createFence(i, j, d)
			}
		}

		return count, sides, fences
	}

	sum := 0
	sum2 := 0

	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if !visited[i][j] {
				a, p, fences := dfs(i, j)
				sum += a * p

				seen := map[FenceType]map[int]bool{}
				count := 0

				for _, f := range fences {
					ft, c := f.getType()

					if seen[ft] == nil {
						seen[ft] = map[int]bool{}
						seen[ft][c] = true
					} else {
						if seen[ft][c-1] || seen[ft][c+1] {
							// part of same fence
							seen[ft][c] = true
						} else {
							seen[ft][c] = true
						}
					}
				}

				for _, v := range seen {
					tcs := []int{}
					for tc, _ := range v {
						tcs = append(tcs, tc)
					}
					sort.Slice(tcs, func(i2, j2 int) bool {
						return tcs[i2] < tcs[j2]
					})
					prev := -100
					for _, tc := range tcs {
						if tc > prev+1 {
							count++
						}
						prev = tc
					}

				}

				sum2 += count * a

			}
		}
	}

	return sum, sum2
}
