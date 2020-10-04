use std::{collections::{ HashMap }, convert::TryInto};
use std::io::{self, Read, Result};

trait Solution {
    fn two_sum(nums: Vec<isize>, target: isize) -> Vec<isize>;
}

struct BruteForce {}
struct On {}

impl Solution for BruteForce {

    /**
     * Time Complexity: O(n^2)
     * Space Complexity: O(1)
     */
    fn two_sum(nums: Vec<isize>, target: isize) -> Vec<isize> {
        let mut i: usize = 0;
        while i < nums.len() {
            let mut j = i + 1;
            while j < nums.len() {
                if nums[j] == target - nums[i] {
                    return vec![i.try_into().unwrap(), j.try_into().unwrap()];
                }
                j += 1;
            }
            i += 1;
        }
        return vec![];
    }
}

impl Solution for On {

    /**
     * Time Complexity: O(n)
     * Space Complexity: O(n)
     */
    fn two_sum(nums: Vec<isize>, target: isize) -> Vec<isize> {
        let mut map: HashMap<isize, usize> = HashMap::new(); // Space Complexity O(n)
        let mut result: Vec<isize> = Vec::with_capacity(2); // Space Complexity O(1).


        for (index, &num) in nums.iter().enumerate() {
            let diff = target - num;
            if map.contains_key(&diff) {
                result.push(map[&diff] as isize);
                result.push(index as isize);
                break;
            }
            map.insert(num, index);
        }

        result
    }
}

fn main() -> Result<()> {
    let mut nums = String::new();
    let mut target = String::new();
    io::stdin().read_line(&mut nums)?;
    io::stdin().read_line(&mut target)?;
    let nums: Vec<isize> =
        nums
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
    let target = target.trim().parse().unwrap();
    println!("{:?}", BruteForce::two_sum(nums, target));
    Ok(())
}
