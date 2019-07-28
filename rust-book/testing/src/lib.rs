#![allow(dead_code)]

pub fn odd_ones_out(list: &[isize]) -> Vec<isize> {
	let mut odds: Vec<isize> = Vec::new();
	for element in list {
		if element & 1 == 1 {
			odds.push(*element);
		}
	}
	odds
}

// Private Adder Function, which can be bring into test mod scope...
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

/**
 * Assert Macros:
 * 	- `assert!` -> Expects a boolean result
 * 	- `assert_eq!` -> Passed Two args, should match in values
 * 	- `assert_ne!` -> Passed Two args, should not match in values
 *
 * As per conventions, Unit tests should be in the same file with a module named tests...
 */

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic] // Helps to keep a test check for code that can panic...
    fn with_custom_msg() {
    	// Should Fail with custom message...
    	assert_eq!(2*3, 4, "2 * 3 !== 4");
    }

    #[test]
    // #[should_panic] // can't be used here as Response::Err is not compatible with `should_panic`
    fn test_result() -> Result<(), String> {
    	if 2 + 2 == 4 {
    		Ok(())
    	} else {
    		Err(String::from("It Fails with some error message..."))
    	}
    }

    #[test]
    fn odd_ones_out() {
    	let list = vec![1,2,3,4,5];
    	let odd_list = super::odd_ones_out(&list);
    	assert_eq!(list, [1, 2, 3, 4, 5]);
    	assert_eq!(odd_list, [1, 3, 5]);
    }

    #[test]
    fn internal_adder() {
    	assert_eq!(super::internal_adder(2,3), 5);
    }
}
