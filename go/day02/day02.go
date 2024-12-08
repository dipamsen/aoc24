package day02

import (
	"math"
	"strconv"
	"strings"
)

func checkGradualMonotonic(nums []int, ignore int) bool {
	if ignore == 0 {
		return checkGradualMonotonic(nums[1:], -1)
	}

	isInc := nums[1] > nums[0]
	isSafe := true

	if ignore == 1 {
		isInc = nums[2] > nums[0]
	}

	for i := 1; i < len(nums); i++ {
		curr := nums[i]
		prev := nums[i-1]

		if i == ignore {
			continue
		}

		if i == ignore+1 {
			prev = nums[i-2]
		}

		if isInc && curr < prev {
			isSafe = false
			break
		}
		if !isInc && curr > prev {
			isSafe = false
			break
		}

		diff := math.Abs(float64(curr - prev))
		if diff < 1 || diff > 3 {
			isSafe = false
			break
		}
	}

	return isSafe
}

func Run(input string) (int, int) {
	// input = `7 6 4 2 1
	// 1 2 7 8 9
	// 9 7 6 2 1
	// 1 3 2 4 5
	// 8 6 4 4 1
	// 1 3 6 7 9`

	lines := strings.Split(input, "\n")

	safeCount := 0

	allNums := make([][]int, 0)

	for _, line := range lines {
		nums := make([]int, 0)
		for _, num := range strings.Fields(line) {
			k, err := strconv.Atoi(num)
			if err != nil {
				panic(err)
			}
			nums = append(nums, k)
		}
		allNums = append(allNums, nums)

		isSafe := checkGradualMonotonic(nums, -1)

		if isSafe {
			safeCount++
		}
	}

	modSafeCount := 0

	for _, list := range allNums {
		isSafe := false
		for i := 0; i < len(list); i++ {
			if checkGradualMonotonic(list, i) {
				isSafe = true
				break
			}
		}
		if isSafe {
			modSafeCount++
		}
	}

	return safeCount, modSafeCount
}
