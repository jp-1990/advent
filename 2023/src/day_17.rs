// use std::cmp::Reverse;
// use std::collections::{BinaryHeap, HashMap};
//
// #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
// enum Move {
//     N,
//     E,
//     S,
//     W,
// }
//
// struct PriorityQueue {
//     queue: BinaryHeap<Reverse<Node>>,
// }
//
// impl PriorityQueue {
//     pub fn new() -> PriorityQueue {
//         PriorityQueue {
//             queue: BinaryHeap::new(),
//         }
//     }
//
//     fn push(&mut self, val: Node) {
//         self.queue.push(Reverse(val))
//     }
//
//     fn pop(&mut self) -> Option<Node> {
//         match self.queue.pop() {
//             Some(val) => Some(val.0),
//             None => None,
//         }
//     }
//
//     fn remove(&mut self, id: String) -> Node {
//         let mut test = vec![];
//         self.queue.retain(|node| {
//             if node.0.id != id {
//                 true
//             } else {
//                 test.push(node.0.clone());
//                 false
//             }
//         });
//
//         test[0].clone()
//     }
//
//     fn contains(&self, id: String) -> bool {
//         self.queue.iter().any(|node| node.0.id == id)
//     }
// }
//
// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// struct Coord {
//     x: usize,
//     y: usize,
// }
//
// #[derive(Debug)]
// struct Matrix {
//     grid: Vec<Vec<usize>>,
//     width: usize,
//     height: usize,
//     start: Coord,
//     end: Coord,
// }
//
// impl Matrix {
//     fn safe_index(&self, x: isize, y: isize) -> Option<usize> {
//         match (x, y) {
//             (x, _) if x < 0 || x >= self.width as isize => None,
//             (_, y) if y < 0 || y >= self.height as isize => None,
//             _ => Some(self.grid[y as usize][x as usize]),
//         }
//     }
//
//     fn build_node(
//         &self,
//         x: isize,
//         y: isize,
//         parent_g: usize,
//         movement: Move,
//         parent_id: String,
//         same_moves: usize,
//     ) -> Option<Node> {
//         let value = match self.safe_index(x, y) {
//             Some(v) => v,
//             None => return None,
//         };
//         let coord = Coord {
//             x: x.try_into().unwrap(),
//             y: y.try_into().unwrap(),
//         };
//
//         let g = parent_g + value;
//         let h = manhattan(&coord, &self.end);
//
//         Some(Node::new(
//             coord,
//             value,
//             g + h,
//             g,
//             h,
//             movement,
//             parent_id,
//             same_moves,
//         ))
//     }
// }
//
// #[derive(Debug, Clone)]
// struct Node {
//     coord: Coord,
//     g: usize,
//     h: usize,
//     f: usize,
//     value: usize,
//     id: String,
//     parent_id: String,
//     movement: Move,
//     same_moves: usize,
// }
//
// impl Node {
//     pub fn new(
//         coord: Coord,
//         value: usize,
//         f: usize,
//         g: usize,
//         h: usize,
//         movement: Move,
//         parent_id: String,
//         same_moves: usize,
//     ) -> Node {
//         let id = format!("{:?}{:?}", &coord.x, &coord.y);
//
//         Node {
//             id,
//             coord,
//             movement,
//             value,
//             f,
//             g,
//             h,
//             parent_id,
//             same_moves,
//         }
//     }
//
//     fn get_neighbours(&self, matrix: &Matrix) -> Vec<Option<Node>> {
//         let mut neighbours: Vec<Option<Node>> = vec![];
//
//         for movement in [Move::N, Move::E, Move::S, Move::W] {
//             if movement == self.movement && self.same_moves == 3 {
//                 continue;
//             }
//
//             match self.movement {
//                 Move::N if movement == Move::S => continue,
//                 Move::E if movement == Move::W => continue,
//                 Move::S if movement == Move::N => continue,
//                 Move::W if movement == Move::E => continue,
//                 _ => (),
//             }
//
//             let same_moves = if self.movement == movement {
//                 self.same_moves + 1
//             } else {
//                 1
//             };
//
//             let x_mod = match movement {
//                 Move::E => 1,
//                 Move::W => -1,
//                 _ => 0,
//             };
//
//             let y_mod = match movement {
//                 Move::N => 1,
//                 Move::S => -1,
//                 _ => 0,
//             };
//
//             let node = matrix.build_node(
//                 self.coord.x as isize + x_mod,
//                 self.coord.y as isize + y_mod,
//                 self.g,
//                 movement,
//                 String::from(&self.id),
//                 same_moves,
//             );
//
//             neighbours.push(node);
//         }
//
//         // let n_node = matrix.build_node(
//         //     self.coord.x as isize,
//         //     self.coord.y as isize - 1,
//         //     self.g,
//         //     Move::N,
//         //     String::from(&self.id),
//         // );
//         // let e_node = matrix.build_node(
//         //     self.coord.x as isize + 1,
//         //     self.coord.y as isize,
//         //     self.g,
//         //     Move::E,
//         //     String::from(&self.id),
//         // );
//         // let s_node = matrix.build_node(
//         //     self.coord.x as isize,
//         //     self.coord.y as isize + 1,
//         //     self.g,
//         //     Move::S,
//         //     String::from(&self.id),
//         // );
//         // let w_node = matrix.build_node(
//         //     self.coord.x as isize - 1,
//         //     self.coord.y as isize,
//         //     self.g,
//         //     Move::W,
//         //     String::from(&self.id),
//         // );
//         //
//         // match self.movement {
//         //     Move::N => neighbours = vec![e_node, n_node, w_node],
//         //     Move::E => neighbours = vec![n_node, s_node, e_node],
//         //     Move::S => neighbours = vec![s_node, e_node, w_node],
//         //     Move::W => neighbours = vec![n_node, w_node, s_node],
//         // }
//
//         neighbours
//     }
// }
//
// impl Ord for Node {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         (self.f).cmp(&other.f)
//     }
// }
//
// impl PartialOrd for Node {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }
//
// impl PartialEq for Node {
//     fn eq(&self, other: &Self) -> bool {
//         self.id == other.id
//     }
// }
//
// impl Eq for Node {}
//
// fn parse_file(file: &str) -> Matrix {
//     let mut width: usize = 0;
//
//     let grid = file
//         .lines()
//         .map(|line| {
//             width = line.len();
//             line.chars()
//                 .map(|s| s.to_digit(10).unwrap().try_into().unwrap())
//                 .collect::<Vec<usize>>()
//         })
//         .collect::<Vec<Vec<usize>>>();
//
//     let height = grid.len();
//
//     Matrix {
//         grid,
//         width,
//         height,
//         start: Coord { x: 0, y: 0 },
//         end: Coord {
//             x: width - 1,
//             y: height - 1,
//         },
//     }
// }
//
// fn manhattan(start: &Coord, end: &Coord) -> usize {
//     // euclidean_floor
//     // let x = isize::pow(start.x as isize - end.x as isize, 2) as f64;
//     // let y = isize::pow(start.y as isize - end.y as isize, 2) as f64;
//     //
//     // f64::sqrt(x + y).floor() as usize
//
//     // ((start.x as isize - end.x as isize).abs() + (start.y as isize - end.y as isize).abs())
//     //     .try_into()
//     //     .unwrap()
//
//     end.x - start.x + end.y - start.y
//
//     // 0
// }
//
// pub fn part_1(file: &str) -> Option<usize> {
//     let matrix = parse_file(file);
//
//     let mut done = false;
//     let mut open_list = PriorityQueue::new();
//     let mut closed_list: HashMap<String, Node> = HashMap::new();
//
//     open_list.push(
//         matrix
//             .build_node(0, 0, 0, Move::E, String::from("START"), 1)
//             .unwrap(),
//     );
//
//     while !done {
//         let current;
//         match open_list.pop() {
//             Some(n) => current = n,
//             None => {
//                 done = true;
//                 continue;
//             }
//         };
//
//         // done
//         if current.coord == matrix.end {
//             done = true;
//             // TODO: trace back path
//             let mut steps = 1;
//             let mut count = current.value;
//             let mut prev_id = &current.parent_id;
//
//             let mut test: Vec<Coord> = vec![current.coord];
//
//             while prev_id != "START" {
//                 let prev_node = &closed_list.get(prev_id).unwrap();
//                 prev_id = &prev_node.parent_id;
//                 count = &prev_node.value + count;
//                 steps += 1;
//
//                 test.push(prev_node.coord)
//             }
//
//             print!("DONE: {:?}: {:?} {:?}\n", steps, count, current.g);
//
//             let mut grid = vec![vec!['.'; matrix.width]; matrix.height];
//
//             for (_, node) in &closed_list {
//                 // println!("{:?}", node);
//                 grid[node.coord.y][node.coord.x] = '|';
//             }
//
//             test.reverse();
//             for coord in test {
//                 // println!("{:?}", coord);
//                 grid[coord.y][coord.x] = '#';
//             }
//
//             // for line in grid {
//             //     println!("{:?}", line);
//             // }
//
//             let res = grid
//                 .iter()
//                 .map(|v| v.iter().collect::<String>())
//                 .collect::<Vec<String>>()
//                 .join("\n");
//
//             std::fs::write("test.txt", res);
//             continue;
//         }
//
//         let neighbours = current.get_neighbours(&matrix);
//
//         // println!("CURRENT: {:?}::", current);
//         for neighbour in neighbours {
//             match neighbour {
//                 Some(n) => {
//                     println!("{:?}::", n);
//
//                     // // if current.movement == n.movement {
//                     //     let mut same_moves = 1;
//                     //     let mut prev_id = &current.parent_id;
//                     //
//                     //     for _ in 0..3 {
//                     //         match &closed_list.get(prev_id) {
//                     //             Some(prev) => {
//                     //                 prev_id = &prev.parent_id;
//                     //                 // println!("{:?}", prev.movement);
//                     //                 if &prev.movement == &current.movement {
//                     //                     same_moves += 1;
//                     //                 } else {
//                     //                     break;
//                     //                 };
//                     //             }
//                     //             // None => (),
//                     //             None => break,
//                     //         };
//                     //     }
//                     //     println!("same: {same_moves}");
//                     //     if same_moves > 3 || same_moves == 3 && n.movement == current.movement {
//                     //         continue;
//                     //     }
//                     // // }
//                     if open_list.contains(String::from(&n.id)) {
//                         // println!("change in open");
//                         let prev_n = open_list.remove(String::from(&n.id));
//                         if n.f >= prev_n.f {
//                             open_list.push(prev_n);
//                         }
//
//                         continue;
//                     }
//
//                     if closed_list.contains_key(&n.id) {
//                         // println!("replace in closed");
//                         let prev_n = closed_list.get(&n.id);
//                         match prev_n {
//                             Some(node) => {
//                                 if n.f >= node.f {
//                                     continue;
//                                     // closed_list.insert(String::from(&n.id), n);
//                                 };
//                             }
//                             None => (),
//                         };
//                     } else {
//                         open_list.push(n);
//                     }
//                 }
//                 None => (),
//             }
//         }
//
//         // println!();
//
//         closed_list.insert(String::from(&current.id), current);
//     }
//
//     Some(0)
// }

