use std::collections::{hash_map::Entry, HashMap};

fn parse_file_1(file: &str) -> Vec<Vec<char>> {
    let lines = file.lines();

    let mut matrix = vec![];

    let mut counter: Vec<usize> = vec![];

    for (i, line) in lines.enumerate() {
        let chars = line.chars().collect::<Vec<char>>();

        if i == 0 {
            counter = vec![0; chars.len()]
        }

        for (j, ch) in chars.clone().into_iter().enumerate() {
            if ch == '#' {
                counter[j] = counter[j] + 1;
            }
        }

        if chars.iter().all(|ch| ch == &'.') {
            matrix.push(chars.clone());
            matrix.push(chars);
        } else {
            matrix.push(chars);
        }
    }

    matrix = matrix
        .clone()
        .into_iter()
        .map(|line| {
            let mut new_line: Vec<char> = vec![];
            for (i, ch) in line.into_iter().enumerate() {
                match counter[i] {
                    n if n == 0 => {
                        new_line.push('.');
                        new_line.push('.');
                    }
                    _ => new_line.push(ch),
                }
            }
            new_line
        })
        .collect::<Vec<Vec<char>>>();

    matrix
}

fn parse_file_2(file: &str) -> (Vec<Vec<char>>, HashMap<char, Vec<usize>>) {
    let lines = file.lines().collect::<Vec<&str>>();

    let mut matrix = vec![];

    let mut x_counter: Vec<usize> = vec![0; lines[0].len()];
    let mut y_counter: Vec<usize> = vec![0; lines.len()];

    for (i, line) in lines.into_iter().enumerate() {
        let chars = line.chars().collect::<Vec<char>>();

        for j in 0..chars.len() {
            if chars[j] == '#' {
                x_counter[j] = x_counter[j] + 1;
            }
        }

        if !chars.iter().all(|ch| ch == &'.') {
            y_counter[i] = y_counter[i] + 1;
        }
        matrix.push(chars);
    }

    let mut xy_hash: HashMap<char, Vec<usize>> = HashMap::new();
    for i in 0..x_counter.len() {
        if x_counter[i] == 0 {
            match xy_hash.entry('x') {
                Entry::Vacant(e) => {
                    e.insert(vec![i]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(i);
                }
            }
        }
    }
    for i in 0..y_counter.len() {
        if y_counter[i] == 0 {
            match xy_hash.entry('y') {
                Entry::Vacant(e) => {
                    e.insert(vec![i]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(i);
                }
            }
        }
    }

    (matrix, xy_hash)
}

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

fn find_galaxies(matrix: Vec<Vec<char>>) -> HashMap<usize, Coord> {
    let mut galaxies: HashMap<usize, Coord> = HashMap::new();

    let mut n: usize = 0;

    for (i, y_axis) in matrix.into_iter().enumerate() {
        let y = i;

        for (j, x_axis) in y_axis.into_iter().enumerate() {
            let x = j;

            if x_axis == '#' {
                galaxies.insert(n, Coord { x, y });
                n = n + 1;
            }
        }
    }

    galaxies
}

fn calc_distance(a: &Coord, b: &Coord, xy_hash: &HashMap<char, Vec<usize>>)->usize {
    let drfit = 999_999;

    let mut sum = 0;
    match a.x {
        x if x > b.x => {
            let gap = b.x..x + 1;
            for i in xy_hash.get(&'x').unwrap() {
                if gap.contains(&i) {
                    sum = sum + drfit;
                }
            }
            sum = sum + (x - b.x);
        }
        x => {
            let gap = x..b.x + 1;
            for i in xy_hash.get(&'x').unwrap() {
                if gap.contains(&i) {
                    sum = sum + drfit;
                }
            }
            sum = sum + (b.x - x)
        }
    }

    match a.y {
        y if y > b.y =>{
            let gap = b.y..y + 1;
            for i in xy_hash.get(&'y').unwrap() {
                if gap.contains(&i) {
                    sum = sum + drfit;
                }
            }
            sum = sum + (y - b.y);
        },
        y =>{
            let gap = y..b.y + 1;
            for i in xy_hash.get(&'y').unwrap() {
                if gap.contains(&i) {
                    sum = sum + drfit;
                }
            }

            sum = sum + (b.y - y);
        }
    };

    sum
}

pub fn part_1(file: &str) -> Option<usize> {
    let parsed_file = parse_file_1(file);

    let galaxies = find_galaxies(parsed_file);
    let range = Vec::from_iter(0..galaxies.keys().len());

    let mut steps = vec![];

    for n in range.clone() {
        let galaxy = galaxies.get(&n).unwrap();
        for i in range[n + 1..].into_iter() {
            let target = galaxies.get(&i).unwrap();

            let mut sum = 0;
            match galaxy.x {
                x if x > target.x => sum = sum + (x - target.x),
                x => sum = sum + (target.x - x),
            }
            match galaxy.y {
                y if y > target.y => sum = sum + (y - target.y),
                y => sum = sum + (target.y - y),
            }

            steps.push(sum);
        }
    }

    let sum_steps: usize = steps.into_iter().sum();

    Some(sum_steps)
}

pub fn part_2(file: &str) -> Option<usize> {
    let (parsed_file, xy_hash) = parse_file_2(file);

    let galaxies = find_galaxies(parsed_file);
    let range = Vec::from_iter(0..galaxies.keys().len());

    let mut steps = vec![];

    for n in range.clone() {
        let galaxy = galaxies.get(&n).unwrap();
        for i in range[n + 1..].into_iter() {
            let target = galaxies.get(&i).unwrap();

            let sum = calc_distance(galaxy, target, &xy_hash);

            steps.push(sum);
        }
    }

    let sum_steps: usize = steps.into_iter().sum();

    Some(sum_steps)
}
