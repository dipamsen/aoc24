package day16

import (
	"math"
	"strings"
)

type Point struct {
	i, j int
}

// Direction
//  0: up
//  1: right
//  2: down
//  3: left

// / DFS with backtracking
func dfs(grid []string, visited map[Point]bool, start, end Point, score int, dirn int) int {
	canGo := func(p Point) bool {
		return p.i >= 0 && p.i < len(grid) && p.j >= 0 && p.j < len(grid[0]) && grid[p.i][p.j] != '#'
	}

	if start == end {
		return score
	}

	dirns := []Point{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}

	visited[start] = true

	// options: go in same direction, turn left, turn right

	var a, b, c int
	a = math.MaxInt
	b = math.MaxInt
	c = math.MaxInt

	forward := Point{start.i + dirns[dirn].i, start.j + dirns[dirn].j}
	if canGo(forward) && !visited[forward] {
		a = dfs(grid, visited, forward, end, score+1, dirn)
	}

	left := Point{start.i + dirns[(dirn+3)%4].i, start.j + dirns[(dirn+3)%4].j}
	if canGo(left) && !visited[left] {
		b = dfs(grid, visited, left, end, score+1001, (dirn+3)%4)
	}

	right := Point{start.i + dirns[(dirn+1)%4].i, start.j + dirns[(dirn+1)%4].j}
	if canGo(right) && !visited[right] {
		c = dfs(grid, visited, right, end, score+1001, (dirn+1)%4)
	}

	delete(visited, start)

	return min(a, b, c)
}

func Run(input string) (int, int) {
	// 	input = `#################
	// #...#...#...#..E#
	// #.#.#.#.#.#.#.#.#
	// #.#.#.#...#...#.#
	// #.#.#.#.###.#.#.#
	// #...#.#.#.....#.#
	// #.#.#.#.#.#####.#
	// #.#...#.#.#.....#
	// #.#.#####.#.###.#
	// #.#.#.......#...#
	// #.#.###.#####.###
	// #.#.#...#.....#.#
	// #.#.#.#####.###.#
	// #.#.#.........#.#
	// #.#.#.#########.#
	// #S#.............#
	// #################`

	grid := strings.Split(input, "\n")

	var start, end Point
	visited := make(map[Point]bool)

	for i, row := range grid {
		for j, cell := range row {
			if cell == 'S' {
				start = Point{i, j}
			} else if cell == 'E' {
				end = Point{i, j}
			}
		}
	}

	return dfs(grid, visited, start, end, 0, 1), 0
}
