// Day 2 - 2015 - https://adventofcode.com/2015/day/2
// File to work with https://adventofcode.com/2015/day/2/input

use advent_2015::Input;

fn main() {
    let input = Input::new();
    let mut total_paper = 0;
    let mut total_ribbon = 0;

    for line in input.data.lines() {
        let mut data: Vec<usize> = line.split("x")
            .map(|value| value.parse().unwrap())
            .collect();
        data.sort();
        let min_face = data[0] * data[1];
        total_paper += 2 * min_face
            + 2 * data[1] * data[2]
            + 2 * data[2] * data[0]
            + min_face;

        // # Day 2.1
        total_ribbon += data[0] + data[0] + data[1] + data[1]
            + (data[0] * data[1] * data[2]);
    }

    println!("Paper: {}", total_paper);
    println!("Ribbon: {}", total_ribbon);
}