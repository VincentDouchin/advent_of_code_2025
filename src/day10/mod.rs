use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

fn input() -> Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)> {
    fs::read_to_string("./src/day10/input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let (part1, part2) = l.split_once(' ').expect(l);
            let indicators = part1
                .chars()
                .filter_map(|c| match c {
                    '.' => Some(false),
                    '#' => Some(true),
                    _ => None,
                })
                .collect::<Vec<_>>();
            let mut buttons_str = part2.split(" ").collect::<Vec<_>>();
            let joltage = buttons_str
                .pop()
                .unwrap()
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let buttons = buttons_str
                .iter()
                .map(|str| {
                    str.trim_matches(|c| c == '(' || c == ')')
                        .split(',')
                        .map(|x| x.parse::<usize>().expect(str))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            (indicators, buttons, joltage)
        })
        .collect::<Vec<_>>()
}

fn find_shortest(target: &Vec<bool>, buttons: &Vec<Vec<usize>>) -> Option<usize> {
    let mut queue = HashSet::new();
    let start = vec![false; target.len()];
    queue.insert(start);

    let mut i: usize = 0;

    loop {
        let mut new_queue = HashSet::new();
        for s in queue {
            for button in buttons {
                let current = s
                    .iter()
                    .enumerate()
                    .map(|(j, p)| if button.contains(&j) { !p } else { *p })
                    .collect::<Vec<_>>();
                // println!("{:#?}", current);
                if &current == target {
                    return Some(i + 1);
                }
                new_queue.insert(current);
            }
        }
        queue = new_queue;
        i += 1;
    }
}

use z3::{Optimize, ast::Int};

fn solve_joltage(buttons: &Vec<Vec<usize>>, target: &Vec<usize>) -> Option<usize> {
    let optimizer = Optimize::new();
    let buttons_var = buttons
        .iter()
        .enumerate()
        .map(|(i, _)| Int::new_const(format!("button_{}", i)))
        .collect::<Vec<_>>();

    for button_var in &buttons_var {
        optimizer.assert(&button_var.ge(&Int::from_i64(0)));
    }

    let num_counters = target.len();
    for counter_idx in 0..num_counters {
        let terms = buttons
            .iter()
            .enumerate()
            .filter_map(|(i, button)| {
                if button.contains(&counter_idx) {
                    Some(&buttons_var[i])
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let counter_sum = Int::add(&terms);
        let target_val = Int::from_i64(target[counter_idx] as i64);
        optimizer.assert(&counter_sum.eq(&target_val));
    }

    let total_presses = Int::add(&buttons_var);
    optimizer.minimize(&total_presses);

    optimizer.check(&[]);
    let model = optimizer.get_model().unwrap();
    let min_presses = model.eval(&total_presses, true).unwrap().as_i64().unwrap();

    Some(min_presses as usize)
}
pub fn part1() -> String {
    input()
        .iter()
        .map(|(indicators, buttons, _)| find_shortest(indicators, buttons).unwrap())
        .sum::<usize>()
        .to_string()
}
pub fn part2() -> String {
    input()
        .iter()
        .map(|(_indicators, buttons, joltage)| solve_joltage(buttons, joltage).unwrap())
        .sum::<usize>()
        .to_string()
}
