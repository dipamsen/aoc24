package main

import (
	"math"
	"sort"
	"strconv"
	"strings"
)

func Day01(input string) (int, int) {
	lines := strings.Split(input, "\n")
	list1 := make([]int, len(lines))
	list2 := make([]int, len(lines))
	for _, line := range lines {
		nums := strings.Fields(line)
		a, err1 := strconv.Atoi(nums[0])
		b, err2 := strconv.Atoi(nums[1])

		if err1 != nil || err2 != nil {
			continue
		}

		list1 = append(list1, a)
		list2 = append(list2, b)
	}

	sort.Slice(list1, func(i, j int) bool {
		return list1[i] < list1[j]
	})

	sort.Slice(list2, func(i, j int) bool {
		return list2[i] < list2[j]
	})

	sum := 0

	for i := range list1 {
		sum += int(math.Abs(float64(list1[i] - list2[i])))
	}

	sim := 0

	j := 0

	for _, num := range list1 {
		for j < len(list2) && list2[j] < num {
			j++
		}

		for j < len(list2) && list2[j] == num {
			sim += num
			j++
		}
	}

	return sum, sim
}
