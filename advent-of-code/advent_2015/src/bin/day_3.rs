/// Day 3 - https://adventofcode.com/2015/day/3

use std::collections::HashSet;
use advent_2015::Input;

fn update(ch: &char, pos: &mut (isize, isize)) {
    match ch {
        '^' => {
            // Move Up
            pos.1 += 1;
        },
        '>' => {
            pos.0 += 1;
        },
        'v' => {
            pos.1 -= 1;
        },
        '<' => {
            pos.0 -= 1;
        }
        _ => {
            // Do nothing
        }
    }
}

/**
 * Will be denoting house number, w.r.t their x/y index.
 * Santa is initially placed at (0, 0)
 */
fn main() {
    let input = Input::new();
    let mut santa_pos = (0, 0);
    let mut robo_pos = (0, 0);
    let mut houses_done = HashSet::new();
    houses_done.insert((0, 0));

    for (index, ch) in input.data.chars().enumerate() {
        if index % 2 == 0 {
            update(&ch, &mut santa_pos);
            houses_done.insert(santa_pos);
        } else {
            update(&ch, &mut robo_pos);
            houses_done.insert(robo_pos);
        }
    }

    println!("Count: {}", houses_done.len());
}
