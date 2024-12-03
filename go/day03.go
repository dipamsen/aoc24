package main

import (
	"regexp"
	"strconv"
)

func Day03(input string) (int, int) {
	re := regexp.MustCompile(`mul\((\d+),(\d+)\)`)
	matches := re.FindAllStringSubmatch(input, -1)
	indices := re.FindAllStringIndex(input, -1)
	prod := make([]int, 0)

	sum := 0
	for _, match := range matches {
		a, _ := strconv.Atoi(match[1])
		b, _ := strconv.Atoi(match[2])
		sum += a * b
		prod = append(prod, a*b)
	}

	doR := regexp.MustCompile(`do\(\)`)
	dontR := regexp.MustCompile(`don't\(\)`)

	doInd := doR.FindAllStringIndex(input, -1)
	dontInd := dontR.FindAllStringIndex(input, -1)

	sum2 := 0

	j := 0
	k := 0
	l := 0
	do := true
	for i := range input {
		if j < len(indices) && i == indices[j][0] {
			if do {
				sum2 += prod[j]
			}
			j++
		}

		if k < len(doInd) && i == doInd[k][0] {
			do = true
			k++
		}

		if l < len(dontInd) && i == dontInd[l][0] {
			do = false
			l++
		}
	}

	return sum, sum2
}
