use std::fs;

fn input() -> Vec<(usize, Vec<(usize, char)>)> {
    fs::read_to_string("./src/day04/input.txt")
        .unwrap()
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(|l| l.chars().enumerate().collect())
        .enumerate()
        .collect()
}
pub fn part1() {
    let grid = input();
    let mut res = 0;
    for (y, line) in &grid {
        let mut line_str = String::new();
        for (x, char) in line {
            let mut c = char.clone().to_string();
            if char == &'@' {
                let valid = vec![
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ]
                .into_iter()
                .fold(0, |acc, (y1, x1)| {
                    let mut is_box = 0;
                    let x2 = x.clone() as isize + x1;
                    let y2 = y.clone() as isize + y1;
                    if x2 >= 0 && y2 >= 0 {
                        if let Some((_, l2)) = grid.get(y2 as usize) {
                            if let Some((_, char2)) = l2.get(x2 as usize) {
                                if char2 == &'@' {
                                    is_box = 1;
                                }
                            }
                        }
                    }
                    acc + is_box
                });

                if valid < 4 {
                    res += 1;
                    c = "x".to_string()
                }
                // c = valid.to_string()
            }
            line_str.push_str(c.as_str());
        }
        println!("{}", line_str)
    }
    println!("{}", res)
}
pub fn part2() {
    let mut grid = input();
    let mut res = 0;
    loop {
		let mut finished = true;
        let mut new_grid: Vec<(usize, Vec<(usize, char)>)> = Vec::new();
        for (y, line) in &grid {
            let mut new_line: Vec<(usize, char)> = Vec::new();
            let mut line_str = String::new();
            for (x, char) in line {
                let mut c = char.clone();
                if char == &'@' {
                    let valid = vec![
                        (-1, -1),
                        (-1, 0),
                        (-1, 1),
                        (0, -1),
                        (0, 1),
                        (1, -1),
                        (1, 0),
                        (1, 1),
                    ]
                    .into_iter()
                    .fold(0, |acc, (y1, x1)| {
                        let mut is_box = 0;
                        let x2 = x.clone() as isize + x1;
                        let y2 = y.clone() as isize + y1;
                        if x2 >= 0 && y2 >= 0 {
                            if let Some((_, l2)) = grid.get(y2 as usize) {
                                if let Some((_, char2)) = l2.get(x2 as usize) {
                                    if char2 == &'@' {
                                        is_box = 1;
                                    }
                                }
                            }
                        }
                        acc + is_box
                    });

                    if valid < 4 {
                        res += 1;
                        c = 'x';
                        finished = false;
                    }
                    // c = valid.to_string()
                }
                line_str.push_str(c.to_string().as_str());
                new_line.push((x.clone(), c));
            }
            new_grid.push((y.clone(), new_line));
            println!("{}", line_str)
        }
        grid = new_grid;
        if finished {
            println!("{}", res);
            return;
        }
    }
}
