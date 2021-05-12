// iterators4.rs


pub fn factorial(num: u64) -> u64 {
    // Cara 1
    // (1..=num).product()

    // Cara 2
    (1..=num).fold(1, |cum, n| cum * n)
}

#[cfg(test)]
mod tests {
    use super::*;

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
