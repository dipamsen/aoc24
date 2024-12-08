use std::collections::{HashMap, HashSet};

pub fn run(input: &str) -> (i32, i32) {
    //     let input = "............
    // ........0...
    // .....0......
    // .......0....
    // ....0.......
    // ......A.....
    // ............
    // ............
    // ........A...
    // .........A..
    // ............
    // ............";

    let mut antenna_pos: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let grid_size = (
        input.lines().count() as i32,
        input.lines().next().unwrap().chars().count() as i32,
    );

    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if c != '.' {
                if antenna_pos.contains_key(&c) {
                    antenna_pos.get_mut(&c).unwrap().push((i as i32, j as i32));
                } else {
                    antenna_pos.insert(c, vec![(i as i32, j as i32)]);
                }
            }
        })
    });
    let within_bounds =
        |(i, j): &(i32, i32)| i >= &0 && j >= &0 && i < &grid_size.0 && j < &grid_size.1;

    let antinodes = antenna_pos
        .iter()
        .flat_map(|(_, pos)| {
            pos.iter().flat_map(|(i1, j1)| {
                pos.iter()
                    .flat_map(|(i2, j2)| {
                        if i1 == i2 && j1 == j2 {
                            return vec![];
                        }
                        vec![
                            (i1 + (i2 - i1) * 2, j1 + (j2 - j1) * 2),
                            (i1 - (i2 - i1), j1 - (j2 - j1)),
                        ]
                    })
                    .collect::<Vec<_>>()
            })
        })
        .filter(|x| within_bounds(x))
        .collect::<HashSet<_>>();

    // println!("{:?}", antinodes.count());

    let get_points_in_line = |(i1, j1): (i32, i32), (i2, j2): (i32, i32)| {
        let mut points = HashSet::new();

        let mut p = (i2, j2);
        while within_bounds(&p) {
            points.insert(p);
            p = (p.0 + (i2 - i1), p.1 + (j2 - j1));
        }
        p = (i1, j1);
        while within_bounds(&p) {
            points.insert(p);
            p = (p.0 - (i2 - i1), p.1 - (j2 - j1));
        }
        points
    };

    let anti_points = antenna_pos
        .iter()
        .flat_map(|(_, pos)| {
            pos.iter().flat_map(|(i1, j1)| {
                pos.iter()
                    .flat_map(|(i2, j2)| {
                        if i1 == i2 && j1 == j2 {
                            return HashSet::new();
                        }
                        get_points_in_line((*i1, *j1), (*i2, *j2))
                    })
                    .collect::<HashSet<_>>()
            })
        })
        .collect::<HashSet<_>>();

    (antinodes.len() as i32, anti_points.len() as i32)
}
