// // TODO: FIX

pub fn run(input: &str) -> (i32, i32) {
    (0, 0)
}

// #[derive(Debug, Clone)]
// struct Pos {
//     i: i32,
//     j: i32,
// }

// #[derive(Clone, Copy)]
// enum Dir {
//     Up,
//     Down,
//     Left,
//     Right,
// }

// /// finds boxes that are connected to a given box
// fn get_connected_boxes<'a>(pos: &Pos, boxes: &'a mut Vec<Pos>, dir: Dir) -> Vec<&'a mut Pos> {
//     //
//     // ***     <---- possible positions of connected boxes
//     //  []     <---- * (main)
//     //
//     //  []#
//     // [][]
//     //  []
//     //
//     let mut connected: Vec<&mut Pos> = vec![];

//     match dir {
//         Dir::Up => {
//             let bs = boxes
//                 .iter_mut()
//                 .filter(|b| (b.j - pos.j).abs() <= 1 && b.i == pos.i - 1)
//                 .collect::<Vec<_>>();
//             connected.extend(bs);
//             for b in bs {
//                 connected.extend(get_connected_boxes(b, boxes, dir));
//             }
//         }

//         Dir::Down => {
//             let bs = boxes
//                 .iter_mut()
//                 .filter(|b| (b.j - pos.j).abs() <= 1 && b.i == pos.i + 1);
//             connected.extend(bs);
//             for b in bs {
//                 connected.extend(get_connected_boxes(b, boxes, dir));
//             }
//         }
//         Dir::Left => {
//             let bs = boxes
//                 .iter_mut()
//                 .filter(|b| (b.i - pos.i).abs() == 0 && b.j == pos.j - 2);
//             connected.extend(bs);
//             for b in bs {
//                 connected.extend(get_connected_boxes(b, boxes, dir));
//             }
//         }
//         Dir::Right => {
//             let bs = boxes
//                 .iter_mut()
//                 .filter(|b| (b.i - pos.i).abs() == 0 && b.j == pos.j + 2);
//             connected.extend(bs);
//             for b in bs {
//                 connected.extend(get_connected_boxes(b, boxes, dir));
//             }
//         }
//     }

//     connected
// }

// fn part1(lines: &str, steps: &Vec<Dir>) -> i32 {
//     let m = lines.lines().count() as i32;
//     let n = lines.lines().next().unwrap().len() as i32;

//     let walls = lines
//         .lines()
//         .enumerate()
//         .flat_map(|(i, l)| {
//             l.chars()
//                 .enumerate()
//                 .filter_map(|(j, c)| {
//                     if c == '#' {
//                         Some(Pos {
//                             i: i as i32,
//                             j: j as i32,
//                         })
//                     } else {
//                         None
//                     }
//                 })
//                 .collect::<Vec<_>>()
//         })
//         .collect::<Vec<_>>();
//     let mut boxes = lines
//         .lines()
//         .enumerate()
//         .flat_map(|(i, l)| {
//             l.chars()
//                 .enumerate()
//                 .filter_map(|(j, c)| {
//                     if c == 'O' {
//                         Some(Pos {
//                             i: i as i32,
//                             j: j as i32,
//                         })
//                     } else {
//                         None
//                     }
//                 })
//                 .collect::<Vec<_>>()
//         })
//         .collect::<Vec<_>>();
//     let mut robot = Pos {
//         i: lines.lines().position(|l| l.contains('@')).unwrap() as i32,
//         j: lines
//             .lines()
//             .find(|l| l.contains('@'))
//             .unwrap()
//             .find('@')
//             .unwrap() as i32,
//     };

//     for step in steps {
//         match step {
//             Dir::Up => {
//                 let nearest_wall = walls
//                     .iter()
//                     .filter(|w| w.i < robot.i && w.j == robot.j)
//                     .max_by(|a, b| a.i.cmp(&b.i))
//                     .unwrap()
//                     .i;
//                 let mut nearest_blank = robot.i - 1;
//                 loop {
//                     if nearest_blank >= 0
//                         && !walls.iter().any(|w| w.i == nearest_blank && w.j == robot.j)
//                         && !boxes.iter().any(|b| b.i == nearest_blank && b.j == robot.j)
//                     {
//                         break;
//                     }
//                     if nearest_blank < 0 {
//                         break;
//                     }
//                     nearest_blank -= 1;
//                 }