use std::{cmp::Ordering, collections::{HashSet, BinaryHeap}};


#[derive(PartialEq, Eq)]
struct Crucible {
    cost: u32,
    pos: Coord,
    dir: Direction,
    moves_in_dir: u8,
}

#[derive(PartialEq, Eq,Hash,Clone,Copy)]
struct Coord {
    row: usize,
    col: usize,
}

#[derive(PartialEq, Eq,Hash,Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// The priority queue holds Crucible
// We define an ordering trait so the one with the lowest cost gets popped from the pq first.
// We do this by flipping the ordering on cost (comparing "other to self" instead of "self to other")
// that way, nodes with a lower cost will compare as Ordering::Greater, and get sent to the front of the pq
impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

// Ensure partialOrd is consistent with Ord. From the docs:
// > If Ord is also implemented for Self and Rhs, it must also be consistent with partial_cmp (see the documentation of that trait for the exact requirements).
// > It’s easy to accidentally make them disagree by deriving some of the traits and manually implementing others.
// tl;dr: implement PartialOrd in terms of the Ord we just specified to ensure correctness. Letting it be automatically determined is wrong sometimes.
impl PartialOrd for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Crucible {
    fn successors(&self, grid: &Vec<Vec<usize>>) -> Vec<Self> {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut successors = Vec::new();
        for dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if self.dir == dir && self.moves_in_dir == 3 {
                // already moved 3 tiles in a straight line, can't move further
                continue;
            }
            if self.dir.opposite() == dir {
                // can't move in opposite direction
                continue;
            }
            // simulate a move inside the bounds
            if let Some(pos) = self.pos.forward(&dir, rows, cols) {
                // calculate the total cost to get to that neighbour
                // it's the total cost to get to the current node + the cost to travel to the neighbour
                let cost = self.cost + grid[pos.row][pos.col] as u32;

                // increment straight_moves if we went straight, else we moved 1 tile in the current direction
                let moves_in_dir = if self.dir == dir {
                    self.moves_in_dir + 1
                } else {
                    1
                };

                successors.push(Crucible {
                    pos,
                    cost,
                    dir,
                    moves_in_dir,
                })
            }
        }

