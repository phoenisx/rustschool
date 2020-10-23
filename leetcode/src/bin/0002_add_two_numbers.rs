use std::io::{self, Result};

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

trait Solution {
    fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

struct BruteForce {}
struct On {}

fn create_result(val: i32) -> Option<Box<ListNode>> {
    Some(Box::new(ListNode {
        val: val % 10,
        next: if val / 10 > 0 {
            Some(Box::new(ListNode {
                val: val / 10,
                next: None,
            }))
        } else {
            None
        }
    }))
}

impl Solution for BruteForce {

    /**
     * Time Complexity: O(n)
     * Space Complexity: O(1)
     */
    fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result: Option<Box<ListNode>> = None;
        let mut result_pointer = &mut result;
        let mut l1_pointer = &l1;
        let mut l2_pointer = &l2;
        let mut finished1 = false;
        let mut finished2 = false;
        loop {
            let i1: i32;
            let i2: i32;
            if let Some(node) = l1_pointer {
                i1 = node.val;
                l1_pointer = &node.next;
            } else {
                i1 = 0;
                finished1 = true;
            }
            if let Some(node) = l2_pointer {
                i2 = node.val;
                l2_pointer = &node.next;
            } else {
                i2 = 0;
                finished2 = true;
            }

            if finished1 && finished2 {
                break;
            } else {
                match result_pointer {
                    Some(node) => {
                        let sum = i1 + i2 + node.val;
                        *result_pointer = create_result(sum);

                    },
                    None => {
                        let sum = i1 + i2;
                        *result_pointer = create_result(sum);
                    }
                }
                if let Some(node) = result_pointer {
                    result_pointer = &mut node.next;
                }
            }
        }
        result
    }
}

fn main() -> Result<()> {
    let mut nums = String::new();
    io::stdin().read_line(&mut nums)?;
    let nums: Vec<i32> = nums
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    let mut l1: Option<Box<ListNode>> = None;
    let mut l1_pointer = &mut l1;
    for number in nums.iter() {
        match l1_pointer {
            Some(node) => {
                node.next = Some(Box::new(ListNode{
                    val: *number,
                    next: None,
                }));
            },
            None => {
                *l1_pointer = Some(Box::new(ListNode{
                    val: *number,
                    next: None,
                }));
            }
        }
        if let Some(node) = l1_pointer {
            l1_pointer = &mut node.next;
        }
    }

    let mut nums = String::new();
    io::stdin().read_line(&mut nums)?;
    let nums: Vec<i32> = nums
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    let mut l2: Option<Box<ListNode>> = None;
    let mut l2_pointer = &mut l2;
    for number in nums.iter() {
        match l2_pointer {
            Some(node) => {
                node.next = Some(Box::new(ListNode{
                    val: *number,
                    next: None,
                }));
            },
            None => {
                *l2_pointer = Some(Box::new(ListNode{
                    val: *number,
                    next: None,
                }));
            }
        }
        if let Some(node) = l2_pointer {
            l2_pointer = &mut node.next;
        }
    }

    println!("{:?}", BruteForce::add_two_numbers(l1, l2));
    Ok(())
}
