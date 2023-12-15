fn parse_file(file: &str) -> Vec<(Vec<char>, Vec<usize>)> {
    file.lines()
        .map(|line| {
            let split_line = line.split(" ").collect::<Vec<&str>>();

            let puzzle = split_line[0];
            let values = split_line[1];

            (
                puzzle.chars().collect::<Vec<char>>(),
                values
                    .split(',')
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            )
        })
        .collect()
}

fn char_group(springs: &Vec<char>, index: usize, group_size: usize) -> (Vec<char>, bool) {
    let mut group: Vec<char> = vec![];

    let max_i = springs.len() - 1;
    let max_group_i = index + group_size - 1;

    for i in index..index + group_size {
        match springs[i] {
            '#' if i == max_group_i && i < max_i => match springs[i + 1] {
                '#' => {
                    group = vec![];
                    break;
                }
                '.' => group.push('#'),
                '?' => group.push('#'),
                _ => (),
            },
            '#' => group.push('#'),
            '.' => {
                group = vec!['.'; group.len()];
                break;
            }
            '?' if i == max_group_i && i < max_i => match springs[i + 1] {
                '#' => {
                    group = vec![];
                    break;
                }
                '.' => group.push('#'),
                '?' => group.push('#'),
                _ => (),
            },
            '?' => group.push('#'),
            _ => (),
        }

        print!("{:?}\n", group);

        if i == max_i {
            break;
        };
    }

    let matched_group = group.len() > 0 && group.iter().all(|&ch| ch == '#');

    if max_i != max_group_i {
        group.push('.');
    }

    print!("{:?}{:?}\n", group, group_size);
    (group, matched_group)
}

fn parse_record(record: (Vec<char>, Vec<usize>)) -> Vec<char> {
    let mut springs_i = 0;
    let mut groups_i = 0;

    let springs = record.0;
    let groups = record.1;

    let mut pattern: Vec<char> = vec![];

    while springs_i != springs.len() {
        if groups_i == groups.len() {
            pattern.push('.');
            springs_i = springs_i + 1;
            continue;
        };

        let (mut ch_group, matched) = char_group(&springs, springs_i, groups[groups_i]);

        springs_i = springs_i + ch_group.len();
        if matched == true {
            groups_i = groups_i + 1;
        };

        pattern.append(&mut ch_group);
    }

    print!("{:?}:{:?}\n\n", pattern, groups);

    pattern
}

pub fn part_1(file: &str) -> Option<usize> {
    let parsed_file = parse_file(file);

    let records = parsed_file
        .into_iter()
        .map(|line| parse_record(line))
        .collect::<Vec<Vec<char>>>();

    print!("{:?}\n", records);

    Some(0)
}

pub fn part_2(file: &str) -> Option<usize> {
    Some(0)
}
