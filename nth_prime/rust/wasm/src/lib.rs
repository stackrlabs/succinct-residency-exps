/// Compute the nth prime number where n is 1-indexed (i.e. p1 = 2, p2 = 3).
#[no_mangle]
pub fn nth_prime(n: u64) -> u64 {
    let p = primal::StreamingSieve::nth_prime(n as usize);
    p as u64
}
