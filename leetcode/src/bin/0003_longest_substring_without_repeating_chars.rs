#![allow(dead_code)]

use std::collections::{HashMap};
use std::io::{self, Result};

trait Solution {
    fn length_of_longest_substring(s: String) -> i32;
}

struct BruteForce {}
struct On {}

impl Solution for BruteForce {

    /**
     * Time Complexity: O(n^2) (took 348 ms)
     * Space Complexity: O(n)
     */
    fn length_of_longest_substring(string: String) -> i32 {
        let mut result: i32 = 0;
        let mut map: HashMap<char, usize> = HashMap::new();

        for (i, ci) in string.chars().enumerate() {
            map.insert(ci, 1);
            let mut counter: i32 = 1;
            for cj in string.chars().skip(i + 1) {
                if map.contains_key(&cj) {
                    break;
                }
                map.insert(cj, 1);
                counter += 1;
            }
            map.clear();
            if counter > result {
                result = counter;
            }
        }
        result
    }
}

fn main() -> Result<()> {
    let mut string = String::new();
    io::stdin().read_line(&mut string)?;

    string.truncate(string.len() - 1);

    println!("{:?}", BruteForce::length_of_longest_substring(string.to_string()));
    Ok(())
}
