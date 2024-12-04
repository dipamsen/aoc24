fn transpose<T: Clone>(input: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    if input.is_empty() {
        return vec![];
    }

    (0..input[0].len())
        .map(|i| input.iter().map(|r| r[i].clone()).collect())
        .collect()
}

pub fn run(input: &str) -> (i32, i32) {
    //     let input = "MMMSXXMASM\n\
    // MSAMXMSMSA\n\
    // AMXSXMAAMM\n\
    // MSAMASMSMX\n\
    // XMASAMXAMM\n\
    // XXAMMXXAMA\n\
    // SMSMSASXSS\n\
    // SAXAMASAAA\n\
    // MAMMMXMMMM\n\
    // MXMXAXMASX";
    let is_valid = |w: &[char]| matches!(w, ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X']);

    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = grid
        .iter()
        .map(|row| row.windows(4).filter(|w| is_valid(w)).count())
        .sum::<usize>();

    count += transpose(&grid)
        .iter()
        .map(|row| row.windows(4).filter(|w| is_valid(w)).count())
        .sum::<usize>();

    count += (0..rows - 3)
        .flat_map(|i| {
            (0..cols - 3)
                .map(|j| {
                    is_valid(&[
                        grid[i][j],
                        grid[i + 1][j + 1],
                        grid[i + 2][j + 2],
                        grid[i + 3][j + 3],
                    ])
                })
                .collect::<Vec<_>>()
        })
        .filter(|&b| b)
        .count();

    count += (0..rows - 3)
        .flat_map(|i| {
            (3..cols)
                .map(|j| {
                    is_valid(&[
                        grid[i][j],
                        grid[i + 1][j - 1],
                        grid[i + 2][j - 2],
                        grid[i + 3][j - 3],
                    ])
                })
                .collect::<Vec<_>>()
        })
        .filter(|&b| b)
        .count();

    let count2 = (0..rows - 2)
        .flat_map(|i| {
            (0..cols - 2)
                .map(|j| {
                    matches!(
                        grid[i..i + 3]
                            .iter()
                            .map(|row| &row[j..j + 3])
                            .collect::<Vec<_>>()[..],
                        [['M', _, 'S'], [_, 'A', _], ['M', _, 'S']]
                            | [['S', _, 'M'], [_, 'A', _], ['S', _, 'M']]
                            | [['M', _, 'M'], [_, 'A', _], ['S', _, 'S']]
                            | [['S', _, 'S'], [_, 'A', _], ['M', _, 'M']]
                    )
                })
                .collect::<Vec<_>>()
        })
        .filter(|&b| b)
        .count();

    (count as i32, count2 as i32)
}
