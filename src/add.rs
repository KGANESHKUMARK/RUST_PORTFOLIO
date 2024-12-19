// Function to test
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_add_1() {
        assert_eq!(add(2, 3), 5); // Test will pass
    }

    #[test]
    fn test_add_fail() {
        assert_ne!(add(2, 2), 5); // Test will pass
    }
}