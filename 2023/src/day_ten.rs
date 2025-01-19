#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Node {
    position: Coord,
    value: char,
    valid_moves: (Coord, Coord),
}

impl Node {
    pub fn new(position: &Coord, value: char) -> Option<Self> {
        match value {
            '|' => Some(Node {
                position: Coord {
                    x: position.x,
                    y: position.y,
                },
                value,
                valid_moves: (
                    Coord {
                        x: position.x,
                        y: position.y + 1,
                    },
                    Coord {
                        x: position.x,
                        y: position.y - 1,
                    },
                ),
            }),
            '-' => Some(Node {
                position: Coord {
                    x: position.x,
                    y: position.y,
                },
                value,
                valid_moves: (
                    Coord {
                        x: position.x + 1,
                        y: position.y,
                    },
                    Coord {
                        x: position.x - 1,
                        y: position.y,
                    },
                ),
            }),
            'L' => Some(Node {
                position: Coord {
                    x: position.x,
                    y: position.y,
                },
                value,
                valid_moves: (
                    Coord {
                        x: position.x,
                        y: position.y - 1,
                    },
                    Coord {
                        x: position.x + 1,
                        y: position.y,
                    },
                ),
            }),
            'J' => Some(Node {
                position: Coord {
                    x: position.x,
                    y: position.y,
                },
                value,
                valid_moves: (
                    Coord {
                        x: position.x,
                        y: position.y - 1,
                    },
                    Coord {
                        x: position.x - 1,
                        y: position.y,
                    },
                ),
            }),
            '7' => Some(Node {
                position: Coord {
                    x: position.x,
                    y: position.y,
                },
                value,
                valid_moves: (
                    Coord {
                        x: position.x - 1,
                        y: position.y,
                    },
                    Coord {
                        x: position.x,
                        y: position.y + 1,
                    },
                ),
            }),
            'F' => Some(Node {
                position: Coord {
                    x: position.x,
                    y: position.y,
                },
                value,
                valid_moves: (
                    Coord {
                        x: position.x + 1,
                        y: position.y,
                    },
                    Coord {
                        x: position.x,
                        y: position.y + 1,
                    },
                ),
            }),
            _ => None,
        }
    }

    fn next(&self, prev: &Coord) -> &Coord {
        if self.valid_moves.0.x == prev.x && self.valid_moves.0.y == prev.y {
            &self.valid_moves.1
        } else {
            &self.valid_moves.0
        }
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn valid_matrix_access(
    matrix: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    direction: Direction,
) -> (char, Option<Coord>) {
    match direction {
        Direction::North if y > 0 => match matrix.get(y - 1) {
            Some(line) => match line.get(x) {
                Some(val) if val == &'|' => (*val, Some(Coord { x, y: y - 1 })),
                Some(val) if val == &'F' => (*val, Some(Coord { x, y: y - 1 })),
                Some(val) if val == &'7' => (*val, Some(Coord { x, y: y - 1 })),
                _ => ('.', None),
            },
            None => ('.', None),
        },
        Direction::North => ('.', None),
        Direction::South => match matrix.get(y + 1) {
            Some(line) => match line.get(x) {
                Some(val) if val == &'|' => (*val, Some(Coord { x, y: y + 1 })),
                Some(val) if val == &'L' => (*val, Some(Coord { x, y: y + 1 })),
                Some(val) if val == &'J' => (*val, Some(Coord { x, y: y + 1 })),
                _ => ('.', None),
            },
            None => ('.', None),
        },
        Direction::East => match matrix.get(y) {
            Some(line) => match line.get(x + 1) {
                Some(val) if val == &'-' => (*val, Some(Coord { x: x + 1, y })),
                Some(val) if val == &'J' => (*val, Some(Coord { x: x + 1, y })),
                Some(val) if val == &'7' => (*val, Some(Coord { x: x + 1, y })),
                _ => ('.', None),
            },
            None => ('.', None),
        },
        Direction::West => match matrix.get(y) {
            Some(line) if x > 0 => match line.get(x - 1) {
                Some(val) if val == &'-' => (*val, Some(Coord { x: x - 1, y })),
                Some(val) if val == &'L' => (*val, Some(Coord { x: x - 1, y })),
                Some(val) if val == &'F' => (*val, Some(Coord { x: x - 1, y })),
                _ => ('.', None),
            },
            Some(_) => ('.', None),
            None => ('.', None),
        },
    }
}

fn parse_file(file: &str) -> (Coord, Node, Node, Vec<Vec<char>>) {
    let mut x: Option<usize> = None;
    let mut y: Option<usize> = None;

    let matrix = file
        .lines()
        .map(|line| {
            let line_chars = line.chars().collect::<Vec<char>>();
            let start_x = line_chars.iter().position(|ch| ch == &'S');

            match y {
                Some(_) if Option::is_none(&x) => y = Some(y.unwrap() + 1),
                None => y = Some(0),
                _ => (),
            }

            match start_x {
                Some(idx) => x = Some(idx.try_into().unwrap()),
                None => (),
            }

            line_chars
        })
        .collect::<Vec<Vec<char>>>();

    let x_ = x.unwrap();
    let y_ = y.unwrap();

    let north = valid_matrix_access(&matrix, x_, y_, Direction::North);
    let east = valid_matrix_access(&matrix, x_, y_, Direction::East);
    let south = valid_matrix_access(&matrix, x_, y_, Direction::South);
    let west = valid_matrix_access(&matrix, x_, y_, Direction::West);

    let mut surrounding_nodes = vec![north, east, south, west]
        .into_iter()
        .filter(|point| point.0 != '.')
        .map(|point| Node::new(&point.1.unwrap(), point.0).unwrap())
        .collect::<Vec<Node>>();

    (
        Coord {
            x: x.unwrap(),
            y: y.unwrap(),
        },
        surrounding_nodes.remove(0),
        surrounding_nodes.remove(0),
        matrix,
    )
}

pub fn part_1(file: &str) -> Option<u32> {
    let (start_coord, _prev_node, next_node, matrix) = parse_file(file);

    let mut next = next_node;
    let mut prev_coord = Coord {
        x: start_coord.x,
        y: start_coord.y,
    };
    let mut steps = 1;

    while !(&next.position.y == &start_coord.y && &next.position.x == &start_coord.x) {
        let next_coord = next.next(&prev_coord);
        let next_char = matrix[next_coord.y][next_coord.x];

        steps = steps + 1;

        prev_coord = Coord {
            x: next.position.x,
            y: next.position.y,
        };
        next = match Node::new(next_coord, next_char) {
            Some(val) => val,
            None => break,
        };

    }

    println!("steps:{:?}", steps/2);

    Some(0)
}

pub fn part_2(file: &str) -> Option<u32> {
    Some(0)
}
