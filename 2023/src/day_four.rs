use std::collections::HashMap;

fn split_line(line: &str) -> (HashMap<u32, bool>, Vec<u32>) {
    let res = line.split([':', '|']).collect::<Vec<&str>>();

    let target_nums = res[1]
        .split(' ')
        .filter(|n| n != &"")
        .map(|n| n.parse::<u32>().unwrap())
        .map(|n| (n, true))
        .collect::<HashMap<_, _>>();

    let input_nums = res[2]
        .split(' ')
        .filter(|n| n != &"")
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    (target_nums, input_nums)
}

fn card_matrix(file: &str) -> Vec<(HashMap<u32, bool>, Vec<u32>)> {
    let lines = file
        .lines()
        .map(split_line)
        .collect::<Vec<(HashMap<u32, bool>, Vec<u32>)>>();

    lines
}

fn process_line_1(tuple: (HashMap<u32, bool>, Vec<u32>)) -> u32 {
    let mut score: u32 = 0;
    let hash = tuple.0;
    let nums = tuple.1;

    for num in nums {
        match hash.get(&num) {
            Some(_) => score = score + 1,
            None => (),
        }
    }

    if score > 1 {
        score = score - 1;
        score = 2u32.pow(score);
    };

    score
}

fn process_line_2(tuple: &(HashMap<u32, bool>, Vec<u32>)) -> u32 {
    let mut score: u32 = 0;
    let hash = &tuple.0;
    let nums = &tuple.1;

    for num in nums {
        match hash.get(&num) {
            Some(_) => score = score + 1,
            None => (),
        }
    }

    score
}

pub fn part_1(file: &str) -> Option<u32> {
    let matrix = card_matrix(file);

    Some(matrix.into_iter().map(process_line_1).sum::<u32>())
}

pub fn part_2(file: &str) -> Option<u32> {
    let matrix = card_matrix(file);
    let mut copies: Vec<u32> = (0..matrix.len()).map(|_| 1u32).collect::<Vec<u32>>();

    for i in 0..matrix.len() {
        for _ in 0..copies[i] {
            let line_score = process_line_2(&matrix[i]) as usize;

            for k in 1usize..line_score + 1 {
                let target_idx = i + k;
                match target_idx {
                    x if x >= matrix.len() => continue,
                    _ => (),
                };
                copies[target_idx] = copies[target_idx] + 1;
            }
        }
    }

    Some(copies.into_iter().sum::<u32>())
}
