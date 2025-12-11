use std::{
    clone,
    collections::{HashMap, HashSet},
    fs,
};

fn input() -> HashMap<String, Vec<String>> {
    fs::read_to_string("./src/day11/input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let (label, devices_str) = l.split_once(":").unwrap();
            let devices = devices_str
                .trim()
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            (label.to_string(), devices)
        })
        .collect()
}

fn dfs_1(last: &String, map: &HashMap<String, Vec<String>>) -> isize {
    let mut res = 0;
    for dest in &map[last] {
        if dest == "out" {
            res += 1;
        } else {
            res += dfs_1(&dest, map)
        }
    }
    res
}
use memoize::memoize;

fn dfs_2(
    last: &String,
    map: &HashMap<String, Vec<String>>,
    dac: bool,
    fft: bool,
    cache: &mut HashMap<(String, bool, bool), isize>,
) -> isize {
    let key = (last.clone(), dac, fft);

    if let Some(v) = cache.get(&key) {
        return *v;
    }

    let mut res = 0;
    for dest in &map[last] {
        if dest == "out" {
            if dac && fft {
                res += 1;
            }
        } else {
            res += dfs_2(dest, map, dac || dest == "dac", fft || dest == "fft", cache);
        }
    }

    cache.insert(key, res);
    res
}

pub fn part1() -> String {
    dfs_1(&"you".to_string(), &input()).to_string()
}
pub fn part2() -> String {
    let mut cache = HashMap::new();
    dfs_2(&"svr".to_string(), &input(), false, false, &mut cache).to_string()
}
