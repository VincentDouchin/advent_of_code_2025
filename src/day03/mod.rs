use std::fs;

fn input1() -> Vec<Vec<String>> {
    fs::read_to_string("./src/day03/input.txt")
        .unwrap()
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| {
            line.split("")
                .map(|ch| ch.to_string())
                .filter(|ch| ch.len() > 0)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn part1() {
    let res = input1().iter_mut().fold(0, |acc, line| {
        let mut max = 0;
        for (i, ch1) in line.iter().enumerate() {
            let rest = line.clone().split_off(i + 1);
            // println!("{:#?}", rest);
            if rest.len() > 0 {
                for ch2 in rest {
                    let nb = (ch1.to_owned() + ch2.as_str()).parse::<isize>().unwrap();
                    if nb > max {
                        max = nb
                    }
                }
            }
        }
        println!("{}", max);
        acc + max
    });
    println!("{}", res)
}

pub fn part2() {
    let res = input1().iter().fold(0, |acc, line| {
        let line_with_nb = line
            .iter()
            .enumerate()
            .map(|(i, ch)| (i, ch.parse::<isize>().unwrap()))
            .collect::<Vec<_>>();

        let mut max_str: String = String::new();
        let mut index = 0;

        while max_str.len() != 12 {
            let need = 12 - max_str.len();

            let rest_end = line.len() - need;

            let rest = &line_with_nb[index..=rest_end];

            let mut best_digit = -1;
            let mut best_index = index;

            for (global_i, nb) in rest {
                if *nb > best_digit {
                    best_digit = *nb;
                    best_index = *global_i;
                    if best_digit == 9 {
                        break;
                    }
                }
            }

            max_str.push(char::from_digit(best_digit as u32, 10).unwrap());

            index = best_index + 1;

            println!("{}", max_str);
        }

        acc + max_str.parse::<isize>().unwrap()
    });
    println!("{}", res);
}
