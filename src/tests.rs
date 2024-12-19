use super::add;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add::add(2, 3), 5); // Test will pass
    }

    #[test]
    fn test_add_fail() {
        assert_ne!(add::add(2, 2), 5); // Test will pass
    }
}