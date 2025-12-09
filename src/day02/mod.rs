use std::{collections::HashSet, fs};

fn is_repeating(str: String) -> bool {
    if str.len() % 2 == 1 {
        return false;
    }
    let mid = str.len() / 2;
    let parts = str.split_at(mid);
    return parts.0 == parts.1;
}

pub fn _part1() {
    let input = fs::read_to_string("./src/day02/input.txt")
        .unwrap()
        .replace("\n", "");
    let ranges = input.split(",").map(|range| {
        range
            .split("-")
            .map(|text| {
                text.parse::<isize>()
                    .expect(format!("{text} is a number").as_str())
            })
            .collect::<Vec<_>>()
    });
    let mut res = 0;
    for range in ranges {
        let nb1 = range[0];
        let nb2 = range[1];

        for n in nb1..nb2 {
            // println!("{}", n.to_string());
            if is_repeating(n.to_string()) {
                res += n
            }
        }
    }
    println!("{}", res)
}
fn is_repeating2(str: String) -> bool {
    for len in 1..str.len() {
        let mut hash = HashSet::new();
        let parts: Vec<String> = str
            .chars()
            .collect::<Vec<_>>() // collect chars
            .chunks(len) // chunk them
            .map(|c| c.iter().collect::<String>())
            .collect();
        for part in parts {
            hash.insert(part);
        }
        if hash.len() == 1 {
            return true;
        }
    }
    return false;
}
pub fn part2() {
    let input = fs::read_to_string("./src/day02/input.txt")
        .unwrap()
        .replace("\n", "");
    let ranges = input.split(",").map(|range| {
        range
            .split("-")
            .map(|text| {
                text.parse::<isize>()
                    .expect(format!("{text} is a number").as_str())
            })
            .collect::<Vec<_>>()
    });
    let mut res = 0;
    for range in ranges {
        let nb1 = range[0];
        let nb2 = range[1];
        for n in nb1..(nb2 + 1) {
            if is_repeating2(n.to_string()) {
                res += n
            }
        }
    }

    println!("{}", res)
}
