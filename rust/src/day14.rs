pub fn run(input: &str) -> (i32, i32) {
    //     let input = "p=0,4 v=3,-3
    // p=6,3 v=-1,-3
    // p=10,3 v=-1,2
    // p=2,0 v=2,-1
    // p=0,0 v=1,3
    // p=3,0 v=-2,-2
    // p=7,6 v=-1,-3
    // p=3,0 v=-1,-2
    // p=9,3 v=2,3
    // p=7,3 v=-1,2
    // p=2,4 v=2,-3
    // p=9,5 v=-3,-3";

    let rem = |a, b| (a % b + b) % b;

    let xw = 101;
    let yw = 103;
    let mut poss = input
        .lines()
        .map(|l| {
            let t = l.split(&['=', ',', ' ']).collect::<Vec<_>>();
            let px = t[1].parse::<i32>().unwrap();
            let py = t[2].parse::<i32>().unwrap();
            let vx = t[4].parse::<i32>().unwrap();
            let vy = t[5].parse::<i32>().unwrap();
            (px, py, vx, vy)
        })
        .collect::<Vec<_>>();
    let step100 = poss
        .iter()
        .map(|(px, py, vx, vy)| (rem(px + 100 * vx, xw), rem(py + 100 * vy, yw)))
        .collect::<Vec<_>>();

    let step =
        |(px, py, vx, vy): (i32, i32, i32, i32)| (rem(px + vx, xw), rem(py + vy, yw), vx, vy);

    let xc = xw / 2;
    let yc = yw / 2;

    let q1 = step100.iter().filter(|(x, y)| *x < xc && *y < yc).count() as i32;
    let q2 = step100.iter().filter(|(x, y)| *x > xc && *y < yc).count() as i32;
    let q3 = step100.iter().filter(|(x, y)| *x < xc && *y > yc).count() as i32;
    let q4 = step100.iter().filter(|(x, y)| *x > xc && *y > yc).count() as i32;

    // print visualisation in terminal
    #[allow(unused)]
    let vis = |poss: std::slice::Iter<(i32, i32, i32, i32)>| {
        // print!("\x1B[2J\x1B[1;1H");
        let poss = poss.collect::<Vec<_>>();
        for i in 0..yw {
            for j in 0..xw {
                if poss.iter().any(|(px, py, _, _)| *px == j && *py == i) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    };

    let mut i = 0;
    loop {
        i += 1;
        poss = poss.iter().map(|x| step(*x)).collect::<Vec<_>>();
        poss.sort_by(|(px1, py1, _, _), (px2, py2, _, _)| {
            if py1 == py2 {
                px1.cmp(px2)
            } else {
                py1.cmp(py2)
            }
        });
        if poss
            .windows(10)
            .any(|w| w[0].1 == w[9].1 && (1..10).all(|k| w[0].0 + k == w[k as usize].0))
        {
            // println!("For i = {}", i);
            // vis(poss.iter());
            break;
        }
    }

    return (q1 * q2 * q3 * q4, i);
}
