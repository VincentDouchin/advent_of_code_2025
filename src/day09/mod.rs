use std::fs;

use itertools::Itertools;

fn input() -> Vec<(isize, isize)> {
    fs::read_to_string("./src/day09/input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let nbs = l
                .split(",")
                .map(|str| str.parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            (nbs[0], nbs[1])
        })
        .collect::<Vec<_>>()
}
pub fn part1() -> String {
    let coords = input();
    let mut max = 0;

    for &(x1, y1) in &coords {
        for &(x2, y2) in &coords {
            max = ((x1 - x2 + 1).abs() * (y1 - y2 + 1).abs()).max(max)
        }
    }
    max.to_string()
}

fn is_point_in_square(point: &(isize, isize), square: &(isize, isize, isize, isize)) -> bool {
    let (x, y) = point;
    let (top, bottom, right, left) = square;
    left < x && x < right && bottom < y && y < top
}

pub fn part2() -> String {
    let coords: Vec<(isize, isize)> = input();

    let edges = coords
        .clone()
        .into_iter()
        .circular_tuple_windows::<(_, _)>()
        .collect::<Vec<_>>();

    let mut max = 0;

    let v_edges = edges
        .iter()
        .filter(|((x1, _y1), (x2, _y2))| x1 == x2)
        .sorted_by_key(|((_, y), (_, _))| y)
        .collect::<Vec<_>>();
    let h_edges = edges
        .iter()
        .filter(|((_x1, y1), (_x2, y2))| y1 == y2)
        .sorted_by_key(|((_, y), (_, _))| y)
        .collect::<Vec<_>>();

    for &(x1, y1) in &coords {
        for &(x2, y2) in &coords {
            let left = x1.min(x2);
            let right = x1.max(x2);
            let top = y1.max(y2);
            let bottom = y1.min(y2);
            let surface = (right - left + 1) * (top - bottom + 1);
            let square = (top, bottom, right, left);
            let int_v = h_edges.iter().any(|((x1, y1), (x2, y2))| {
                let l = x1.min(x2);
                let r = x1.max(x2);

                ((l < &left && &left < r) || (l < &right && &right < r))
                    && (&bottom < y1 && y2 < &top)
            });
            let int_h = v_edges.iter().any(|((x1, y1), (x2, y2))| {
                let b = y1.min(y2);
                let t = y1.max(y2);
                ((b < &bottom && &bottom < t) || (b < &top && &top < t))
                    && (&left < x1 && x2 < &right)
            });
            let tile_inside = coords.iter().any(|p| is_point_in_square(p, &square));

            if max < surface && !int_v && !int_h && !tile_inside {
                max = surface;
            }
        }
    }
    max.to_string()
}
