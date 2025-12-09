use core::f64;
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs, isize,
};

use itertools::Itertools;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}
impl Point {
    fn distance_to(&self, other: &Self) -> isize {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

fn input() -> Vec<Point> {
    fs::read_to_string("./src/day08/input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let nbs = l
                .split(",")
                .map(|nb| nb.parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            Point {
                x: nbs[0],
                y: nbs[1],
                z: nbs[2],
            }
        })
        .collect::<Vec<_>>()
}

pub fn part1() -> String {
    let boxes = input();
    let mut dists: Vec<(isize, &Point, &Point)> = Vec::new();
    for b1 in &boxes {
        for b2 in &boxes {
            if b1 != b2 {
                let d = b1.distance_to(b2);
                dists.push((d, b1, b2));
            }
        }
    }
    dists.sort_by_key(|c| c.0);
    let mut circuits: Vec<Vec<&Point>> = boxes.iter().map(|b| vec![b]).collect();

    for (_, p1, p2) in &dists[0..1000] {
        let c1 = circuits.iter().find(|c| c.contains(p1)).unwrap();
        let c2 = circuits.iter().find(|c| c.contains(p2)).unwrap();

        let mut new_circuits = circuits
            .clone()
            .into_iter()
            .filter(|c| !c.contains(p1) && !c.contains(p2))
            .collect::<Vec<_>>();

        // merge
        let mut c3 = c1.clone();
        c3.extend(c2);

        new_circuits.push(
            c3.into_iter()
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>(),
        );
        circuits = new_circuits;
    }
    circuits.sort_by_key(|c| c.len());
    circuits.reverse();

    circuits
        .iter()
        .take(3)
        .map(|c| c.len())
        .product::<usize>()
        .to_string()
}
pub fn part2() -> String {
    let boxes = input();
    let mut dists: Vec<(isize, &Point, &Point)> = Vec::new();
    for b1 in &boxes {
        for b2 in &boxes {
            if b1 != b2 {
                let d = b1.distance_to(b2);
                dists.push((d, b1, b2));
            }
        }
    }
    dists.sort_by_key(|c| c.0);
    let mut circuits: Vec<Vec<&Point>> = boxes.iter().map(|b| vec![b]).collect();

    let mut res = 0;
    let mut index = 0;
    while circuits.len() > 1 {
        let (_, p1, p2) = &dists[index];
        index += 1;
        let c1 = circuits.iter().find(|c| c.contains(p1)).unwrap();
        let c2 = circuits.iter().find(|c| c.contains(p2)).unwrap();

        let mut new_circuits = circuits
            .clone()
            .into_iter()
            .filter(|c| !c.contains(p1) && !c.contains(p2))
            .collect::<Vec<_>>();

        // merge
        let mut c3 = c1.clone();
        c3.extend(c2);

        new_circuits.push(
            c3.into_iter()
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>(),
        );
        circuits = new_circuits;
        res = p1.x * p2.x;
    }
    res.to_string()
}
