// Rust
// Calculates the nth Fibonacci number using an iterative approach and returns it as an Option.
// Handles potential overflow by returning None if the number exceeds u64::MAX.

fn nth_fibonacci(n: u32) -> Option<u64> {
    if n == 0 {
        return Some(0);
    }
    if n == 1 {
        return Some(1);
    }

    let mut a: u64 = 0;
    let mut b: u64 = 1;

    for _ in 2..=n {
        let next_fib = a.checked_add(b)?;
        a = b;
        b = next_fib;
    }
    Some(b)
}