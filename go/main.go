package main

import (
	"fmt"
	"os"
)

func main() {
	day := 2
	run := Day02
	filename := fmt.Sprintf("../inputs/day%02d.txt", day)

	file, err := os.ReadFile(filename)

	if err != nil {
		fmt.Println(err)
		return
	}

	input := string(file)

	a, b := run(input)

	fmt.Println("Day 01:", a, b)

}