//                 if nearest_blank > nearest_wall && nearest_blank >= 0 {
//                     robot.i -= 1;
//                     boxes
//                         .iter_mut()
//                         .find(|b| b.i == robot.i && b.j == robot.j)
//                         .map(|b| b.i = nearest_blank);
//                 }
//             }
//             Dir::Down => {
//                 let nearest_wall = walls
//                     .iter()
//                     .filter(|w| w.i > robot.i && w.j == robot.j)
//                     .min_by(|a, b| a.i.cmp(&b.i))
//                     .unwrap()
//                     .i;
//                 let mut nearest_blank = robot.i + 1;
//                 loop {
//                     if nearest_blank < m as i32
//                         && !walls.iter().any(|w| w.i == nearest_blank && w.j == robot.j)
//                         && !boxes.iter().any(|b| b.i == nearest_blank && b.j == robot.j)
//                     {
//                         break;
//                     }
//                     if nearest_blank >= m {
//                         break;
//                     }
//                     nearest_blank += 1;
//                 }

//                 if nearest_blank < nearest_wall && nearest_blank < m as i32 {
//                     robot.i += 1;
//                     boxes
//                         .iter_mut()
//                         .find(|b| b.i == robot.i && b.j == robot.j)
//                         .map(|b| b.i = nearest_blank);
//                 }
//             }
//             Dir::Left => {
//                 let nearest_wall = walls
//                     .iter()
//                     .filter(|w| w.i == robot.i && w.j < robot.j)
//                     .max_by(|a, b| a.j.cmp(&b.j))
//                     .unwrap()
//                     .j;
//                 let mut nearest_blank = robot.j - 1;
//                 loop {
//                     if nearest_blank >= 0
//                         && !walls.iter().any(|w| w.i == robot.i && w.j == nearest_blank)
//                         && !boxes.iter().any(|b| b.i == robot.i && b.j == nearest_blank)
//                     {
//                         break;
//                     }
//                     if nearest_blank < 0 {
//                         break;
//                     }
//                     nearest_blank -= 1;
//                 }

//                 if nearest_blank > nearest_wall && nearest_blank >= 0 {
//                     robot.j -= 1;
//                     boxes
//                         .iter_mut()
//                         .find(|b| b.i == robot.i && b.j == robot.j)
//                         .map(|f| f.j = nearest_blank);
//                 }
//             }
//             Dir::Right => {
//                 let nearest_wall = walls
//                     .iter()
//                     .filter(|w| w.i == robot.i && w.j > robot.j)
//                     .min_by(|a, b| a.j.cmp(&b.j))
//                     .unwrap()
//                     .j;
//                 let mut nearest_blank = robot.j + 1;
//                 loop {
//                     if nearest_blank < n as i32
//                         && !walls.iter().any(|w| w.i == robot.i && w.j == nearest_blank)
//                         && !boxes.iter().any(|b| b.i == robot.i && b.j == nearest_blank)
//                     {
//                         break;
//                     }
//                     if nearest_blank >= n {
//                         break;
//                     }
//                     nearest_blank += 1;
//                 }

//                 if nearest_blank < nearest_wall && nearest_blank < n as i32 {
//                     robot.j += 1;
//                     boxes
//                         .iter_mut()
//                         .find(|b| b.i == robot.i && b.j == robot.j)
//                         .map(|f| f.j = nearest_blank);
//                 }
//             }
//         }
//     }

//     boxes.iter().map(|b| 100 * b.i + b.j).sum()
// }

// fn part2(lines: &str, steps: &Vec<Dir>) -> i32 {
//     let m = lines.lines().count() as i32;
//     let n = 2 * lines.lines().next().unwrap().len() as i32;

//     let walls = lines
//         .lines()
//         .enumerate()
//         .flat_map(|(i, l)| {
//             l.chars()
//                 .enumerate()
//                 .filter_map(|(j, c)| {
//                     if c == '#' {
//                         Some([
//                             Pos {
//                                 i: i as i32,
//                                 j: 2 * j as i32,
//                             },
//                             Pos {
//                                 i: i as i32,
//                                 j: 2 * j as i32 + 1,
//                             },
//                         ])
//                     } else {
//                         None
//                     }
//                 })
//                 .flatten()
//                 .collect::<Vec<_>>()
//         })
//         .collect::<Vec<_>>();
//     let mut boxes = lines
//         .lines()
//         .enumerate()
//         .flat_map(|(i, l)| {
//             l.chars()
//                 .enumerate()
//                 .filter_map(|(j, c)| {
//                     if c == 'O' {
//                         Some(Pos {
//                             i: i as i32,
//                             j: 2 * j as i32,
//                         })
//                     } else {
//                         None
//                     }
//                 })
//                 .collect::<Vec<_>>()
//         })
//         .collect::<Vec<_>>();
//     let mut robot = Pos {
//         i: lines.lines().position(|l| l.contains('@')).unwrap() as i32,
//         j: 2 * lines
//             .lines()
//             .find(|l| l.contains('@'))
//             .unwrap()
//             .find('@')
//             .unwrap() as i32,
//     };

//     for step in steps {
//         //
//         //  [][][]
//         // [][]
//         //  @
//         // What happens when you go up?

