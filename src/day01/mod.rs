use regex::Regex;
use std::fs;
pub fn part1() {
    let input = fs::read_to_string("./src/day01/input.txt").unwrap();
    let lines = input.split("\n").filter(|line| line.len() > 0);
    let re = Regex::new(r"^([LR])(\d+)$").unwrap();
    let mut pointer = 50;
    let mut nb_of_zero = 0;
    for line in lines {
        let captures = re.captures(line).unwrap();
        let letter = captures.get(1).unwrap().as_str();
        let number_str = captures.get(2).unwrap().as_str();
        let number: i64 = number_str.parse().unwrap();
        if letter == "R" {
            pointer += number;
        }
        if letter == "L" {
            pointer -= number;
        }
        while pointer >= 100 {
            pointer -= 100;
        }
        while pointer < 0 {
            pointer += 100;
        }
        if pointer == 0 {
            nb_of_zero += 1;
        }
        println!("{} {} {}", letter, number, pointer);
    }
    dbg!(nb_of_zero);
}
pub fn part2() {
    let input = fs::read_to_string("./src/day01/input.txt").unwrap();
    let lines = input.split("\n").filter(|line| line.len() > 0);
    let re = Regex::new(r"^([LR])(\d+)$").unwrap();
    let mut pointer = 50;
    let mut nb_of_zero = 0;
    for line in lines {
        let captures = re.captures(line).unwrap();
        let letter = captures.get(1).unwrap().as_str();
        let number_str = captures.get(2).unwrap().as_str();
        let mut number: i64 = number_str.parse().unwrap();

        for _n in 0..number {
            number -= 1;
            if letter == "R" {
                pointer += 1;
            }
            if letter == "L" {
                pointer -= 1;
            }

            while pointer >= 100 {
                pointer -= 100;
            }
            while pointer < 0 {
                pointer += 100;
            }
            if pointer == 0 {
                nb_of_zero += 1;
            }
        }
    }
    dbg!(nb_of_zero);
}
