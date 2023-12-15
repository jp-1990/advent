pub fn part_1(file: &str) -> Option<u32> {
    let lines = &file.split("\n").collect::<Vec<&str>>();

    let mut sum: u32 = 0;

    for line in lines {
        let mut digits: Vec<char> = vec![];
        for char in line.chars() {
            if char.is_digit(10) {
                digits.push(char);
            };
        }

        if digits.len() == 0 {
            continue;
        }
        let first_digit = digits.first().unwrap();
        let last_digit = digits.last().unwrap();
        let number = format!("{}{}", first_digit, last_digit)
            .parse::<u32>()
            .unwrap();

        sum = sum + number;
    }

    Some(sum)
}

pub fn part_2(file: &str) -> Option<u32> {
    const LUT: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    fn parse_line_2(line: &str) -> u32 {
        let first = find_pattern(0..line.len(), line);
        let last = find_pattern((0..line.len()).rev(), line);
        10 * first + last
    }

    fn find_pattern(mut it: impl Iterator<Item = usize>, line: &str) -> u32 {
        it.find_map(|i| compare_slice(&line[i..])).unwrap()
    }

    fn compare_slice(slice: &str) -> Option<u32> {
        LUT.iter()
            .enumerate()
            .find(|(_, pattern)| slice.starts_with(*pattern))
            .map(|(i, _)| i as u32 + 1)
            .or_else(|| {
                let x = slice.chars().next().unwrap().to_digit(10);
                x
            })
    }

    Some(
        file.lines()
            .map(|x| {
                let res = parse_line_2(x);
                res
            })
            .sum::<u32>(),
    )
}
