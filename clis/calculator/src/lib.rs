//! Rust calculator CLI
//!
//! # Examples
//!
//! ```shell
//! $ cargo run
//! Rust calculator CLI
//!
//! Usage: calculator [COMMAND]
//!
//! Commands:
//!   add       Add two numbers
//!   subtract  Subtract two numbers
//!   multiply  Multiply two numbers
//!   divide    Divide two numbers
//!   mod       Perform a % b
//!   power     Raise a to power of b
//!   help      Print this message or the help of the given subcommand(s)
//!
//! Options:
//!   -h, --help     Print help
//!   -V, --version  Print version
//! ```

/// Add two numbers
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtract two numbers
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiply two numbers
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Divide two numbers
pub fn divide(a: i32, b: i32) -> i32 {
    a / b
}

/// Reminder of a divided by b
pub fn reminder(a: i32, b: i32) -> i32 {
    a % b
}

/// a raised to power of b
pub fn power(a: i32, b: i32) -> i32 {
    a.pow(b as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3)
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(1, 1), 0)
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 2), 4)
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(2, 2), 1)
    }

    #[test]
    fn test_reminder() {
        assert_eq!(reminder(2, 2), 0)
    }

    #[test]
    fn test_power() {
        assert_eq!(power(2, 2), 4)
    }
}
