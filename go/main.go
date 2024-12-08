package main

import (
	"aoc24/day01"
	"aoc24/day02"
	"aoc24/day03"
	"aoc24/day04"
	"aoc24/day05"
	"aoc24/day06"
	"aoc24/day07"
	"aoc24/day08"

	/* insert-import */
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	day, _ := strconv.Atoi(os.Args[len(os.Args)-1])
	filename := fmt.Sprintf("../inputs/day%02d.txt", day)

	file, err := os.ReadFile(filename)

	if err != nil {
		fmt.Println(err)
		return
	}

	input := string(file)
	input = strings.Trim(input, "\n")
	input = strings.ReplaceAll(input, "\r\n", "\n")

	fmt.Print("Day ", day, ": ")

	switch day {
	case 1:
		fmt.Println(day01.Run(input))
	case 2:
		fmt.Println(day02.Run(input))
	case 3:
		fmt.Println(day03.Run(input))
	case 4:
		fmt.Println(day04.Run(input))
	case 5:
		fmt.Println(day05.Run(input))
	case 6:
		fmt.Println(day06.Run(input))
	case 7:
		fmt.Println(day07.Run(input))
	case 8:
		fmt.Println(day08.Run(input))
	/* insert-case */
	default:
		fmt.Println("Day not implemented")
	}
}
