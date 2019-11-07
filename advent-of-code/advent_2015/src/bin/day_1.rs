// Question can be found here - https://adventofcode.com/2015/day/1

use advent_2015::Input;

fn main() {
    let input = Input::new();
    let mut direction = 0;

    // Since this problem is very simple,
    // and can be done in O(n) complexity, will keep the code simple as well...

    for ch in input.data.chars() {
        if ch == '(' {
            direction += 1;
        } else if ch == ')' {
            direction -= 1;
        }
    }

    println!("{}", direction);
}
