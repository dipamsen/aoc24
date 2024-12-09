#[derive(Debug, Clone)]
struct File {
    id: i64,
    size: i64,
    pos: i64,
}

pub fn run(input: &str) -> (i64, i64) {
    // let input = "2333133121414131402";

    let mut mem = input
        .chars()
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, c)| {
            let size: i64 = c.to_digit(10).unwrap() as i64;
            for _ in 0..size {
                if i % 2 == 0 {
                    acc.push(i as i64 / 2);
                } else {
                    acc.push(-1);
                }
            }
            acc
        });

    let mut l = 0;
    let mut r = mem.len() - 1;
    loop {
        while mem[l] != -1 {
            l += 1
        }
        while mem[r] == -1 {
            r -= 1
        }
        if l >= r {
            break;
        }
        mem.swap(l, r);
    }

    let ans1 = mem
        .iter()
        .enumerate()
        .map(|(i, &x)| if x >= 0 { (i as i64) * x } else { 0 })
        .sum::<i64>();

    let parsed = input.chars().enumerate().fold(Vec::new(), |mut acc, x| {
        let (i, c) = x;
        let size: i64 = c.to_digit(10).unwrap() as i64;
        let id: i64 = if i % 2 == 0 { i as i64 / 2 } else { -1 };
        let pos = acc.last().map_or(0, |f: &File| f.pos + f.size);
        acc.push(File { id, size, pos });
        acc
    });
    let mut files = parsed
        .iter()
        .filter(|f| f.id != -1)
        .cloned()
        .collect::<Vec<_>>();
    let mut blanks = parsed
        .iter()
        .filter(|f| f.id == -1)
        .cloned()
        .collect::<Vec<_>>();

    for f in files.iter_mut().rev() {
        for b in blanks.iter_mut() {
            if b.pos >= f.pos {
                break;
            }
            if b.size >= f.size {
                f.pos = b.pos; // ****
                b.size -= f.size;
                b.pos += f.size;
                break;
            }
        }
    }

    let ans2 = files
        .iter()
        .map(|f| f.id * (f.pos..(f.pos + f.size)).sum::<i64>())
        .sum();

    (ans1, ans2)
}
