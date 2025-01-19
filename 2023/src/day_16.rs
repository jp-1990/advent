fn parse_file(file: &str) -> Vec<Vec<char>> {
    file.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Cell {
    coord: Coord,
    symbol: char,
    direction: Direction,
    max_x: usize,
    max_y: usize,
}

impl Cell {
    pub fn new(
        coord: Coord,
        symbol: char,
        direction: Direction,
        max_x: usize,
        max_y: usize,
    ) -> Cell {
        Cell {
            coord,
            direction,
            symbol,
            max_y,
            max_x,
        }
    }

    fn check_bounds(&self, direction: Direction) -> Option<Coord> {
        match direction {
            Direction::N if self.coord.y > 0 => Some(Coord {
                x: self.coord.x,
                y: self.coord.y - 1,
            }),
            Direction::E if self.max_x > self.coord.x => Some(Coord {
                x: self.coord.x + 1,
                y: self.coord.y,
            }),
            Direction::S if self.max_y > self.coord.y => Some(Coord {
                x: self.coord.x,
                y: self.coord.y + 1,
            }),
            Direction::W if self.coord.x > 0 => Some(Coord {
                x: self.coord.x - 1,
                y: self.coord.y,
            }),
            _ => None,
        }
    }

    fn next(&self) -> Vec<(Coord, Direction)> {
        let mut directions: Vec<Direction> = vec![];

        match self.symbol {
            '|' => match self.direction {
                Direction::N => directions.push(self.direction),
                Direction::E => {
                    directions.push(Direction::N);
                    directions.push(Direction::S)
                }
                Direction::S => directions.push(self.direction),
                Direction::W => {
                    directions.push(Direction::N);
                    directions.push(Direction::S)
                }
            },
            '-' => match self.direction {
                Direction::N => {
                    directions.push(Direction::E);
                    directions.push(Direction::W)
                }
                Direction::E => directions.push(self.direction),
                Direction::S => {
                    directions.push(Direction::E);
                    directions.push(Direction::W)
                }
                Direction::W => directions.push(self.direction),
            },
            '\\' => match self.direction {
                Direction::N => directions.push(Direction::W),
                Direction::E => directions.push(Direction::S),
                Direction::S => directions.push(Direction::E),
                Direction::W => directions.push(Direction::N),
            },
            '/' => match self.direction {
                Direction::N => directions.push(Direction::E),
                Direction::E => directions.push(Direction::N),
                Direction::S => directions.push(Direction::W),
                Direction::W => directions.push(Direction::S),
            },
            _ => directions.push(self.direction),
        };

        let mut result = vec![];
        directions
            .into_iter()
            .for_each(|dir| match self.check_bounds(dir) {
                Some(coord) => result.push((coord, dir)),
                None => (),
            });

        result
    }
}

fn walk(
    coord: Coord,
    direction: Direction,
    matrix: &Vec<Vec<char>>,
    seen: &mut Vec<Coord>,
) -> Vec<Coord> {
    let max_x = matrix[0].len() - 1;
    let max_y = matrix.len() - 1;

    let ch = matrix[coord.y][coord.x];
    let cell = Cell::new(coord, ch, direction, max_x, max_y);

    let cells = cell.next();

    match cell.symbol {
        '|' => seen.push(cell.coord),
        '-' => seen.push(cell.coord),
        _ => (),
    }

    let mut walked: Vec<Coord> = vec![];
    walked.push(cell.coord);

    for i in 0..cells.len() {
        let mut matched = false;
        for j in 0..seen.len() {
            if seen[j].x == cells[i].0.x && seen[j].y == cells[i].0.y {
                matched = true;
            }
        }
        if matched {
            continue;
        };

        let mut result = walk(cells[i].0, cells[i].1, matrix, seen);
        walked.append(&mut result);
    }

    walked
}

pub fn part_1(file: &str) -> Option<usize> {
    let matrix = parse_file(file);
    let mut seen: Vec<Coord> = vec![];

    let start_coord = Coord { x: 0, y: 0 };
    let start_direction = Direction::E;

    let result = walk(start_coord, start_direction, &matrix, &mut seen);

    let mut energized: Vec<Coord> = vec![];
    for i in 0..result.len() {
        let mut duplicate = false;
        for j in 0..energized.len() {
            if energized[j].x == result[i].x && energized[j].y == result[i].y {
                duplicate = true
            };
        }
        if duplicate == true {
            continue;
        }
        energized.push(result[i]);
    }

    Some(energized.len())
}

pub fn part_2(file: &str) -> Option<usize> {
    let matrix = parse_file(file);

    let height = matrix.len();
    let width = matrix[0].len();

    let mut results: Vec<usize> = vec![];

    for i in 0..width {
        let mut seen: Vec<Coord> = vec![];

        let start_coord = Coord { x: i, y: 0 };
        let start_direction = Direction::S;

        let result = walk(start_coord, start_direction, &matrix, &mut seen);

        let mut energized: Vec<Coord> = vec![];

        for i in 0..result.len() {
            let mut duplicate = false;
            for j in 0..energized.len() {
                if energized[j].x == result[i].x && energized[j].y == result[i].y {
                    duplicate = true
                };
            }
            if duplicate == true {
                continue;
            }
            energized.push(result[i]);
        }

        results.push(energized.len());

        let mut seen: Vec<Coord> = vec![];

        let start_coord = Coord {
            x: i,
            y: height - 1,
        };
        let start_direction = Direction::N;

        let result = walk(start_coord, start_direction, &matrix, &mut seen);

        let mut energized: Vec<Coord> = vec![];

        for i in 0..result.len() {
            let mut duplicate = false;
            for j in 0..energized.len() {
                if energized[j].x == result[i].x && energized[j].y == result[i].y {
                    duplicate = true
                };
            }
            if duplicate == true {
                continue;
            }
            energized.push(result[i]);
        }

        results.push(energized.len());
    }

    for i in 0..height {
        let mut seen: Vec<Coord> = vec![];

        let start_coord = Coord { x: 0, y: i };
        let start_direction = Direction::E;

        let result = walk(start_coord, start_direction, &matrix, &mut seen);

        let mut energized: Vec<Coord> = vec![];

        for i in 0..result.len() {
            let mut duplicate = false;
            for j in 0..energized.len() {
                if energized[j].x == result[i].x && energized[j].y == result[i].y {
                    duplicate = true
                };
            }
            if duplicate == true {
                continue;
            }
            energized.push(result[i]);
        }

        results.push(energized.len());

        let mut seen: Vec<Coord> = vec![];

        let start_coord = Coord { x: width - 1, y: i };
        let start_direction = Direction::W;

        let result = walk(start_coord, start_direction, &matrix, &mut seen);

        let mut energized: Vec<Coord> = vec![];

        for i in 0..result.len() {
            let mut duplicate = false;
            for j in 0..energized.len() {
                if energized[j].x == result[i].x && energized[j].y == result[i].y {
                    duplicate = true
                };
            }
            if duplicate == true {
                continue;
            }
            energized.push(result[i]);
        }

        results.push(energized.len());
    }

    Some(results.into_iter().max().unwrap())
}
