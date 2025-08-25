pub fn central_divide(mut n: u64, p: u64) -> bool {
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

pub fn check_kummer_condition(
    n: u64,
    m: u64,
    sieve: &primal::Sieve,
    exclude_primes: &[u64],
) -> bool {
    let max_prime = 2 * m;
    let found_a_failure = sieve
        // Is is enough to check starting from 3 here, since all central binoms are even.
        .primes_from(3)
        .map(|p| p as u64)
        .take_while(|&p| p <= max_prime)
        .filter(|p| !exclude_primes.contains(p))
        .any(|p| central_divide(n, p) != central_divide(m, p));

    !found_a_failure
}

#[cfg(test)]
mod tests {
    use super::*;
    use primal::Sieve;
    use std::collections::HashSet;

    #[test]
    fn dist_zero_pairs() {
        // Any number compared with itself should be true.
        let sieve = Sieve::new(1000);
        for n in 0..=500 {
            assert!(
                check_kummer_condition(n, n + 0, &sieve, &[]),
                "distance zero check failed for n = {}",
                n
            );
        }
    }

    #[test]
    fn dist_one_pairs() {
        // From https://oeis.org/A129515
        const TRUE_CASES: &[u64] = &[
            87, 199, 237, 467, 607, 967, 1127, 1319, 1483, 1903, 1943, 2012, 2047, 2287, 2348,
            2359, 2464, 2479, 2495, 2507, 2623, 2645, 2719, 3349, 3467, 3514, 3568, 3629, 3633,
            3712, 3847, 3919, 4088, 4224, 4287, 4360, 4479, 4927, 4987, 5087, 5167, 5224, 5669,
        ];

        let max_n = *TRUE_CASES.iter().max().unwrap();
        let sieve = Sieve::new(2 * (max_n as usize + 1));

        let true_set: HashSet<u64> = TRUE_CASES.iter().cloned().collect();

        for n in 1..=max_n {
            assert_eq!(
                check_kummer_condition(n, n + 1, &sieve, &[]),
                true_set.contains(&n)
            );
        }
    }

    #[test]
    #[ignore]
    fn dist_two_pairs() {
        // From https://oeis.org/A129515
        const TRUE_CASES: &[u64] = &[
            10003, 17374, 47487, 111547, 121602, 129784, 133161, 142239, 142781, 143762, 152190,
            213425, 233332, 250711, 253273, 266843, 288062, 291786, 295135, 303772, 306008, 356277,
        ];

        let max_n = *TRUE_CASES.iter().max().unwrap();
        let sieve = Sieve::new(2 * (max_n as usize + 1));

        let true_set: HashSet<u64> = TRUE_CASES.iter().cloned().collect();

        for n in 1..=max_n {
            let result = check_kummer_condition(n, n + 2, &sieve, &[]);

            if true_set.contains(&n) {
                assert!(result);
                // For those numbers (smallest counterexample to that is 2381725, see below),
                // we also have that the number in between has the same prime factors for its central binomial coefficient.
                assert!(check_kummer_condition(n, n + 1, &sieve, &[]));
                assert!(check_kummer_condition(n + 1, n, &sieve, &[]));
            } else {
                assert!(!result);
            }
        }
    }

    #[test]
    #[ignore]
    fn dist_two_pairs_with_gap() {
        const GAP_CASES: &[u64] = &[2381725, 129320551, 136226152, 177560668, 177687550];
        let max_n = *GAP_CASES.iter().max().unwrap();
        let sieve = Sieve::new(2 * (max_n as usize + 1));
        for &n in GAP_CASES {
            assert!(check_kummer_condition(n, n + 2, &sieve, &[]));
            assert!(!check_kummer_condition(n, n + 1, &sieve, &[]));
            assert!(!check_kummer_condition(n + 1, n + 2, &sieve, &[]));
            assert!(check_kummer_condition(n, n + 1, &sieve, &[3]));
            assert!(check_kummer_condition(n + 1, n + 2, &sieve, &[3]));
        }
    }

    #[test]
    #[ignore]
    fn dist_three_pairs() {
        // From https://oeis.org/A129515
        const TRUE_CASES: &[u64] = &[3894942, 4505065, 6218569, 7506679, 8879450];

        let max_n = *TRUE_CASES.iter().max().unwrap();
        let sieve = Sieve::new(2 * (max_n as usize + 1));

        let true_set: HashSet<u64> = TRUE_CASES.iter().cloned().collect();

        for n in 1..=max_n {
            let result = check_kummer_condition(n, n + 3, &sieve, &[]);

            if true_set.contains(&n) {
                assert!(result);
                // For those numbers (smallest counterexample to that is 1_488_831_402, see below),
                // we also have that the numbers in between have the same prime factors for its central binomial coefficient.
                assert!(check_kummer_condition(n, n + 1, &sieve, &[]));
                assert!(check_kummer_condition(n, n + 2, &sieve, &[]));
                assert!(check_kummer_condition(n + 1, n + 3, &sieve, &[]));
                assert!(check_kummer_condition(n + 2, n + 3, &sieve, &[]));
            } else {
                assert!(!result);
            }
        }
    }

    #[test]
    #[ignore]
    fn dist_three_pairs_with_gap() {
        const GAP_CASES: &[u64] = &[1_488_831_402];
        let max_n = *GAP_CASES.iter().max().unwrap();
        let sieve = Sieve::new(2 * (max_n as usize + 1));
        for &n in GAP_CASES {
            assert!(check_kummer_condition(n, n + 3, &sieve, &[]));
            assert!(!check_kummer_condition(n, n + 1, &sieve, &[]));
            assert!(!check_kummer_condition(n, n + 2, &sieve, &[]));
            assert!(!check_kummer_condition(n + 1, n + 3, &sieve, &[]));
            assert!(!check_kummer_condition(n + 2, n + 3, &sieve, &[]));
            assert!(check_kummer_condition(n + 1, n + 2, &sieve, &[]));
            assert!(check_kummer_condition(n, n + 1, &sieve, &[5]));
            assert!(check_kummer_condition(n, n + 2, &sieve, &[5]));
            assert!(check_kummer_condition(n + 2, n + 3, &sieve, &[5]));
            assert!(check_kummer_condition(n + 1, n + 3, &sieve, &[5]));
        }
    }

    // TODO(firsching): add more test: dist_four_pairs, dist_four_pairs_with_gap, dist_five_pairs
}
