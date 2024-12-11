fn evolve(n: i64) -> Vec<i64> {
    let mut new_nums = vec![];
    let charlen = (f64::floor(f64::log10(n as f64).floor()) as i32) + 1;
    if n == 0 {
        new_nums.push(1);
    } else if charlen % 2 == 0 {
        new_nums.push(n / f64::powi(10f64, charlen / 2) as i64);
        new_nums.push(n % f64::powi(10f64, charlen / 2) as i64);
    } else {
        new_nums.push(n * 2024);
    }
    new_nums
}

pub fn run(input: &str) -> (i64, i64) {
    let nums: Vec<i64> = input
        .split_ascii_whitespace()
        .map(|l| l.parse().unwrap())
        .collect();

    let mut map = std::collections::HashMap::new();
    for num in nums.iter() {
        map.insert(*num, 1);
    }

    for _ in 0..25 {
        let mut new_nums = std::collections::HashMap::new();
        for (v, c) in map.iter() {
            for n in evolve(*v).iter() {
                let count = new_nums.entry(*n).or_insert(0);
                *count += c;
            }
        }
        map = new_nums;
    }

    let part1 = map.iter().map(|(_, c)| c).sum::<i64>();

    for _ in 0..50 {
        let mut new_nums = std::collections::HashMap::new();
        for (v, c) in map.iter() {
            for n in evolve(*v).iter() {
                let count = new_nums.entry(*n).or_insert(0);
                *count += c;
            }
        }
        map = new_nums;
    }
    let part2 = map.iter().map(|(_, c)| c).sum::<i64>();

    (part1, part2)
}
