use std::collections::HashMap;

fn parse_seed_line(line: &str) -> Vec<u64> {
    line.split(": ").collect::<Vec<&str>>()[1]
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

fn parse_range_line(line: &str) -> (u64, u64, u64) {
    let split_line: Vec<u64> = line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
    (split_line[0], split_line[1], split_line[2])
}

fn parse_file(file: &str) -> (Vec<u64>, HashMap<&str, Vec<(u64, u64, u64)>>) {
    let mut hash = HashMap::new();
    let mut hash_key_idx = 0;
    let hash_keys = [
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    let mut lines = file.lines().into_iter();
    let seeds: Vec<u64> = parse_seed_line(&lines.next().unwrap());

    lines.next();
    for line in lines {
        match line {
            "" => hash_key_idx = hash_key_idx + 1,
            &_ if Option::is_some(&line.chars().nth(0).unwrap().to_digit(10)) => {
                let tuple = parse_range_line(line);
                hash.entry(hash_keys[hash_key_idx])
                    .and_modify(|e: &mut Vec<(u64, u64, u64)>| e.push(tuple))
                    .or_insert(vec![tuple]);
            }
            &_ => (),
        };
    }

    (seeds, hash)
}

fn parse_map_1(num: u64, map: &Vec<(u64, u64, u64)>) -> u64 {
    let mut mapped_n: u64 = num;

    for entry in map {
        let range = entry.2;

        mapped_n = match num {
            n if u64::from(n) >= entry.1 && u64::from(n) < entry.1 + range => {
                let diff = u64::from(num) - entry.1;
                (entry.0 + diff).try_into().unwrap()
            }
            _ => num,
        };

        if mapped_n != num {
            break;
        }
    }

    mapped_n
}

fn parse_seed_range(seed_range: Vec<u64>, entry: (u64, u64, u64)) -> (Vec<u64>, Vec<u64>) {
    let upper = u64::from(seed_range[0] + seed_range[1]);
    let lower = u64::from(seed_range[0]);
    let range = u64::from(seed_range[1]);

    let e_target = entry.0;
    let e_source = entry.1;
    let e_range = entry.2;
    let e_source_upper = e_source + e_range;

    let mut mapped_ranges = vec![];
    let mut ranges_to_map = vec![];

    if lower >= e_source && upper < e_source_upper {
        println!("full range inside source + range");
        let diff = lower - e_source;
        let mut new_range = vec![(e_target + diff), range];
        mapped_ranges.append(&mut new_range);
    } else if lower < e_source && upper >= e_source && upper < e_source_upper {
        println!("lower overlap, uppper within");
        let lower_diff = e_source - lower;
        let mut new_lower_range = vec![lower, lower_diff + 1];
        println!("new lower:{:?}", new_lower_range);
        ranges_to_map.append(&mut new_lower_range);

        let upper_lower = e_source;
        let upper_range = upper - upper_lower;
        let mut new_upper_range = vec![e_target, upper_range];
        println!("new upper:{:?}", new_upper_range);
        mapped_ranges.append(&mut new_upper_range);
    } else if lower >= e_source && lower < e_source_upper && upper >= e_source_upper {
        println!("lower within, uppper overlap");
        let upper_diff = upper - e_source_upper;
        let mut new_upper_range = vec![upper, upper_diff + 1];
        println!("new upper:{:?}", new_upper_range);
        ranges_to_map.append(&mut new_upper_range);

        let lower_diff = lower - e_source;
        let lower_range = range - upper_diff;
        let mut new_lower_range = vec![(e_target + lower_diff), lower_range];
        println!("new lower:{:?}", new_lower_range);
        mapped_ranges.append(&mut new_lower_range);
    } else if lower < e_source && upper >= e_source_upper {
        println!("lower overlap, uppper overlap");
        let lower_diff = e_source - lower;
        let mut new_lower_range = vec![lower, lower_diff];
        println!("new lower:{:?}", new_lower_range);
        ranges_to_map.append(&mut new_lower_range);

        let upper_diff = upper - e_source_upper;
        let mut new_upper_range = vec![e_source_upper, upper_diff +1];
        println!("new upper:{:?}", new_upper_range);
        ranges_to_map.append(&mut new_upper_range);

        mapped_ranges.append(&mut vec![
            e_target,
            e_range,
        ]);
    } else if upper < e_source || lower >= e_source_upper {
        // ranges.append(&mut seed_range.to_vec());
        ()
    }

    println!("mapped:{:?}", mapped_ranges);
    println!("rangesTo:{:?}", ranges_to_map);
    (mapped_ranges, ranges_to_map)
}

fn parse_map_2(seeds: Vec<u64>, map: &Vec<(u64, u64, u64)>) -> Vec<u64> {
    let mut ranges = vec![];

    for seed_range in seeds.windows(2).step_by(2) {
        let mut ranges_to_map = vec![];
        let mut found_range = false;

        let mut entries = map.into_iter().peekable();
        while let Some(entry) = entries.next() {
            let mut more_ranges_to_map = vec![];
            let (mut new_mapped_ranges, mut new_ranges_to_map) =
                parse_seed_range(seed_range.to_vec(), *entry);

            if new_mapped_ranges.len() > 0 {
                found_range = true
            }

            ranges.append(&mut new_mapped_ranges);
            more_ranges_to_map.append(&mut new_ranges_to_map);

            for range in ranges_to_map.windows(2).step_by(2) {
                let (mut nmr, mut rtm) = parse_seed_range(range.to_vec(), *entry);

                if nmr.len() > 0 {
                    found_range = true
                }

                ranges.append(&mut nmr);
                more_ranges_to_map.append(&mut rtm);
            }

            ranges_to_map.append(&mut more_ranges_to_map);

            if entries.peek().is_none() && !found_range && ranges_to_map.len() == 0 {
                println!("no match - return original value");
                ranges.append(&mut seed_range.to_vec());
            }
        }

        ranges.append(&mut ranges_to_map);
    }

    ranges
}

pub fn part_1(file: &str) -> Option<u64> {
    let (seeds, hash) = parse_file(file);

    let location = seeds.into_iter().map(|s| {
        let soil = parse_map_1(s, hash.get("seed-to-soil").unwrap());
        let fertilizer = parse_map_1(soil, hash.get("soil-to-fertilizer").unwrap());
        let water = parse_map_1(fertilizer, hash.get("fertilizer-to-water").unwrap());
        let light = parse_map_1(water, hash.get("water-to-light").unwrap());
        let temperature = parse_map_1(light, hash.get("light-to-temperature").unwrap());
        let humidity = parse_map_1(temperature, hash.get("temperature-to-humidity").unwrap());
        let location = parse_map_1(humidity, hash.get("humidity-to-location").unwrap());

        location
    });

    Some(location.min().unwrap().into())
}

pub fn part_2(file: &str) -> Option<u64> {
    let (seeds, hash) = parse_file(file);

    let location: Vec<_> = seeds
        .windows(2)
        .step_by(2)
        .map(|s| {
            let soil = parse_map_2(s.to_vec(), hash.get("seed-to-soil").unwrap());
            println!("soil:{:?}\n", soil);
            let fertilizer = parse_map_2(soil, hash.get("soil-to-fertilizer").unwrap());
            println!("fertilizer:{:?}\n", fertilizer);
            let water = parse_map_2(fertilizer, hash.get("fertilizer-to-water").unwrap());
            println!("water:{:?}\n", water);
            let light = parse_map_2(water, hash.get("water-to-light").unwrap());
            println!("light:{:?}\n", light);
            let temperature = parse_map_2(light, hash.get("light-to-temperature").unwrap());
            println!("temperature:{:?}\n", temperature);
            let humidity = parse_map_2(temperature, hash.get("temperature-to-humidity").unwrap());
            println!("humidity:{:?}\n", humidity);
            let location = parse_map_2(humidity, hash.get("humidity-to-location").unwrap());
            println!("location:{:?}\n", location);

            location
        })
        .collect::<Vec<Vec<_>>>()
        .into_iter()
        .map(|loc| loc.into_iter().step_by(2).min().unwrap())
        .collect();

    println!("location:{:?}", location);

    // answer: 52510809

    // Some(location.min().unwrap())
    Some(0)
}
