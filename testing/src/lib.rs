/**
 * Assert Macros:
 * 	- `assert!` -> Expects a boolean result
 * 	- `assert_eq!` -> Passed Two args, should match in values
 * 	- `assert_ne!` -> Passed Two args, should not match in values
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
    	if 2 + 3 == 4 {
    		Ok(())
    	} else {
    		Err(String::from("It Fails with some error message..."))
    	}
    }
}
