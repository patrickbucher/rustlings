// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

struct Factorial {
    i: u64,
    cache: Vec<u64>,
}

impl Factorial {
    fn new() -> Factorial {
        Factorial {
            i: 0,
            cache: Vec::new(),
        }
    }
}

impl Iterator for Factorial {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let result = Some(self.cache.iter().fold(1, |acc, i| acc * i));
        self.i += 1;
        self.cache.push(self.i);
        result
    }
}

pub fn factorial(num: u64) -> u64 {
    Factorial::new().nth(num as usize).unwrap()
    // Complete this function to return the factorial of num
    // Do not use:
    // - early returns (using the `return` keyword explicitly)
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
