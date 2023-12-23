fn parse_file(file: &str) -> Vec<&str> {
    file.split([',', '\n'])
        .filter(|code| code.len() > 0)
        .collect::<Vec<&str>>()
}

fn calc_value(ch: char, value: usize) -> usize {
    let mut cur_val = value;
    let ascii_val = ch as usize;

    cur_val += ascii_val;
    cur_val *= 17;
    cur_val %= 256;

    cur_val
}

pub fn part_1(file: &str) -> Option<usize> {
    let codes = parse_file(file);

    let sum: usize = codes
        .into_iter()
        .map(|code| code.chars().fold(0, |acc, ch| calc_value(ch, acc)))
        .sum();

    Some(sum)
}

#[derive(Debug)]
struct LenseBox {
    contents: Vec<(String, u32)>,
}
impl LenseBox {
    pub fn new() -> LenseBox {
        LenseBox { contents: vec![] }
    }
}

pub fn part_2(file: &str) -> Option<usize> {
    let codes = parse_file(file);

    let mut boxes: Vec<LenseBox> = vec![];
    for _ in 0..256 {
        boxes.push(LenseBox::new());
    }

    codes.into_iter().for_each(|code| {
        let op_idx = code.find(['=', '-']).unwrap();
        let code_chars = code.chars().collect::<Vec<char>>();
        let op = code_chars[op_idx];

        let mut label_chars: Vec<char> = vec![];
        for i in 0..op_idx {
            label_chars.push(code_chars[i]);
        }

        let box_index = label_chars.iter().fold(0, |acc, ch| calc_value(*ch, acc));
        let lense_label = label_chars.iter().collect::<String>();

        match op {
            '=' => {
                let focal = code_chars[op_idx + 1].to_digit(10).unwrap();
                let lense_index = boxes[box_index]
                    .contents
                    .clone()
                    .into_iter()
                    .position(|(label, _)| label == lense_label);
                match lense_index {
                    Some(i) => {
                        boxes[box_index].contents[i].1 = focal;
                    }
                    None => {
                        boxes[box_index].contents.push((lense_label, focal));
                    }
                }
            }
            '-' => {
                boxes[box_index].contents = boxes[box_index]
                    .contents
                    .clone()
                    .into_iter()
                    .filter(|(label, _)| label != &lense_label)
                    .collect();
            }
            _ => (),
        };
    });

    let sum = boxes.into_iter().enumerate().map(|(i, b)| {
        let n = i + 1;
        let box_sum: usize = b
            .contents
            .into_iter()
            .enumerate()
            .map(|(j, val)| -> usize {
                let focal: usize = val.1.try_into().unwrap();
                n * (j+1) * focal
            })
            .sum();

        box_sum
    }).sum();

    Some(sum)
}
