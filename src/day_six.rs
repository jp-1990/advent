fn parse_file(file: &str) -> (Vec<u64>, Vec<u64>) {
    let lines: Vec<&str> = file
        .lines()
        .map(|line| line.split(':').collect::<Vec<&str>>()[1])
        .collect::<Vec<&str>>();

    let times = lines[0]
        .split_whitespace()
        .collect::<Vec<_>>()
        .into_iter()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let distances = lines[1]
        .split_whitespace()
        .collect::<Vec<_>>()
        .into_iter()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    (times, distances)
}

fn calc(time: u64, distance: u64) -> u64 {
    let range = 0..time;
    let mut a = 1;
    let mut b = time - 1;

    let mut results: Vec<u64> = vec![];

    for _ in range {
        let new_dist = a * b;
        if new_dist > distance {
            results.push(new_dist);
        };
        a = a + 1;
        if b >= 1 {
            b = b - 1;
        }
    }

    results.len().try_into().unwrap()
}

pub fn part_1(file: &str) -> Option<u64> {
    let (times, distances) = parse_file(file);


    let mut product = 1;
    for i in 0..times.len() {
        let result = calc(times[i], distances[i]);
        println!("{:?}", result);
        product = result * product;
    }

    Some(product)
}

pub fn part_2(file: &str) -> Option<u64> {
    let (times, distances) = parse_file(file);


    let mut product = 1;
    for i in 0..times.len() {
        let result = calc(times[i], distances[i]);
        println!("{:?}", result);
        product = result * product;
    }

    Some(product)
}
