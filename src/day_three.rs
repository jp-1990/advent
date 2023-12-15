use std::collections::VecDeque;

fn parse_line(line: &str) -> Vec<char> {
    line.chars().collect::<Vec<char>>()
}

fn walk_left(char_vec: &Vec<char>, idx: usize) -> String {
    let mut end = false;
    let mut left_idx = idx;

    let mut num = String::from("");
    while end == false {
        match char_vec[left_idx] {
            x if Option::is_some(&x.to_digit(10)) => {
                num = String::from(x) + &num;
                if left_idx == 0 {
                    break;
                };
                left_idx = left_idx - 1;
            }
            _ => end = true,
        };
    }
    num
}

fn walk_right(char_vec: &Vec<char>, idx: usize) -> String {
    let mut end = false;
    let mut right_idx = idx;

    let mut num = String::from("");
    while end == false {
        match char_vec[right_idx] {
            x if Option::is_some(&x.to_digit(10)) => {
                num = num + &String::from(x);
                if right_idx == char_vec.len() - 1 {
                    break;
                };
                right_idx = right_idx + 1;
            }
            _ => end = true,
        };
    }
    num
}

fn find_numbers(window: &VecDeque<Vec<char>>, idx: usize) -> Vec<u32> {
    let top = window.get(0).unwrap();
    let mid = window.get(1).unwrap();
    let bot = window.get(2).unwrap();

    let mut top_nums: String = String::from("");
    let mut mid_nums: String = String::from("");
    let mut bot_nums: String = String::from("");

    for n in -1isize..2 {
        let search_idx = n.checked_add(idx as isize);
        let search_idx: usize = match search_idx {
            Some(x) => x as usize,
            None => continue,
        };
        if search_idx >= top.len() {
            continue;
        }

        match top[search_idx] {
            x if Option::is_some(&x.to_digit(10)) => {
                if n == -1 {
                    let left_chars = walk_left(&top, idx - 1);
                    top_nums = left_chars + &top_nums;
                }
                if n == 0 {
                    let middle_char = String::from(x);
                    top_nums = top_nums + &middle_char;
                }
                if n == 1 {
                    let right_chars = walk_right(&top, idx + 1);
                    top_nums = top_nums + &right_chars;
                }
            }
            _ => top_nums = top_nums + ":",
        };

        match mid[search_idx] {
            x if Option::is_some(&x.to_digit(10)) => {
                if n == -1 {
                    let left_chars = walk_left(&mid, idx - 1);
                    mid_nums = left_chars + &mid_nums;
                }
                if n == 0 {
                    let middle_char = String::from(x);
                    mid_nums = mid_nums + &middle_char;
                }
                if n == 1 {
                    let right_chars = walk_right(&mid, idx + 1);
                    mid_nums = mid_nums + &right_chars;
                }
            }
            _ => mid_nums = mid_nums + ":",
        };

        match bot[search_idx] {
            x if Option::is_some(&x.to_digit(10)) => {
                if n == -1 {
                    let left_chars = walk_left(&bot, idx - 1);
                    bot_nums = left_chars + &bot_nums;
                }
                if n == 0 {
                    let middle_char = String::from(x);
                    bot_nums = bot_nums + &middle_char;
                }
                if n == 1 {
                    let right_chars = walk_right(&bot, idx + 1);
                    bot_nums = bot_nums + &right_chars;
                }
            }
            _ => bot_nums = bot_nums + ":",
        };
    }

    let mut nums = vec![];
    for val in top_nums.split(':').collect::<Vec<&str>>() {
        match val {
            "" => (),
            _ => nums.push(val.parse::<u32>().unwrap()),
        }
    }
    for val in mid_nums.split(':').collect::<Vec<&str>>() {
        match val {
            "" => (),
            _ => nums.push(val.parse::<u32>().unwrap()),
        }
    }
    for val in bot_nums.split(':').collect::<Vec<&str>>() {
        match val {
            "" => (),
            _ => nums.push(val.parse::<u32>().unwrap()),
        }
    }

    nums
}

pub fn part_1(file: &str) -> Option<u32> {
    let mut window = VecDeque::new();

    let mut find_part_numbers = |line: &str| -> Vec<u32> {
        let cur_line = parse_line(line);

        let mut should_pop_front = false;
        let mut ready = false;

        window.push_back(cur_line);
        match window.len() {
            3 => ready = true,
            4 => {
                should_pop_front = true;
                ready = true
            }
            _ => (),
        }

        if should_pop_front == true {
            window.pop_front();
        }

        let mut found_numbers: Vec<u32> = vec![];
        if ready == true {
            for (i, ch) in window.get(1).unwrap().iter().enumerate() {
                let symbol_idx = match ch {
                    x if x.is_digit(10) => Option::None,
                    '.' => Option::None,
                    _ => Option::Some(i),
                };

                match symbol_idx {
                    Some(x) => {
                        found_numbers.append(&mut find_numbers(&window, x));
                    }
                    None => (),
                };
            }
        };
        found_numbers
    };

    let mut lines = file.lines().collect::<Vec<&str>>();
    let first_line = (0..lines[0].len())
        .map(|_| ".")
        .collect::<Vec<&str>>()
        .join("");
    let last_line = (0..lines[0].len())
        .map(|_| ".")
        .collect::<Vec<&str>>()
        .join("");
    lines.append(&mut vec![&last_line]);

    let mut merged_lines = vec![&first_line[..]];
    merged_lines.extend(&lines);

    let mut nums = vec![];
    for line in merged_lines {
        let mut nums_for_line = find_part_numbers(line);
        nums.append(&mut nums_for_line);
    }

    Some(nums.into_iter().sum::<u32>())
}

pub fn part_2(file: &str) -> Option<u32> {
    let mut window = VecDeque::new();

    let mut find_part_numbers = |line: &str| -> Vec<u32> {
        let cur_line = parse_line(line);

        let mut should_pop_front = false;
        let mut ready = false;

        window.push_back(cur_line);
        match window.len() {
            3 => ready = true,
            4 => {
                should_pop_front = true;
                ready = true
            }
            _ => (),
        }

        if should_pop_front == true {
            window.pop_front();
        }

        let mut found_numbers: Vec<u32> = vec![];
        if ready == true {
            for (i, ch) in window.get(1).unwrap().iter().enumerate() {
                let symbol_idx = match ch {
                    x if x.is_digit(10) => Option::None,
                    '.' => Option::None,
                    '*' => Option::Some(i),
                    _ => Option::None,
                };

                match symbol_idx {
                    Some(x) => {
                        let nums = find_numbers(&window, x);
                        if nums.len() == 2 {
                            let res = nums[0]*nums[1];
                            found_numbers.push(res);
                        }
                    }
                    None => (),
                };
            }
        };
        found_numbers
    };

    let mut lines = file.lines().collect::<Vec<&str>>();
    let first_line = (0..lines[0].len())
        .map(|_| ".")
        .collect::<Vec<&str>>()
        .join("");
    let last_line = (0..lines[0].len())
        .map(|_| ".")
        .collect::<Vec<&str>>()
        .join("");
    lines.append(&mut vec![&last_line]);

    let mut merged_lines = vec![&first_line[..]];
    merged_lines.extend(&lines);

    let mut nums = vec![];
    for line in merged_lines {
        let mut nums_for_line = find_part_numbers(line);
        nums.append(&mut nums_for_line);
    }
    println!("{:?}", nums);

    Some(nums.into_iter().sum::<u32>())
}
