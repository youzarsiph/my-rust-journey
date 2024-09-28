//! Actix calculator service

/// Add two numbers
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

/// Subtract two numbers
pub fn sub(x: i32, y: i32) -> i32 {
    x - y
}

/// Multiply two numbers
pub fn mul(x: i32, y: i32) -> i32 {
    x * y
}

/// Divide two numbers
pub fn div(x: i32, y: i32) -> i32 {
    x / y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 1), 2)
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(1, 1), 0)
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul(1, 1), 1)
    }

    #[test]
    fn test_div() {
        assert_eq!(div(1, 1), 1)
    }
}
