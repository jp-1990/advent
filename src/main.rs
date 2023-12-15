/*
find the first and last int on each line
concat them
if the first digit is the last digit, repeat it

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

get the values for each line, sum them
*/

#![allow(dead_code)]

use std::fs;
mod day_eight;
mod day_11;
mod day_five;
mod day_four;
mod day_nine;
mod day_one;
mod day_seven;
mod day_six;
mod day_ten;
mod day_three;
mod day_two;
mod day_12;

fn read_file(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(filepath).unwrap();

    Ok(contents)
}

fn day_one(filepath: &str) {
    let file = read_file(filepath).unwrap();

    let one = day_one::part_1(&file);
    let two = day_one::part_2(&file);

    println!("1:1 {:?}", one.unwrap());
    println!("1:2 {:?}", two.unwrap());
}

fn day_two(filepath: &str) {
    let file = read_file(filepath).unwrap();

    let one = day_two::part_1(&file);
    let two = day_two::part_2(&file);

    println!("2:1 {:?}", one.unwrap());
    println!("2:2 {:?}", two.unwrap());
}

fn day_three(filepath: &str) {
    let file = read_file(filepath).unwrap();

    let one = day_three::part_1(&file);
    let two = day_three::part_2(&file);

    println!("3:1 {:?}", one.unwrap());
    println!("3:2 {:?}", two.unwrap());
}

fn day_four(filepath: &str) {
    let file = read_file(filepath).unwrap();

    let one = day_four::part_1(&file);
    let two = day_four::part_2(&file);

    println!("4:1 {:?}", one.unwrap());
    println!("4:2 {:?}", two.unwrap());
}

fn day_five(filepath: &str) {
    let file = read_file(filepath).unwrap();

    let one = day_five::part_1(&file);
    let two = day_five::part_2(&file);

    println!("5:1 {:?}", one.unwrap());
    println!("5:2 {:?}", two.unwrap());
}

fn day_six(filepath: &str) {
    let file = read_file(filepath).unwrap();

    let one = day_six::part_1(&file);
    let two = day_six::part_2(&file);

    println!("6:1 {:?}", one.unwrap());
    println!("6:2 {:?}", two.unwrap());
}

fn day_seven(filepath: &str) {
    let file = read_file(filepath).unwrap();

    let one = day_seven::part_1(&file);
    let two = day_seven::part_2(&file);

    println!("7:1 {:?}", one.unwrap());
    println!("7:2 {:?}", two.unwrap());
}

fn day_eight(filepath: &str) {
    let file = read_file(filepath).unwrap();

    let one = day_eight::part_1(&file);
    let two = day_eight::part_2(&file);

    println!("8:1 {:?}", one.unwrap());
    println!("8:2 {:?}", two.unwrap());
}

fn day_nine(filepath: &str) {
    let file = read_file(filepath).unwrap();

    let one = day_nine::part_1(&file);
    let two = day_nine::part_2(&file);

    println!("9:1 {:?}", one.unwrap());
    println!("9:2 {:?}", two.unwrap());
}

fn day_ten(filepath: &str) {
    let file = read_file(filepath).unwrap();

    let one = day_ten::part_1(&file);
    let two = day_ten::part_2(&file);

    println!("10:1 {:?}", one.unwrap());
    println!("10:2 {:?}", two.unwrap());
}

fn day_11(filepath: &str) {
    let file = read_file(filepath).unwrap();

    let one = day_11::part_1(&file);
    let two = day_11::part_2(&file);

    println!("11:1 {:?}", one.unwrap());
    println!("11:2 {:?}", two.unwrap());
}

fn day_12(filepath: &str) {
    let file = read_file(filepath).unwrap();

    let one = day_12::part_1(&file);
    let two = day_12::part_2(&file);

    println!("12:1 {:?}", one.unwrap());
    println!("12:2 {:?}", two.unwrap());
}

fn main() {
    // day_one("01_input.txt");
    // day_two("02_input.txt");
    // day_three("03_input.txt");
    // day_four("04_input.txt");
    // day_five("05_input.txt");
    // day_six("06_input.txt");
    // day_seven("07_input.txt");
    // day_eight("08_input.txt");
    // day_nine("09_input.txt");
    // day_ten("10_input.txt");
    // day_11("11_input.txt");
    day_12("12_input.txt");
}
