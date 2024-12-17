package day17

import (
	"fmt"
	"math"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func Run(input string) (string, int64) {
	// 	input = `Register A: 10
	// Register B: 0
	// Register C: 0

	// Program: 5,0,5,1,5,4`
	parse := func(a string) int64 {
		n, _ := strconv.ParseInt(a, 10, 64)
		return n
	}

	re, _ := regexp.Compile(`Register A: (\d+)\sRegister B: (\d+)\sRegister C: (\d+)\s+Program: (.*)`)
	matches := re.FindStringSubmatch(input)
	a := parse(matches[1])
	b := parse(matches[2])
	c := parse(matches[3])
	program := strings.Split(matches[4], ",")

	literal := func(op string) int64 {
		return parse(op)
	}

	run := func(program []string, a int64, b int64, c int64) string {
		combo := func(op string) int64 {
			switch op {
			case "0":
				return 0
			case "1":
				return 1
			case "2":
				return 2
			case "3":
				return 3
			case "4":
				return a
			case "5":
				return b
			case "6":
				return c
			default:
				return 0
			}
		}
		out := ""

		for i := 0; i < len(program); i += 2 {
			switch program[i] {
			case "0": // adv
				a = a / int64(math.Pow(2, float64(combo(program[i+1]))))
			case "1": // bxl (xor)
				b = b ^ literal(program[i+1])
			case "2": // bst (mod)
				b = combo(program[i+1]) % 8
			case "3": // jnz
				if a != 0 {
					i = int(literal(program[i+1])) - 2
				}
			case "4": // bxc
				b = b ^ c
			case "5": // out
				out += "," + strconv.FormatInt(combo(program[i+1])%8, 10)
			case "6": // bdv
				b = a / int64(math.Pow(2, float64(combo(program[i+1]))))
			case "7": // cdv
				c = a / int64(math.Pow(2, float64(combo(program[i+1]))))
			}
		}

		return out[1:]
	}

	comboAsm := func(op string) string {
		switch op {
		case "0":
			return "0"
		case "1":
			return "1"
		case "2":
			return "2"
		case "3":
			return "3"
		case "4":
			return "A"
		case "5":
			return "B"
		case "6":
			return "C"
		default:
			return "0"
		}
	}

	asm := func(program []string) {
		for i := 0; i < len(program); i += 2 {
			switch program[i] {
			case "0": // adv
				fmt.Printf("SET A TO A / 2^%s\n", comboAsm(program[i+1]))
			case "1": // bxl (xor)
				fmt.Printf("SET B TO B XOR %s\n", program[i+1])
			case "2": // bst (mod)
				fmt.Printf("SET B TO %s %% 8\n", comboAsm(program[i+1]))
			case "3": // jnz
				fmt.Printf("JUMP TO %s IF A != 0\n", program[i+1])
			case "4": // bxc
				fmt.Printf("SET B TO B XOR C\n")
			case "5": // out
				fmt.Printf("OUTPUT %s %% 8\n", comboAsm(program[i+1]))
			case "6": // bdv
				fmt.Printf("SET B TO A / 2^%s\n", comboAsm(program[i+1]))
			case "7": // cdv
				fmt.Printf("SET C TO A / 2^%s\n", comboAsm(program[i+1]))
			}
		}
	}

	asm(program)

	f, _ := os.ReadFile("day17/day17.txt")
	re, _ = regexp.Compile(`(?m)^(\d) \= (.*)$`)
	match := re.FindAllStringSubmatch(string(f), -1)
	patterns := make(map[int][]string)
	for _, m := range match {
		n, _ := strconv.Atoi(m[1])
		patterns[n^6] = append(patterns[n^6], m[2])
	}

	// tests for patterns
	for i := 0; i < 8; i++ {
		for _, p := range patterns[i] {
			p = strings.ReplaceAll(p, "_", "1")
			a, _ := strconv.ParseInt(p, 2, 64)
			o := run(program, a, b, c)
			if o[0:1] != strconv.FormatInt(int64(i), 10) {
				fmt.Printf("FAIL: %d %s %s\n", i, p, o)
			}
		}
	}

	patternMatch := func(a, b string) bool {
		for i := 0; i < len(b); i++ {
			if b[len(b)-i-1] != a[len(a)-i-1] && b[len(b)-i-1] != '_' {
				return false
			}
		}
		return true
	}

	answers := make([]string, 0)
	var rec func(int, string)
	rec = func(i int, ans string) {
		if i == -1 {
			answers = append(answers, ans)
			return
		}
		want := program[i]
		for _, p := range patterns[int(parse(want))] {
			// pattern like 110000 / 010____100 / 011___101 ...
			// _ matches any bit. Implied is ____s before the pattern.
			// successful match if end of answer matches pattern[:-3] (ignoring the last 3 bits)
			// if it does, append pattern's last 3 bits to answer
			if patternMatch(ans, p[:len(p)-3]) {
				rec(i-1, ans+p[len(p)-3:])
			}
		}
	}

	rec(len(program)-1, "00000000")
	// for _, p := range program {
	// 	n := parse(p)
	// 	fmt.Printf("%d ", n^6)
	// 	fmt.Println()
	// 	fmt.Println("")
	// }
	out1 := run(program, a, b, c)
	// "00000000-110-101-110-010-011-101-000-100-101-100-010-101-111-010-111-111"
	var min int64
	min = math.MaxInt64
	for _, ans := range answers {
		a, _ = strconv.ParseInt(ans, 2, 64)
		out2 := run(program, a, b, c)
		p2 := strings.Split(out2, ",")
		for i, c := range p2 {
			if i > 0 && program[i] != c {
				fmt.Printf("MISMATCH: %s %s\n", program[i], c)
				break
			}
		}
		if a < min {
			min = a
		}
	}

	return out1, min
}

// ANS:

// 110-001-001
// 110-001-011
// 110-001-101
// 110-101-011
// 110-101-101
// 110-101-110