        successors
    }
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl Coord {
    fn forward(&self, dir: &Direction, rows: usize, cols: usize) -> Option<Self> {
        let coord = match dir {
            Direction::Up if self.row > 0 => Coord {
                row: self.row - 1,
                col: self.col,
            },
            Direction::Down if self.row < (rows - 1) => Coord {
                row: self.row + 1,
                col: self.col,
            },
            Direction::Left if self.col > 0 => Coord {
                row: self.row,
                col: self.col - 1,
            },
            Direction::Right if self.col < (cols - 1) => Coord {
                row: self.row,
                col: self.col + 1,
            },
            _ => return None,
        };
        Some(coord)
    }
}

pub fn part_1(file: &str) -> Option<u32> {
           let grid = file
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        c.to_digit(10).unwrap().try_into().unwrap()
                    })
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

    let goal = Coord {
        row: grid.len() - 1,
        col: grid[0].len() - 1,
    };

    let mut pq = BinaryHeap::new();
    let mut seen = HashSet::new();

    let right = Crucible {
        cost: grid[0][1] as u32,
        dir: Direction::Right,
        pos: Coord { row: 0, col: 1 },
        moves_in_dir: 1,
    };
    pq.push(right);

    let down = Crucible {
        cost: grid[1][0] as u32,
        dir: Direction::Down,
        pos: Coord { row: 1, col: 0 },
        moves_in_dir: 1,
    };
    pq.push(down);

    while let Some(crucible) = pq.pop() {
        if crucible.pos == goal {
            return Some(crucible.cost);
        }
        for crucible in crucible.successors(&grid) {
            if seen.insert((crucible.pos, crucible.dir, crucible.moves_in_dir)) {
                pq.push(crucible);
            }
        }
    }

    panic!("No path found")
}

pub fn part_2(file: &str) -> Option<usize> {
    Some(0)
}