//         match step {
//             Dir::Up => {
//                 // find connected boxes, also check if all of them can go up.
//                 let mut connected_boxes: Vec<&mut Pos> = vec![]; // references to boxes from boxes
//                 let first = boxes
//                     .iter_mut()
//                     .find(|b| b.i == robot.i - 1 && b.j == robot.j || b.j == robot.j - 1);

//                 if let Some(first) = first {
//                     connected_boxes.push(*first);
//                     connected_boxes.extend(get_connected_boxes(*first, &boxes, Dir::Up));

//                     let can_move = connected_boxes.iter().all(|b| {
//                         !walls
//                             .iter()
//                             .any(|w| w.i == b.i - 1 && (w.j == b.j || w.j == b.j - 1))
//                     });

//                     if can_move {
//                         robot.i -= 1;
//                         for b in connected_boxes {
//                             b.i -= 1;
//                         }
//                     }
//                 } else {
//                     // check for walls
//                     let can_move = !walls.iter().any(|w| w.i == robot.i - 1 && w.j == robot.j);
//                     if can_move {
//                         robot.i -= 1;
//                     }
//                 }
//             }
//             Dir::Down => {
//                 let mut connected_boxes = vec![];
//                 let first = boxes
//                     .iter()
//                     .find(|b| b.i == robot.i + 1 && b.j == robot.j || b.j == robot.j - 1);

//                 if let Some(first) = first {
//                     connected_boxes.push(*first);
//                     connected_boxes.extend(get_connected_boxes(*first, &boxes, Dir::Down));

//                     let can_move = connected_boxes.iter().all(|b| {
//                         !walls
//                             .iter()
//                             .any(|w| w.i == b.i + 1 && (w.j == b.j || w.j == b.j - 1))
//                     });

//                     if can_move {
//                         robot.i += 1;
//                         for b in connected_boxes.iter_mut() {
//                             b.i += 1;
//                         }
//                     }
//                 } else {
//                     let can_move = !walls.iter().any(|w| w.i == robot.i + 1 && w.j == robot.j);
//                     if can_move {
//                         robot.i += 1;
//                     }
//                 }
//             }
//             Dir::Left => {
//                 let mut connected_boxes = vec![];
//                 let first = boxes.iter().find(|b| b.i == robot.i && b.j == robot.j - 2);

//                 if let Some(first) = first {
//                     connected_boxes.push(*first);
//                     connected_boxes.extend(get_connected_boxes(*first, &boxes, Dir::Left));

//                     let can_move = connected_boxes
//                         .iter()
//                         .all(|b| !walls.iter().any(|w| w.j == b.j - 1 && w.i == b.i));

//                     if can_move {
//                         robot.j -= 1;
//                         for b in connected_boxes.iter_mut() {
//                             b.j -= 1;
//                         }
//                     }
//                 } else {
//                     let can_move = !walls.iter().any(|w| w.j == robot.j - 1 && w.i == robot.i);
//                     if can_move {
//                         robot.j -= 1;
//                     }
//                 }
//             }
//             Dir::Right => {
//                 let mut connected_boxes = vec![];
//                 let first = boxes.iter().find(|b| b.i == robot.i && b.j == robot.j + 1);

//                 if let Some(first) = first {
//                     connected_boxes.push(*first);
//                     connected_boxes.extend(get_connected_boxes(*first, &boxes, Dir::Right));

//                     let can_move = connected_boxes
//                         .iter()
//                         .all(|b| !walls.iter().any(|w| w.j == b.j + 2 && w.i == b.i));

//                     if can_move {
//                         robot.j += 1;
//                         for b in connected_boxes.iter_mut() {
//                             b.j += 1;
//                         }
//                     }
//                 } else {
//                     let can_move = !walls.iter().any(|w| w.j == robot.j + 1 && w.i == robot.i);
//                     if can_move {
//                         robot.j += 1;
//                     }
//                 }
//             }
//         }
//     }

//     boxes.iter().map(|b| 100 * b.i + b.j).sum()
// }

// pub fn run(input: &str) -> (i32, i32) {
//     let input = "#######
// #...#.#
// #.....#
// #..OO@#
// #..O..#
// #.....#
// #######

// <vv<<^^<<^^";

//     let (lines, steps) = input.split_once("\n\n").unwrap();

//     let steps = steps
//         .lines()
//         .flat_map(|l| {
//             l.chars().map(|c| match c {
//                 '^' => Dir::Up,
//                 'v' => Dir::Down,
//                 '<' => Dir::Left,
//                 '>' => Dir::Right,
//                 _ => unreachable!(),
//             })
//         })
//         .collect::<Vec<_>>();

//     return (part1(lines, &steps), part2(lines, &steps));
// }
