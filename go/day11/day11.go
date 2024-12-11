package day11

import (
	"math"
	"strconv"
	"strings"
)

func numchars(i int) int {
	return int(math.Log10(float64(i))) + 1
}

type Num struct {
	n     int
	count int
}

func evolve(n int) map[int]int {
	ans := make(map[int]int, 0)
	if n == 0 {
		ans[1]++
	} else if numchars(n)%2 == 0 {
		ans[n/int(math.Pow10(numchars(n)/2))]++
		ans[n%int(math.Pow10(numchars(n)/2))]++
	} else {
		ans[n*2024]++
	}

	return ans
}

func Run(input string) (int, int) {
	nums := make(map[int]int, 0)
	for _, w := range strings.Fields(input) {
		n, _ := strconv.Atoi(w)
		nums[n]++
	}

	ans1 := 0
	ans2 := 0
	for i := 0; i < 75; i++ {
		newNums := make(map[int]int, 0)
		for v, c := range nums {
			nexts := evolve(v)
			for n, c2 := range nexts {
				newNums[n] += c * c2
			}
		}
		nums = newNums
		if i == 49 {
			for _, c := range nums {
				ans1 += c
			}
		}
	}
	for _, c := range nums {
		ans2 += c
	}
	return ans1, ans2
}
