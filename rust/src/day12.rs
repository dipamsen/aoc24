use itertools::Itertools;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Ord, PartialOrd)]
enum Direction {
    TOP,
    RIGHT,
    BOTTOM,
    LEFT,
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Fence {
    p: i32, // primary coordinate
    t: i32, // transverse coordinate
    dir: Direction,
}

impl Direction {
    fn get_offset(&self) -> (i32, i32) {
        match self {
            Direction::TOP => (-1, 0),
            Direction::RIGHT => (0, 1),
            Direction::BOTTOM => (1, 0),
            Direction::LEFT => (0, -1),
        }
    }
}

fn dfs(
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    fences: &mut HashSet<Fence>,
    i: i32,
    j: i32,
) -> (i32, i32) {
    let dirns = vec![
        Direction::TOP,
        Direction::RIGHT,
        Direction::BOTTOM,
        Direction::LEFT,
    ];
    visited[i as usize][j as usize] = true;
    let mut area = 1;
    let mut perimeter = 4;

    for dir in dirns {
        let (di, dj) = dir.get_offset();
        let ni = i + di;
        let nj = j + dj;

        if ni >= 0
            && ni < grid.len() as i32
            && nj >= 0
            && nj < grid[0].len() as i32
            && grid[ni as usize][nj as usize] == grid[i as usize][j as usize]
        {
            if !visited[ni as usize][nj as usize] {
                let (a, p) = dfs(grid, visited, fences, ni, nj);
                area += a;
                perimeter += p;
            }
            perimeter -= 1;
        } else {
            if dir == Direction::TOP || dir == Direction::BOTTOM {
                // horizontal fence
                fences.insert(Fence { p: i, t: j, dir });
            }
            if dir == Direction::LEFT || dir == Direction::RIGHT {
                // vertical fence
                fences.insert(Fence { t: i, p: j, dir });
            }
        }
    }
    (area, perimeter)
}

pub fn run(input: &str) -> (i32, i32) {
    //     let input = "RRRRIICCFF
    // RRRRIICCCF
    // VVRRRCCFFF
    // VVRCCCJFFF
    // VVVVCJJCFE
    // VVIVCCJJEE
    // VVIIICJJEE
    // MIIIIIJJEE
    // MIIISIJEEE
    // MMMISSJEEE";
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let m = grid.len();
    let n = grid[0].len();

    let mut visited = grid
        .iter()
        .map(|r| r.iter().map(|_| false).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut cost1 = 0;
    let mut cost2 = 0;
    for i in 0..m {
        for j in 0..n {
            if !visited[i][j] {
                let mut fences = HashSet::new();
                let (a, p) = dfs(&grid, &mut visited, &mut fences, i as i32, j as i32);
                cost1 += a * p;
                let e = fences
                    .iter()
                    .sorted_by(|a, b| a.dir.cmp(&b.dir).then(a.p.cmp(&b.p)).then(a.t.cmp(&b.t)))
                    .group_by(|a| (a.dir, a.p))
                    .into_iter()
                    .map(|(_, chunk)| {
                        // println!("{:?}", chunk);
                        let mut arr = chunk.map(|c| c.t).collect::<Vec<_>>();
                        arr.sort();
                        arr.windows(2).fold(
                            1,
                            |acc, w| {
                                if w[1] - w[0] == 1 {
                                    acc
                                } else {
                                    acc + 1
                                }
                            },
                        )
                    })
                    .sum::<i32>();
                // println!("{:?}", fences);
                // println!("{:?} {:?}", a, e);
                cost2 += a * e
            }
        }
    }

    return (cost1, cost2);
}
