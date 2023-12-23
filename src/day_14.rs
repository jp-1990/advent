fn parse_file(file: &str) -> Vec<Vec<usize>>{
    let lines = file.lines().collect::<Vec<&str>>();

    let mut stacks: Vec<Vec<usize>> = vec![vec![]; lines.len()];

    for i in 0..lines.len() {
        let cols = lines[i].chars();

        cols.enumerate().for_each(|(j, col)| match col {
            'O' => {
                let col_len = stacks[j].len();
                stacks[j].push(lines.len() - col_len);
            }
            '#' => {
                let col_len = stacks[j].len();
                stacks[j].append(&mut vec![0; (i - col_len) + 1]);
            }
            _ => (),
        });
    }

    stacks
}

pub fn part_1(file: &str) -> Option<usize> {
    let parsed_file = parse_file(file);

    let sum = parsed_file.iter().map(|stack|stack.iter().sum::<usize>()).sum();

    Some(sum)
}

pub fn part_2(file: &str) -> Option<usize> {
    Some(0)
}
