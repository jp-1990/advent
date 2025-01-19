use std::collections::HashMap;

fn parse_file(file: &str) -> Vec<(&str, u32)> {
    file.lines()
        .map(|line| {
            let split_line = line.split_whitespace().collect::<Vec<&str>>();
            (split_line[0], split_line[1].parse::<u32>().unwrap())
        })
        .collect()
}

#[derive(Debug,Clone, Eq, Ord, PartialEq, PartialOrd)]
enum Rules {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
    None = 0,
}

#[derive(Debug,Clone)]
struct Hand {
    cards: Vec<char>,
    bid: u32,
    rule: Rules,
    values: Vec<u32>,
}

impl Hand {
    pub fn new(hand: &str, bid: u32) -> Self {
        let cards = hand.chars().collect::<Vec<char>>();
        let mut hash = HashMap::new();

        for ch in &cards {
            let count = hash.entry(ch).or_insert(0);
            *count += 1;
        }

        let mut rule: Rules = Rules::None;

        let hash_iter = hash.into_iter();
        let hash_len = &hash_iter.len();
        for (_, value) in hash_iter {
            match value {
                5 if hash_len == &1 => {
                    rule = Rules::FiveOfAKind;
                    break;
                }
                4 if hash_len == &2 => {
                    rule = Rules::FourOfAKind;
                    break;
                }
                3 if hash_len == &2 => {
                    rule = Rules::FullHouse;
                    break;
                }
                3 if hash_len == &3 => {
                    rule = Rules::ThreeOfAKind;
                    break;
                }
                2 if hash_len == &2 => {
                    rule = Rules::FullHouse;
                    break;
                }
                2 if hash_len == &3 => {
                    rule = Rules::TwoPair;
                    break;
                }
                2 if hash_len == &4 => {
                    rule = Rules::OnePair;
                    break;
                }
                1 if hash_len == &5 => {
                    rule = Rules::HighCard;
                    break;
                }
                _ => (),
            }
        }

        let card_values: HashMap<char, u32> = HashMap::from([
            ('2', 2),
            ('3', 3),
            ('4', 4),
            ('5', 5),
            ('6', 6),
            ('7', 7),
            ('8', 8),
            ('9', 9),
            ('T', 10),
            ('J', 11),
            ('Q', 12),
            ('K', 13),
            ('A', 14),
        ]);

        let values = cards
            .clone()
            .into_iter()
            .map(|c| *card_values.get(&c).unwrap())
            .collect::<Vec<u32>>();

        Hand {
            cards,
            bid,
            rule,
            values,
        }
    }

    pub fn new_2(hand: &str, bid: u32) -> Self {
        let cards = hand.chars().collect::<Vec<char>>();
        let mut hash = HashMap::new();

        for ch in &cards {
            let count = hash.entry(ch).or_insert(0);
            *count += 1;
        }
        let n_jokers = *hash.get(&'J').unwrap_or(&0);

        let mut rule: Rules = Rules::None;

        let hash_iter = hash.into_iter();
        let hash_len = &hash_iter.len();

        for (_, value) in hash_iter {
            match value {
                5 if hash_len == &1 => {
                    rule = Rules::FiveOfAKind;
                    break;
                }
                4 if hash_len == &2 => {
                    match n_jokers {
                        4 => rule = Rules::FiveOfAKind,
                        1 => rule = Rules::FiveOfAKind,
                        _ => rule = Rules::FourOfAKind,
                    }
                    break;
                }
                3 if hash_len == &2 => {
                    match n_jokers {
                        3 => rule = Rules::FiveOfAKind,
                        2 => rule = Rules::FiveOfAKind,
                        _ => rule = Rules::FullHouse,
                    }
                    break;
                }
                3 if hash_len == &3 => {
                    match n_jokers {
                        3 => rule = Rules::FourOfAKind,
                        1 => rule = Rules::FourOfAKind,
                        _ => rule = Rules::ThreeOfAKind,
                    }
                    break;
                }
                2 if hash_len == &2 => {
                    match n_jokers {
                        3 => rule = Rules::FiveOfAKind,
                        2 => rule = Rules::FiveOfAKind,
                        _ => rule = Rules::FullHouse,
                    }
                    break;
                }
                2 if hash_len == &3 => {
                    match n_jokers {
                        2 => rule = Rules::FourOfAKind,
                        1 => rule = Rules::FullHouse,
                        _ => rule = Rules::TwoPair,
                    }
                    break;
                }
                2 if hash_len == &4 => {
                    match n_jokers {
                        2 => rule = Rules::ThreeOfAKind,
                        1 => rule = Rules::ThreeOfAKind,
                        _ => rule = Rules::OnePair,
                    }
                    break;
                }
                1 if hash_len == &5 => {
                    match n_jokers {
                        1 => rule = Rules::OnePair,
                        _ => rule = Rules::HighCard,
                    }
                    break;
                }
                _ => (),
            }
        }

        let card_values: HashMap<char, u32> = HashMap::from([
            ('2', 2),
            ('3', 3),
            ('4', 4),
            ('5', 5),
            ('6', 6),
            ('7', 7),
            ('8', 8),
            ('9', 9),
            ('T', 10),
            ('J', 1),
            ('Q', 11),
            ('K', 12),
            ('A', 13),
        ]);

        let values = cards
            .clone()
            .into_iter()
            .map(|c| *card_values.get(&c).unwrap())
            .collect::<Vec<u32>>();

        Hand {
            cards,
            bid,
            rule,
            values,
        }
    }


    fn cmp(self, hand: Hand) -> std::cmp::Ordering {
        let mut order = self.rule.cmp(&hand.rule);

        if order == std::cmp::Ordering::Equal {
            let mut val_order = std::cmp::Ordering::Equal;

            for (a, b) in self.values.into_iter().zip(hand.values.into_iter()) {
                val_order = b.cmp(&a);
                match val_order {
                    std::cmp::Ordering::Equal => continue,
                    _ => {
                        order = val_order;
                        break;
                    }
                }
            }

            match val_order {
                std::cmp::Ordering::Equal => order = std::cmp::Ordering::Less,
                _ => ()
            }
        }

        order
    }
}

pub fn part_1(file: &str) -> Option<u32> {
    let lines = parse_file(file);

    let mut hands = lines
        .into_iter()
        .map(|line| Hand::new(line.0, line.1))
        .collect::<Vec<Hand>>();

    hands.sort_by(|a, b| {
        let res = a.rule.cmp(&b.rule);
        match res {
            std::cmp::Ordering::Equal => b.clone().cmp(a.clone()),
            _ => res,
        }
    });

    let total_winnings: u32 = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| {
            let rank: u32 = <usize as TryInto<u32>>::try_into(i).unwrap() + 1;
            hand.bid * rank
        })
        .sum();

    Some(total_winnings)
}

pub fn part_2(file: &str) -> Option<u32> {
    let lines = parse_file(file);

    let mut hands = lines
        .into_iter()
        .map(|line| Hand::new_2(line.0, line.1))
        .collect::<Vec<Hand>>();

    hands.sort_by(|a, b| {
        let res = a.rule.cmp(&b.rule);
        match res {
            std::cmp::Ordering::Equal => b.clone().cmp(a.clone()),
            _ => res,
        }
    });

    for hand in &hands {
        println!("{:?}", hand)
    }

    let total_winnings: u32 = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| {
            let rank: u32 = <usize as TryInto<u32>>::try_into(i).unwrap() + 1;
            hand.bid * rank
        })
        .sum();

    Some(total_winnings)
}
