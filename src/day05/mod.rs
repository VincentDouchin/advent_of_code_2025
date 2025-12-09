use itertools::*;
use std::{collections::HashSet, fs};

fn input() -> (Vec<Vec<isize>>, Vec<isize>) {
    let parts = fs::read_to_string("./src/day05/input.txt")
        .unwrap()
        .split("\n\n")
        .filter(|l| l.len() > 0)
        .map(|line| line.to_string())
        .collect::<Vec<_>>();
    let ranges = parts[0]
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(|l| {
            l.split("-")
                .map(|c| c.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let ids = parts[1]
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(|nb| nb.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    (ranges, ids)
}

pub fn part1() -> String {
    let (ranges, ids) = input();
    let res = ids.iter().fold(0, |acc, id| {
        acc + if ranges.iter().any(|r| {
            let start = r[0];
            let end = r[1];
            id > &start && id <= &end
        }) {
            1
        } else {
            0
        }
    });
    res.to_string()
}

fn merge_ranges(mut ranges: Vec<Vec<isize>>) -> Vec<Vec<isize>> {
    ranges.sort_by_key(|r| r[0]);

    let mut merged: Vec<Vec<isize>> = Vec::new();

    for r in ranges {
        if let Some(last) = merged.last_mut() {
            if r[0] <= last[1] + 1 {
                last[1] = last[1].max(r[1]);
                continue;
            }
        }
        merged.push(r);
    }

    merged
}

fn vec_to_tuple(vec: &Vec<isize>) -> (isize, isize) {
    (vec[0], vec[1])
}

pub fn part2() -> String {
    let (ranges, _) = input();

    merge_ranges(ranges)
        .iter()
        .map(vec_to_tuple)
        .fold(0, |acc, (s, e)| acc + (e - s) + 1)
        .to_string()
}
