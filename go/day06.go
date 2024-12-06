package main

import (
	"strings"
)

type position struct {
	i int
	j int
}

type state struct {
	p position
	d int
}

func move(pos position, dir int) position {
	if dir == 0 {
		return position{pos.i - 1, pos.j}
	} else if dir == 1 {
		return position{pos.i, pos.j + 1}
	} else if dir == 2 {
		return position{pos.i + 1, pos.j}
	} else if dir == 3 {
		return position{pos.i, pos.j - 1}
	}
	return position{0, 0}
}

func Day06(input string) (int, int) {
	// 	input = `....#.....
	// .........#
	// ..........
	// ..#.......
	// .......#..
	// ..........
	// .#..^.....
	// ........#.
	// #.........
	// ......#...`

	var grid [][]int // stores obstacles
	var pos position
	var init position
	dir := 0 // 0: up, 1: right, 2: down, 3: left

	for i, line := range strings.Split(input, "\n") {
		row := make([]int, len(line))
		for j, char := range line {
			if char == '#' {
				row[j] = 1
			}
			if char == '^' {
				init = position{i, j}
			}
		}
		grid = append(grid, row)
	}

	pos = position{init.i, init.j}

	visited := make(map[position]bool)
	visited[pos] = true

	for {
		new := move(pos, dir)
		if new.i < len(grid) && new.i >= 0 && new.j < len(grid[0]) && new.j >= 0 {
			if grid[new.i][new.j] == 1 {
				dir = (dir + 1) % 4
			} else {
				pos = new
				visited[pos] = true
			}
		} else {
			break
		}
	}

	ans1 := len(visited)
	ans2 := 0

	for i := range grid {
		for j := range grid[i] {
			if grid[i][j] == 0 && (i != init.i || j != init.j) {
				grid[i][j] = 1

				pos = position{init.i, init.j}
				dir = 0
				visitedDir := make(map[state]bool)
				visitedDir[state{pos, dir}] = true
				isLoop := false
				for {
					new := move(pos, dir)
					if new.i < len(grid) && new.i >= 0 && new.j < len(grid[0]) && new.j >= 0 {
						if grid[new.i][new.j] == 1 {
							dir = (dir + 1) % 4
						} else {
							pos = new
						}
						if visitedDir[state{pos, dir}] {
							isLoop = true
							break
						}
						visitedDir[state{pos, dir}] = true
					} else {
						break
					}
				}
				if isLoop {
					ans2 += 1
				}

				grid[i][j] = 0
			}
		}
	}

	return ans1, ans2
}
