fn central_divide(mut n: u64, p: u64) -> bool {
    if n == 0 {
        return false;
    }

    let p_half = p / 2;

    while n > 0 {
        let digit = n % p;
        if digit > p_half {
            return true;
        }
        n /= p;
    }
    false
}

fn check_kummer_condition(n: u64, m: u64) -> bool {
    let max_prime : usize = (2 * m).max(2 * n).try_into().unwrap();

    let sieve = primal::Sieve::new(max_prime as usize);

    for p in sieve.primes_from(2) {
        if central_divide(n, p as u64) != central_divide(m, p as u64) {
            return false;
        }
    }

    true
}

fn main() {
    let limit = 10_000_000;

    for n in 0..limit {
        if check_kummer_condition(n, n + 2) {
            println!("{} {}", n, check_kummer_condition(n, n+1));
        }
    }
    println!("Search complete.");
}