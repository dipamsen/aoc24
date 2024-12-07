pub fn run(input: &str) -> (i64, i64) {
    //     let input = "190: 10 19
    // 3267: 81 40 27
    // 83: 17 5
    // 156: 15 6
    // 7290: 6 8 6 15
    // 161011: 16 10 13
    // 192: 17 8 14
    // 21037: 9 7 18 13
    // 292: 11 6 16 20";

    let tests = input
        .lines()
        .map(|line| {
            line.split_once(": ")
                .map(|(a, b)| {
                    (
                        a.parse::<i64>().unwrap(),
                        b.split_ascii_whitespace()
                            .map(|x| x.trim().parse::<i64>().unwrap())
                            .collect::<Vec<_>>(),
                    )
                })
                .unwrap()
        })
        .collect::<Vec<_>>();

    fn rec(sl: &[i64], ans: i64, target: i64) -> bool {
        if sl.is_empty() {
            return ans == target;
        }
        if ans > target {
            return false;
        }
        rec(&sl[1..], ans + sl[0], target) || rec(&sl[1..], ans * sl[0], target)
    }

    let part1 = tests
        .iter()
        .filter(|(target, sl)| rec(&sl[1..], sl[0], *target))
        .map(|x| x.0)
        .sum::<i64>();

    fn cnct(a: i64, b: i64) -> i64 {
        format!("{}{}", a, b).parse::<i64>().unwrap()
    }

    fn rec2(sl: &[i64], ans: i64, target: i64) -> bool {
        if sl.is_empty() {
            return ans == target;
        }
        if ans > target {
            return false;
        }
        rec2(&sl[1..], ans + sl[0], target)
            || rec2(&sl[1..], ans * sl[0], target)
            || rec2(&sl[1..], cnct(ans, sl[0]), target)
    }

    let part2 = tests
        .iter()
        .filter(|(target, sl)| rec2(&sl[1..], sl[0], *target))
        .map(|x| x.0)
        .sum::<i64>();

    (part1, part2)
}
