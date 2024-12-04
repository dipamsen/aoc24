package main

import (
	"strings"
)

func Day04(input string) (int, int) {

	// 	input = `MMMSXXMASM
	// MSAMXMSMSA
	// AMXSXMAAMM
	// MSAMASMSMX
	// XMASAMXAMM
	// XXAMMXXAMA
	// SMSMSASXSS
	// SAXAMASAAA
	// MAMMMXMMMM
	// MXMXAXMASX`
	count := 0
	lines := strings.Split(input, "\n")
	for i, line := range lines {
		for j := range line {
			if j+3 < len(line) && (line[j:j+4] == "XMAS" || line[j:j+4] == "SAMX") {
				count++
			}

			if i+3 < len(lines) &&
				((lines[i][j] == 'X' && lines[i+1][j] == 'M' && lines[i+2][j] == 'A' && lines[i+3][j] == 'S') ||
					(lines[i][j] == 'S' && lines[i+1][j] == 'A' && lines[i+2][j] == 'M' && lines[i+3][j] == 'X')) {
				count++
			}

			if i+3 < len(lines) && j+3 < len(line) &&
				((lines[i][j] == 'X' && lines[i+1][j+1] == 'M' && lines[i+2][j+2] == 'A' && lines[i+3][j+3] == 'S') ||
					(lines[i][j] == 'S' && lines[i+1][j+1] == 'A' && lines[i+2][j+2] == 'M' && lines[i+3][j+3] == 'X')) {
				count++
			}

			if i+3 < len(lines) && j-3 >= 0 &&
				((lines[i][j] == 'X' && lines[i+1][j-1] == 'M' && lines[i+2][j-2] == 'A' && lines[i+3][j-3] == 'S') ||
					(lines[i][j] == 'S' && lines[i+1][j-1] == 'A' && lines[i+2][j-2] == 'M' && lines[i+3][j-3] == 'X')) {
				count++
			}
		}
	}

	count2 := 0

	for i, line := range lines {
		for j := range line {
			if i+2 < len(lines) && j+2 < len(line) && lines[i+1][j+1] == 'A' {
				isXMas1 := (lines[i][j] == 'M' && lines[i+2][j+2] == 'S') || (lines[i][j] == 'S' && lines[i+2][j+2] == 'M')
				isXMas2 := (lines[i][j+2] == 'M' && lines[i+2][j] == 'S') || (lines[i][j+2] == 'S' && lines[i+2][j] == 'M')

				if isXMas1 && isXMas2 {
					count2++
				}

			}
		}
	}
	return count, count2
}
