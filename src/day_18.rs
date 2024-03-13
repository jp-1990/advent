use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    U,
    R,
    D,
    L,
}

#[derive(Debug,Copy,Clone,PartialEq, Eq,Hash)]
struct Position {
    x: i8,
    y: i8,
}

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    count: i8,
    color: String,
}

fn parse_file(file: &str) -> Vec<Instruction> {
    file.lines()
        .map(|line| {
            let split_line = line.split(' ').collect::<Vec<&str>>();
            let mut parsed_instruction: Instruction = Instruction {
                dir: Direction::U,
                count: 0,
                color: String::from(""),
            };

            match split_line.as_slice() {
                [dir, count, color] => {
                    let count: i8 = count.parse().unwrap();

                    let mut direction: Direction = Direction::U;
                    match dir {
                        &"U" => direction = Direction::U,
                        &"R" => direction = Direction::R,
                        &"D" => direction = Direction::D,
                        &"L" => direction = Direction::L,
                        _ => (),
                    };
                    let dir = direction;

                    let color: String = String::from(&color[1..color.len() - 1]);

                    parsed_instruction = Instruction { dir, count, color };
                }
                _ => (),
            }

            parsed_instruction
        })
        .collect::<Vec<_>>()
}

pub fn part_1(file: &str) -> Option<usize> {
    let instructions = parse_file(file);

    // y:x
    let mut edge: HashMap<Position, Position> = HashMap::new();
    let mut seen: HashMap<i8, i8> = HashMap::new();

    let mut pos = Position { x: 0, y: 0 };
    edge.insert(pos,pos);

    let mut y_total:i8 = 0;
    let mut x_total:i8=0;
    let len:i8 = instructions.len().try_into().unwrap();
    instructions
        .into_iter()
        .for_each(|instruction| match instruction.dir {
            Direction::U => {
                for _ in 0..instruction.count {
                    pos.y -= 1;
                    y_total += pos.y;
                    let coord = Position {x:pos.x,y:pos.y};
                    println!("{:?}",coord);
                    edge.insert(coord,coord);
                }
            }
            Direction::L => {
                for _ in 0..instruction.count {
                    pos.x -= 1;
                    x_total += pos.x;
                    let coord = Position {x:pos.x,y:pos.y};
                    println!("{:?}",coord);
                    edge.insert(coord,coord);
                }
            }
            Direction::D => {
                for _ in 0..instruction.count {
                    pos.y += 1;
                    y_total += pos.y;
                    let coord = Position {x:pos.x,y:pos.y};
                    println!("{:?}",coord);
                    edge.insert(coord,coord);
                }
            }
            Direction::R => {
                for _ in 0..instruction.count {
                    pos.x += 1;
                    x_total += pos.x;
                    let coord = Position {x:pos.x,y:pos.y};
                    println!("{:?}",coord);
                    edge.insert(coord,coord);
                }

            }
        });

    let mean_y = y_total/len;
    let mean_x = x_total/len;

    println!("{:?} {:?}",y_total,mean_y);
    println!("{:?} {:?}",x_total,mean_x);

    let center_pos = Position { x:mean_x,y:mean_y};

    // for (key, _value) in edge.into_iter() {
    //     println!("{:?}", key );
    // }
    //
    // println!();
    //
    // for (key, _value) in seen.into_iter() {
    //     println!("{:?}", key );
    // }

    Some(0)
}

pub fn part_2(file: &str) -> Option<usize> {
    Some(0)
}
