pub fn run(input: &str) -> (i64, i64) {
    let (part1, part2) = input.split("\n\n").fold((0, 0), |acc, m| {
        let lines = m.lines().collect::<Vec<_>>();
        let x1 = &lines[0][12..14].parse::<i64>().unwrap();
        let y1 = &lines[0][18..20].parse::<i64>().unwrap();
        let x2 = &lines[1][12..14].parse::<i64>().unwrap();
        let y2 = &lines[1][18..20].parse::<i64>().unwrap();
        let t = lines[2].split(&['=', ',']).collect::<Vec<_>>();
        let x = t[1].parse::<i64>().unwrap();
        let y = t[3].parse::<i64>().unwrap();
        let xm = x + 10000000000000;
        let ym = y + 10000000000000;
        let d: i64 = x1 * y2 - y1 * x2;
        let a: i64 = x * y2 - y * x2;
        let b: i64 = x1 * y - y1 * x;
        let am = xm * y2 - ym * x2;
        let bm = x1 * ym - y1 * xm;
        let mut p1 = 0;
        let mut p2 = 0;
        if a % d == 0 && b % d == 0 {
            let t1 = a / d;
            let t2 = b / d;
            if t1 >= 0 && t2 >= 0 {
                p1 = 3 * t1 + t2;
            }
        }
        if am % d == 0 && bm % d == 0 {
            let t1 = am / d;
            let t2 = bm / d;
            if t1 >= 0 && t2 >= 0 {
                p2 = 3 * t1 + t2;
            }
        }
        return (acc.0 + p1, acc.1 + p2);
    });
    return (part1, part2);
}
