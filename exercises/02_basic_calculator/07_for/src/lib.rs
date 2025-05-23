// Rewrite the factorial function using a `for` loop.
pub fn factorial(n: u32) -> u32 {
    //todo!()
    let mut sum: u32 = 1;
    for i in 1..=n {
        sum *= i;
    }
    sum
}

/// Compute the nth Fibonacci number using an iterative approach.
pub fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let next = a + b;
        a = b;
        b = next;
    }
    b
}

#[cfg(test)]
mod tests {
    use crate::{factorial, fibonacci};

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn fib_zero() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn fib_one() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn fib_two() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn fib_three() {
        assert_eq!(fibonacci(3), 2);
    }

    #[test]
    fn fib_ten() {
        assert_eq!(fibonacci(10), 55);
    }
}
