use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Present {
    max_x: usize,
    max_y: usize,
    pub coords: Vec<(usize, usize)>,
}

impl Present {
    fn new(coords: Vec<(usize, usize)>) -> Self {
        let max_x = coords.iter().map(|(x, _)| x).max().unwrap().clone();
        let max_y = coords.iter().map(|(_, y)| y).max().unwrap().clone();
        Present {
            max_x,
            max_y,
            coords,
        }
    }
    fn rotate_90(&self) -> Self {
        let new_coords = self
            .coords
            .clone()
            .into_iter()
            .map(|(x, y)| (y, x))
            .collect::<Vec<_>>();
        Present::new(new_coords)
    }
    fn flip_h(&self) -> Self {
        let new_coords = self
            .coords
            .clone()
            .into_iter()
            .map(|(x, y)| (self.max_x - x, y))
            .collect();
        Present::new(new_coords)
    }
    fn flip_v(&self) -> Self {
        let new_coords = self
            .coords
            .clone()
            .into_iter()
            .map(|(x, y)| (x, self.max_y - y))
            .collect();
        Present::new(new_coords)
    }

    fn print(&self) {
        for y in 0..self.max_y + 1 {
            let mut line: String = "".to_string();
            for x in 0..self.max_x + 1 {
                if self.coords.contains(&(x, y)) {
                    line.push_str("#");
                } else {
                    line.push_str(".");
                }
            }
            println!("{line}")
        }
        println!("\n")
    }
    fn get_all_orientations(&self) -> HashSet<Present> {
        let mut orientations = HashSet::new();
        let mut current = self.clone();
        for _ in 0..4 {
            orientations.insert(current.clone());
            orientations.insert(current.flip_h());
            orientations.insert(current.flip_v());
            orientations.insert(current.flip_h().flip_v());

            current = current.rotate_90();
        }

        orientations.into_iter().collect()
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Region {
    x: usize,
    y: usize,
    cells: Vec<Vec<bool>>,
}
impl Region {
    fn new(x: usize, y: usize) -> Self {
        Region {
            x,
            y,
            cells: vec![vec![false; x]; y],
        }
    }
    fn has_enough_space(
        &self,
        present_indexes: &Vec<usize>,
        presents: &Vec<Present>,
        from_index: usize,
    ) -> bool {
        let free_cells = self
            .cells
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| !cell)
            .count();
        let present_sizes = presents
            .iter()
            .map(|p| p.coords.iter().count())
            .collect::<Vec<_>>();
        let needed_cells: usize = present_indexes[from_index..]
            .iter()
            .map(|p| present_sizes[*p])
            .sum();

        free_cells >= needed_cells
    }

    fn can_place(&self, present: &Present, offset: (usize, usize)) -> bool {
        for (x, y) in &present.coords {
            let x = x + offset.0;
            let y = y + offset.1;

            if x >= self.x || y >= self.y {
                return false;
            }

            if self.cells[y][x] {
                return false;
            }
        }
        true
    }

    fn place(&mut self, present: &Present, offset: (usize, usize)) {
        for (x, y) in &present.coords {
            let x = x + offset.0;
            let y = y + offset.1;
            self.cells[y][x] = true;
        }
    }

    fn remove(&mut self, present: &Present, offset: (usize, usize)) {
        for (x, y) in &present.coords {
            let x = x + offset.0;
            let y = y + offset.1;
            self.cells[y][x] = false;
        }
    }
}

pub fn input() -> (Vec<(Region, Vec<usize>)>, Vec<Present>) {
    let str = fs::read_to_string("./src/day12/input.txt").unwrap();
    let mut input = str.split("\n\n").collect::<Vec<_>>();
    let regions_str = input.pop();
    let presents = input
        .iter()
        .map(|str| {
            let lines = str.split("\n").collect::<Vec<_>>();
            let coords = lines[1..]
                .iter()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.chars()
                        .enumerate()
                        .filter_map(|(x, c)| if c == '#' { Some((x, y)) } else { None })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            Present::new(coords)
        })
        .collect::<Vec<_>>();
    let regions = regions_str
        .unwrap()
        .split("\n")
        .map(|str| {
            let (dim_str, indexes_str) = str.split_once(": ").unwrap();
            let dims = dim_str.split("x").collect::<Vec<_>>();
            let x = dims[0].parse::<usize>().unwrap();
            let y = dims[1].parse::<usize>().unwrap();
            let indexes = indexes_str
                .split(" ")
                .enumerate()
                .filter_map(|(i, nb_str)| {
                    let nb = nb_str
                        .trim()
                        .parse::<usize>()
                        .expect(format!("\n here:'{}' \n", nb_str).to_string().as_str());
                    if nb > 0 { Some(vec![i; nb]) } else { None }
                })
                .flatten()
                .collect::<Vec<_>>();
            (Region::new(x, y), indexes)
        })
        .collect::<Vec<_>>();

    (regions, presents)
}

fn dfs(
    region: &mut Region,
    presents_indexes: &Vec<usize>,
    presents: &Vec<Present>,
    present_index: usize,
) -> bool {
    if present_index == presents_indexes.len() {
        return true;
    }
    if !region.has_enough_space(presents_indexes, presents, present_index) {
        return false;
    }

    let present = &presents[presents_indexes[present_index]];

    for orientation in present.get_all_orientations() {
        for y in 0..region.y {
            for x in 0..region.x {
                if region.can_place(&orientation, (x, y)) {
                    region.place(&orientation, (x, y));

                    if dfs(region, presents_indexes, presents, present_index + 1) {
                        return true;
                    }

                    region.remove(&orientation, (x, y));
                }
            }
        }
    }

    false
}

pub fn part1() -> String {
    let (regions, presents) = input();
    let mut res = 0;
    for (mut region, present_indexes) in regions {
        if dfs(&mut region, &present_indexes, &presents, 0) {
            res += 1;
        }
    }
    res.to_string()
}
pub fn part2() -> String {
    "".to_string()
}
