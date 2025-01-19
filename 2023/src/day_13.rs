#[derive(Debug)]
struct Pattern {
    rows: Vec<Vec<char>>,
    columns: Vec<Vec<char>>,
}

#[derive(Debug)]
struct Res {
    index: usize,
    iterations: usize,
}

fn parse_file(file: &str) -> Vec<Pattern> {
    let lines = file.lines().collect::<Vec<&str>>();
    let mut patterns: Vec<Pattern> = vec![];

    let mut pattern = Pattern {
        rows: vec![],
        columns: vec![vec![]; lines[0].len()],
    };

    for i in 0..lines.len() {
        match lines[i] {
            l if l.len() == 0 => {
                patterns.push(pattern);
                pattern = Pattern {
                    rows: vec![],
                    columns: vec![vec![]; lines[i+1].len()],
                }
            }
            _ => {
                let line: Vec<char> = lines[i].chars().collect();

                for j in 0..line.len() {
                    pattern.columns[j].push(line[j]);
                }

                pattern.rows.push(line);
            }
        }
    }

    patterns.push(pattern);

    patterns
}

fn walk(row: &Vec<char>, mut next: usize, mut prev: usize) -> (bool, usize, usize) {
    let min_prev = 0;
    let max_next = row.len() - 1;

    if next == max_next || prev == min_prev {
        return (false, next, prev);
    }

    let a = row[prev];
    let b = row[next];

    if a == b {
        next = next + 1;
        prev = prev - 1;
        (true, next, prev)
    } else {
        (false, next, prev)
    }
}

fn parse_record(row: &Vec<char>) -> Res {
    let mut result = Res {
        index: 0,
        iterations: 0,
    };

    for (i, _) in row.windows(2).enumerate() {
        let mut equal = true;
        let mut next_i = i + 1;
        let mut prev_i = i;
        let mut counter = 0;

        while equal {
            (equal, next_i, prev_i) = walk(&row, next_i, prev_i);
            if equal == true {
                counter = counter + 1;
            }
        }

        if result.iterations < counter {
            result.index = i;
            result.iterations = counter;
        }
    }

    result
}

pub fn part_1(file: &str) -> Option<usize> {
    let patterns = parse_file(file);

    let sum:usize =patterns.iter().map(|pattern| {
        let mut row_result = Res {
            index: 0,
            iterations: 0,
        };
        let mut col_result = Res {
            index: 0,
            iterations: 0,
        };

        for i in 0..pattern.rows.len() {
            let row_res = parse_record(&pattern.rows[i]);

            if row_result.iterations > 0 && row_res.iterations != row_result.iterations {
                row_result.index = 0;
                row_result.iterations = 0;
                break;
            }

            row_result = row_res;
        }

        for i in 0..pattern.columns.len() {
            let col_res = parse_record(&pattern.columns[i]);

            if col_result.iterations > 0 && col_res.iterations != col_result.iterations {
                col_result.index = 0;
                col_result.iterations = 0;
                break;
            }

            col_result = col_res;
        }

        let  pattern_value:usize;
        if row_result.iterations > col_result.iterations {
            pattern_value = row_result.index +1;
            
        } else {
            pattern_value = (col_result.index +1) * 100;
        }

        pattern_value
    }).sum();

    Some(sum)
}

pub fn part_2(file: &str) -> Option<usize> {
    Some(0)
}
