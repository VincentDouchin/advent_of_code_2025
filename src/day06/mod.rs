use std::fs;

use itertools::Itertools;

fn input() -> Vec<(String, Vec<isize>)> {
    let mut lines: Vec<Vec<String>> = fs::read_to_string("./src/day06/input.txt")
        .unwrap()
        .lines()
        .map(|line| line.split_whitespace().map(|c| c.to_string()).collect())
        .collect();
    let last = lines.pop().unwrap();
    last.iter()
        .enumerate()
        .map(|(index, op)| {
            let nbs = lines
                .iter()
                .map(|l| l[index].parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            (op.clone(), nbs.clone())
        })
        .collect::<Vec<_>>()
}
fn input2() -> Vec<(String, Vec<isize>)> {
    let lines = fs::read_to_string("./src/day06/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.to_string().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut spaces = lines[0]
        .iter()
        .enumerate()
        .map(|(index, _)| index)
        .filter(|index| lines.iter().all(|l| l[*index] == ' '))
        .collect::<Vec<_>>();
    spaces.insert(0, 0);
    spaces.push(lines[0].len());
    let groups = spaces
        .windows(2)
        .collect::<Vec<_>>()
        .into_iter()
        .map(|window| {
            let start = window[0];
            let end = window[1];
            lines
                .iter()
                .map(move |l| l[start..end].iter().clone().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .map(move |mut lines| {
            let op = lines
                .pop()
                .unwrap()
                .into_iter()
                .collect::<String>()
                .trim()
                .to_string();
            let nbs = lines.clone()[0]
                .iter()
                .enumerate()
                .map(|(index, ch)| {
                    let mut arr = vec![ch.to_owned()];
                    for l in &lines[1..] {
                        arr.push(l[index]);
                    }
                    arr.into_iter().collect::<String>().trim().to_string()
                })
                .filter(move |str| str.len() > 0)
                .map(|str| str.parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            (op, nbs)
        })
        .collect::<Vec<_>>();
    groups
}

fn calculate(input: Vec<(String, Vec<isize>)>) -> String {
    input
        .iter()
        .map(|(op, nbs)| {
            if op == "*" {
                nbs.iter().product::<isize>()
            } else {
                nbs.iter().sum()
            }
        })
        .sum::<isize>()
        .to_string()
}

pub fn part1() -> String {
    calculate(input())
}
pub fn part2() -> String {
    calculate(input2())
}
