use std::collections::HashMap;

fn parse_file(file: &str) -> (Vec<char>, HashMap<&str, [&str; 2]>) {
    let mut lines = file.lines();

    let instructions = lines.next().unwrap().chars().collect::<Vec<char>>();
    lines.next();

    let puzzle = lines
        .map(|line| {
            let split_line = line.split(" = ").collect::<Vec<&str>>();

            let key = split_line[0];
            let str_values = split_line[1];

            let values = &str_values[1..&str_values.len() - 1]
                .split(", ")
                .collect::<Vec<&str>>();

            (key, [values[0], values[1]])
        })
        .collect::<HashMap<&str, [&str; 2]>>();

    (instructions, puzzle)
}

fn parse_file_2(file: &str) -> (Vec<char>, Vec<&str>, HashMap<&str, [&str; 2]>) {
    let mut lines = file.lines();

    let instructions = lines.next().unwrap().chars().collect::<Vec<char>>();
    lines.next();

    let mut start_keys: Vec<&str> = vec![];

    let puzzle = lines
        .map(|line| {
            let split_line = line.split(" = ").collect::<Vec<&str>>();

            let key = split_line[0];
            let str_values = split_line[1];

            if &key[2..] == "A" {
                start_keys.push(&key)
            };

            let values = &str_values[1..&str_values.len() - 1]
                .split(", ")
                .collect::<Vec<&str>>();

            (key, [values[0], values[1]])
        })
        .collect::<HashMap<&str, [&str; 2]>>();

    (instructions, start_keys, puzzle)
}

fn no_matching_end_keys(keys: &Vec<&str>) -> bool {
    keys.into_iter().any(|key| &key[2..] != "Z")
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u128, b: u128) -> u128 {
    if a == 0 || b == 0 {
        0
    } else {
        (a / gcd(a, b)) * b
    }
}

fn find_lcm_of_set(numbers: &[u128]) -> u128 {
    if numbers.is_empty() {
        return 0;
    }

    let mut result = numbers[0];

    for &num in &numbers[1..] {
        result = lcm(result, num);
    }

    result
}

pub fn part_1(file: &str) -> Option<u32> {
    let (instructions, puzzle) = parse_file(file);

    let mut key: &str = "AAA";
    let mut steps: u32 = 0;

    while key != "ZZZ" {
        for direction in &instructions {
            key = match direction {
                'L' => puzzle.get(key).unwrap()[0],
                'R' => puzzle.get(key).unwrap()[1],
                _ => key,
            };

            steps = steps + 1;
        }
    }

    Some(steps)
}

pub fn part_2(file: &str) -> Option<u128> {
    let (instructions, start_keys, puzzle) = parse_file_2(file);

    let mut keys: Vec<&str> = start_keys.clone();
    let mut steps_per_key: Vec<u128> = vec![];

    for i in 0..start_keys.len() {
        let mut steps: u128 = 0;

        while &keys[i][2..] != "Z" {
            for direction in &instructions {
                keys[i] = match direction {
                    'L' => puzzle.get(keys[i]).unwrap()[0],
                    'R' => puzzle.get(keys[i]).unwrap()[1],
                    _ => keys[i],
                };

                steps = steps + 1;

                if &keys[i][2..] == "Z" {
                    break;
                };
            }
        }

        steps_per_key.push(steps);
    }

    Some(find_lcm_of_set(&steps_per_key).into())

    // let max = &steps_per_key.clone().into_iter().max().unwrap();
    // let mut max_steps = *max;
    // let mut counter = steps_per_key.clone();
    //
    // let mut done = counter.clone().into_iter().all(|step| step == max_steps);
    //
    // while done == false {
    //     for i in 0..steps_per_key.len() {
    //         if &counter[i] < &max_steps {
    //             counter[i] = counter[i] + steps_per_key[i];
    //         }
    //         if &counter[i] > &max_steps {
    //             max_steps = max_steps + *max;
    //         }
    //     }
    //     done = counter.clone().into_iter().all(|step| step == max_steps);
    // }
    //
    // Some(counter[0])

}
