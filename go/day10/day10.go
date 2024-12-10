package day10

import (
	"strings"
)

type Pos struct {
	i, j int
}

func Run(input string) (int, int) {
	// 	input = `89010123
	// 78121874
	// 87430965
	// 96549874
	// 45678903
	// 32019012
	// 01329801
	// 10456732`
	trailheads := make([]Pos, 0)
	grid := strings.Split(input, "\n")
	for i, row := range grid {
		for j, cell := range row {
			if cell == '0' {
				trailheads = append(trailheads, Pos{i, j})
			}
		}
	}

	var countScore func(grid []string, i, j int) map[Pos]int
	countScore = func(grid []string, i, j int) map[Pos]int {
		curr := grid[i][j]
		next := curr + 1

		peaks := make(map[Pos]int)

		if curr == '9' {
			peaks[Pos{i, j}]++
			return peaks
		}

		// up
		if i > 0 && grid[i-1][j] == next {
			newPeaks := countScore(grid, i-1, j)
			for peak := range newPeaks {
				peaks[peak] += newPeaks[peak]
			}
		}
		// right
		if j < len(grid[0])-1 && grid[i][j+1] == next {
			newPeaks := countScore(grid, i, j+1)
			for peak := range newPeaks {
				peaks[peak] += newPeaks[peak]
			}
		}
		// down
		if i < len(grid)-1 && grid[i+1][j] == next {
			newPeaks := countScore(grid, i+1, j)
			for peak := range newPeaks {
				peaks[peak] += newPeaks[peak]
			}
		}
		// left
		if j > 0 && grid[i][j-1] == next {
			newPeaks := countScore(grid, i, j-1)
			for peak := range newPeaks {
				peaks[peak] += newPeaks[peak]
			}
		}
		return peaks
	}

	score := 0
	score2 := 0
	for _, pos := range trailheads {
		peaks := countScore(grid, pos.i, pos.j)
		s := len(peaks)
		score += s
		for _, v := range peaks {
			score2 += v
		}
	}

	return score, score2
}
