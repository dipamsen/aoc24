use std::collections::HashMap;

type Point = (i32, i32);

fn increment(c: char) -> char {
    (c as u8 + 1) as char
}

fn dfs(grid: &Vec<Vec<char>>, i: usize, j: usize) -> HashMap<Point, i32> {
    let curr = grid[i][j];
    if curr == '9' {
        let mut m = HashMap::new();
        m.insert((i as i32, j as i32), 1);
        return m;
    }

    let next = increment(curr);
    let mut m = HashMap::new();

    let dirns = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    for (di, dj) in dirns {
        let ni = i as i32 + di;
        let nj = j as i32 + dj;
        if ni >= 0
            && (ni as usize) < grid.len()
            && nj >= 0
            && (nj as usize) < grid[0].len()
            && grid[ni as usize][nj as usize] == next
        {
            let k = dfs(&grid, ni as usize, nj as usize);
            for (p, v) in k {
                m.entry(p).and_modify(|e| *e += v).or_insert(v);
            }
        }
    }
    m
}

pub fn run(input: &str) -> (i32, i32) {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let res = grid
        .iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.iter()
                .enumerate()
                .map(|(j, c)| (i, j, c))
                .collect::<Vec<_>>()
        })
        .filter(|(_, _, &c)| c == '0')
        .map(|(i, j, _)| dfs(&grid, i, j))
        .collect::<Vec<_>>();

    (
        res.iter().map(|m| m.len() as i32).sum::<i32>(),
        res.iter().map(|m| m.values().sum::<i32>()).sum(),
    )
}
