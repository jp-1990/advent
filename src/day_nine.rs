fn parse_file(file: &str) -> Vec<Vec<i32>> {
    file.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

fn next_line(line: &Vec<i32>) -> Vec<i32> {
    let mut new_line: Vec<i32> = vec![];

    for pair in line.windows(2) {
        new_line.push(pair[1] - pair[0]);
    }

    new_line
}

fn parse_line_1(line: Vec<i32>) -> i32 {
    let mut lines: Vec<Vec<i32>> = vec![];
    lines.push(line);

    loop {
        let lines_len = lines.len();
        let new_line = next_line(&lines[lines_len - 1]);

        lines.push(new_line.clone());
        if new_line[..].into_iter().all(|n| n == &0) {
            break;
        };
    }

    let mut nums: Vec<i32> = vec![];
    for line in lines {
        let n = line.last().unwrap();
        nums.push(*n);
    }

    nums.into_iter().sum()
}

fn parse_line_2(line: Vec<i32>) -> i32 {
    let mut lines: Vec<Vec<i32>> = vec![];
    lines.push(line);

    loop {
        let lines_len = lines.len();
        let new_line = next_line(&lines[lines_len - 1]);

        lines.push(new_line.clone());
        if new_line[..].into_iter().all(|n| n == &0) {
            break;
        };
    }

    let mut nums: Vec<i32> = vec![];
    let mut prev:i32 = 0;

    lines.reverse();
    for line in lines {
        let n = line[0];
        nums.push(n - prev);
        prev = n - prev;
    }

*nums.last().unwrap()
}

pub fn part_1(file: &str) -> Option<i32> {
    let lines = parse_file(file);

    let sum = lines.into_iter().map(|line| parse_line_1(line)).sum();

    Some(sum)
}

pub fn part_2(file: &str) -> Option<i32> {
    let lines = parse_file(file);

    let sum = lines.into_iter().map(|line| parse_line_2(line)).sum();

    Some(sum)
}
