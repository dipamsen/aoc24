package day05

import (
	"slices"
	"sort"
	"strconv"
	"strings"
)

func unwrap[T any](value T, _ error) T {
	return value
}

func followRule(query []int, rule [2]int) bool { // O(m) (each query has m rules)
	a := slices.Index(query, rule[0])
	b := slices.Index(query, rule[1])

	if a == -1 || b == -1 {
		return true
	}
	if a < b {
		return true
	}
	return false
}

func Run(input string) (int, int) {
	// 	input = `47|53
	// 97|13
	// 97|61
	// 97|47
	// 75|29
	// 61|13
	// 75|53
	// 29|13
	// 97|29
	// 53|29
	// 61|53
	// 97|53
	// 61|29
	// 47|13
	// 75|47
	// 97|75
	// 47|61
	// 75|61
	// 47|29
	// 75|13
	// 53|13

	// 75,47,61,53,29
	// 97,61,53,29,13
	// 75,29,13
	// 75,97,47,61,53
	// 61,13,29
	// 97,13,75,29,47`

	x := strings.Split(input, "\n\n")
	rulesTxt := strings.Split(x[0], "\n")
	queriesTxt := strings.Split(x[1], "\n")

	rules := make([][2]int, 0)
	for _, rule := range rulesTxt {
		x := strings.Split(rule, "|")
		rules = append(rules, [2]int{
			unwrap(strconv.Atoi(x[0])),
			unwrap(strconv.Atoi(x[1])),
		})
	}

	queries := make([][]int, 0)
	for _, q := range queriesTxt {
		x := strings.Split(q, ",")
		query := make([]int, 0)
		for _, y := range x {
			query = append(query, unwrap(strconv.Atoi(y)))
		}
		queries = append(queries, query)
	}

	ans := 0
	ans2 := 0

	for _, query := range queries { // n
		follows := true
		for _, rule := range rules { // k
			if !followRule(query, rule) { // m
				follows = false
				break
			}
		}
		if follows {
			ans += query[len(query)/2]
		} else {
			// O(n log n * m)
			sort.Slice(query, func(i, j int) bool { // i before j, if:
				for _, rule := range rules {
					if rule[0] == query[i] && rule[1] == query[j] {
						return true
					}
				}
				return false
			})
			ans2 += query[len(query)/2]
		}
	}

	// overall ~ O(n^2 log n * m^2 * k)

	return ans, ans2
}
