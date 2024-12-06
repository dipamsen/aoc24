use std::collections::HashSet;

#[derive(Clone, Hash, PartialEq, Eq)]
struct Position {
    i: usize,
    j: usize,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Grid {
    grid: Vec<Vec<i32>>,
}

#[derive(PartialEq, Eq, Hash)]
struct MoveState {
    pos: Position,
    dir: Direction,
}

impl Position {
    fn mov(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.i -= 1,
            Direction::Right => self.j += 1,
            Direction::Down => self.i += 1,
            Direction::Left => self.j -= 1,
        }
    }

    fn next(&self, dir: &Direction) -> Position {
        let mut next = self.clone();
        next.mov(dir);
        next
    }
}

impl Direction {
    fn rotate(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl Grid {
    fn obstacle_at(&self, pos: &Position) -> bool {
        self.grid
            .get(pos.i)
            .and_then(|r| r.get(pos.j))
            .map_or(false, |&x| x == 1)
    }

    fn next_in_bounds(&self, pos: &Position, dir: &Direction) -> bool {
        match dir {
            Direction::Up => pos.i > 0,
            Direction::Right => pos.j < self.grid[0].len() - 1,
            Direction::Down => pos.i < self.grid.len() - 1,
            Direction::Left => pos.j > 0,
        }
    }
}

fn infinite_loop(grid: &Grid, init: Position) -> bool {
    let mut pos = init.clone();
    let mut dir = Direction::Up;

    let mut states = HashSet::new();

    loop {
        if grid.next_in_bounds(&pos, &dir) {
            if grid.obstacle_at(&pos.next(&dir)) {
                dir.rotate();
            } else {
                pos.mov(&dir);
            }
            let state = MoveState {
                pos: pos.clone(),
                dir: dir.clone(),
            };
            if states.contains(&state) {
                return true;
            }
            states.insert(state);
        } else {
            return false;
        }
    }
}

pub fn run(input: &str) -> (i32, i32) {
    let i = input
        .lines()
        .map(|line| line.find('^'))
        .position(|x| x.is_some())
        .unwrap();
    let j = input.lines().nth(i).unwrap().find('^').unwrap();

    let init: Position = Position { i, j };

    let mut grid = Grid {
        grid: input
            .lines()
            .map(|line| line.chars().map(|c| if c == '#' { 1 } else { 0 }).collect())
            .collect(),
    };

    let mut pos = init.clone();
    let mut dir = Direction::Up;

    let mut visited = HashSet::new();
    visited.insert(pos.clone());

    loop {
        if grid.next_in_bounds(&pos, &dir) {
            if grid.obstacle_at(&pos.next(&dir)) {
                dir.rotate();
            } else {
                pos.mov(&dir);
                visited.insert(pos.clone());
            }
        } else {
            break;
        }
    }

    let mut count = 0;

    for pos in visited.iter() {
        grid.grid[pos.i][pos.j] = 1;

        if infinite_loop(&grid, init.clone()) {
            count += 1;
        }

        grid.grid[pos.i][pos.j] = 0;
    }

    (visited.len() as i32, count)
}
