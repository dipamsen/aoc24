package main

import (
	"strconv"
	"strings"
)

func Day07(input string) (int64, int64) {

	type Test struct {
		target int64
		nums   []int64
	}
	// 	input = `190: 10 19
	// 3267: 81 40 27
	// 83: 17 5
	// 156: 15 6
	// 7290: 6 8 6 15
	// 161011: 16 10 13
	// 192: 17 8 14
	// 21037: 9 7 18 13
	// 292: 11 6 16 20`

	parse := func(s string) int64 {
		n, _ := strconv.ParseInt(s, 10, 64)
		return n
	}

	var tests []Test
	for _, line := range strings.Split(input, "\n") {
		target, nums, _ := strings.Cut(line, ": ")
		test := Test{target: parse(target), nums: make([]int64, 0)}
		for _, num := range strings.Split(nums, " ") {
			test.nums = append(test.nums, parse(num))
		}
		tests = append(tests, test)
	}

	var rec func(sl []int64, ans, target int64) bool
	rec = func(sl []int64, ans, target int64) bool {
		if len(sl) == 0 {
			return ans == target
		}
		return rec(sl[1:], ans+sl[0], target) || rec(sl[1:], ans*sl[0], target)
	}

	concat := func(a, b int64) int64 {
		return parse(strconv.FormatInt(a, 10) + strconv.FormatInt(b, 10))
	}

	var rec2 func(sl []int64, ans, target int64) bool
	rec2 = func(sl []int64, ans, target int64) bool {
		if len(sl) == 0 {
			return ans == target
		}
		return rec2(sl[1:], ans+sl[0], target) || rec2(sl[1:], ans*sl[0], target) || rec2(sl[1:], concat(ans, sl[0]), target)
	}

	var part1, part2 int64
	for _, test := range tests {
		if rec(test.nums[1:], test.nums[0], test.target) {
			part1 += test.target
			part2 += test.target
		} else if rec2(test.nums[1:], test.nums[0], test.target) {
			part2 += test.target
		}
	}

	return part1, part2
}
