use std::collections::HashMap;

pub fn part_1(file: &str) -> Option<u32> {
    fn parse_line(line: &str) -> u32 {
        let mut min_values = HashMap::new();
        min_values.insert("red", 12);
        min_values.insert("green", 13);
        min_values.insert("blue", 14);

        let mut line_iter = line.split(": ");

        let game = line_iter.next();
        let mut invalid = false;

        let rounds = line_iter.next().unwrap().split("; ").collect::<Vec<&str>>();
        for round in &rounds {
            let cubes = round.split(", ").collect::<Vec<&str>>();

            for cube in &cubes {
                let mut key_value = cube.split(' ').collect::<Vec<&str>>();
                let key = key_value.pop().unwrap();
                let value = key_value.pop().unwrap().parse::<u32>().unwrap();

                match min_values.get(key) {
                    Some(x) if &value > x => invalid = true,
                    _ => (),
                };
            }
        }
        match invalid {
            false => game
                .unwrap()
                .split(' ')
                .collect::<Vec<&str>>()
                .pop()
                .unwrap()
                .parse::<u32>()
                .unwrap(),
            true => 0,
        }
    }

    Some(file.lines().map(parse_line).sum::<u32>())
}

pub fn part_2(file: &str) -> Option<u32> {
    fn parse_line(line: &str) -> u32 {
        let mut min_values = HashMap::new();
        min_values.insert("red", 0);
        min_values.insert("green", 0);
        min_values.insert("blue", 0);

        let mut line_iter = line.split(": ");
        line_iter.next();

        let rounds = line_iter.next().unwrap().split("; ").collect::<Vec<&str>>();
        for round in &rounds {
            let cubes = round.split(", ").collect::<Vec<&str>>();

            for cube in &cubes {
                let mut key_value = cube.split(' ').collect::<Vec<&str>>();
                let key = key_value.pop().unwrap();
                let value = key_value.pop().unwrap().parse::<u32>().unwrap();

                match min_values.get(key) {
                    Some(x) if &value > x => { min_values.insert(key, value); },
                    _ => (),
                };
            }
        }

        let red = min_values.get("red").unwrap();
        let green = min_values.get("green").unwrap();
        let blue = min_values.get("blue").unwrap();

        red * green * blue
    }

    Some(file.lines().map(parse_line).sum::<u32>())
}
