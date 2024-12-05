package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	day := 5
	run := Day05
	filename := fmt.Sprintf("../inputs/day%02d.txt", day)

	file, err := os.ReadFile(filename)

	if err != nil {
		fmt.Println(err)
		return
	}

	input := string(file)
	input = strings.ReplaceAll(input, "\r\n", "\n")

	a, b := run(input)

	fmt.Println("Day", day, ":", a, b)

}
