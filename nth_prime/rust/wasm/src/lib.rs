/// Compute the nth prime number where n is 1-indexed (i.e. p1 = 2, p2 = 3).
#[no_mangle]
fn nth_prime(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut primes = vec![2];
    let mut candidate = 3;
    while primes.len() < n as usize {
        if primes.iter().all(|&p| candidate % p != 0) {
            primes.push(candidate);
        }
        candidate += 2;
    }
    primes[n as usize - 1]
}
