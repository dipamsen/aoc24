pub fn run(input: &str) -> (i32, i32) {
    let (rules, queries) = input.split_once("\n\n").unwrap();

    let rules = rules.lines().map(|l| {
        match l
            .split("|")
            .map(|n| n.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>()
            .as_slice()
        {
            [a, b] => (*a, *b),
            _ => unreachable!(),
        }
    });

    // Assumption: There are n * (n + 1) / 2 rules, where n is the number of distinct numbers used.
    // There exists a rule for each pair of distinct numbers, which specifies their order.

    let queries = queries
        .lines()
        .map(|l| {
            l.split(",")
                .map(|n| n.trim().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // To check if a query is valid, it is sufficient to check if every two consecutive numbers
    // in the query are in the correct order according to the rules. (under the above assumption.)

    // Want: constant time lookup for order of any two numbers.
    // Possible soln: map (i32, i32) -> i32. Creation: O(n^2).

    let mut order: std::collections::HashMap<(i32, i32), bool> = std::collections::HashMap::new();
    for rule in rules {
        order.insert((rule.0, rule.1), true);
        order.insert((rule.1, rule.0), false);
    }

    let is_valid = |query: &[i32]| query.windows(2).all(|w| order[&(w[0], w[1])]);

    let ans1: i32 = queries
        .iter()
        .filter(|q| is_valid(q))
        .map(|q| q[q.len() / 2])
        .sum();

    let ans2: i32 = queries
        .iter()
        .filter(|q| !is_valid(q))
        .map(|q| {
            let mut sorted = q.to_owned();
            sorted.sort_by(|a, b| {
                if *a == *b {
                    std::cmp::Ordering::Equal
                } else if order[&(*a, *b)] {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });
            sorted[q.len() / 2]
        })
        .sum();

    (ans1, ans2)
}
