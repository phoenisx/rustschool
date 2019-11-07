// Question can be found here - https://adventofcode.com/2015/day/1#part2

use advent_2015::Input;

fn main() {
    let input = Input::new();
    let mut direction = 0;
    let mut found_index: usize = 0;

    let is_found = !input.data.chars().enumerate().all(|(index, ch)| {
        if ch == '(' { direction += 1 }
        else if ch == ')' { direction -= 1 }

        if direction == -1 {
            found_index = index + 1;
            return false
        }
        return true
    });

    if is_found {
        println!("{}", found_index);
    } else {
        println!("Not Found");
    }

}
