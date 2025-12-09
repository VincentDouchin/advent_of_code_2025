use std::{collections::HashMap, fs};

fn input() -> Vec<Vec<char>> {
    fs::read_to_string("./src/day07/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

pub fn part1() -> String {
    let input = input();
    let mut split = 0;
    let mut rays: Vec<usize> = Vec::new();
    let start = input
        .first()
        .unwrap()
        .iter()
        .enumerate()
        .find_map(|(i, ch)| if ch == &'S' { Some(i) } else { None })
        .unwrap();
    rays.push(start);
    // input.pop();
    // input.pop();
    for line in &input[1..] {
        let mut r: Vec<usize> = Vec::new();
        for (i, ch) in line.iter().enumerate() {
            if rays.contains(&i) {
                if ch == &'^' {
                    r.push(i - 1);
                    r.push(i + 1);
                    split += 1;
                } else {
                    r.push(i);
                }
            }
        }
        println!(
            "{:#?}",
            line.iter()
                .enumerate()
                .map(|(i, ch)| { if r.contains(&i) { '|' } else { *ch } })
                .collect::<String>()
        );
        rays = r;
    }

    split.to_string()
}

pub fn part2() -> String {
    let input = input();
    let start = input
        .first()
        .unwrap()
        .iter()
        .enumerate()
        .find_map(|(i, ch)| if ch == &'S' { Some(i) } else { None })
        .unwrap();
    let mut r: HashMap<usize, usize> = HashMap::new();
    r.insert(start, 1);
    for line in &input[1..] {
        let mut r1: HashMap<usize, usize> = HashMap::new();

        for (i, q) in &r {
            if line[*i] == '^' {
                *r1.entry(*i - 1).or_insert(0) += *q;
                *r1.entry(*i + 1).or_insert(0) += *q;
            } else {
                *r1.entry(*i).or_insert(0) += *q;
            }
        }
        r = r1;
    }
    r.values().sum::<usize>().to_string()
}
