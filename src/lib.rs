// This magic number allows replacing division with multiplication.
// This step is now done on every function call.
pub fn get_divisor(p: u64) -> u64 {
    u64::MAX / p + 1
}

pub fn central_divide_match(mut n: u64, mut m: u64, p: u64, m_inv: u64) -> bool {
    // Pre-calculate the values needed for the loop.
    let p_half = p / 2;

    let mut ndiv = false;
    let mut mdiv = false;

    while (n > 0 || m > 0) && !(ndiv && mdiv) && !(n == m && ndiv == mdiv) {
        // Calculate quotient `q = n / p` using 128-bit multiplication.
        // The result is the "high" 64 bits of the full 128-bit product.
        let qn = ((n as u128 * m_inv as u128) >> 64) as u64;
        let qm = ((m as u128 * m_inv as u128) >> 64) as u64;

        // Calculate remainder `digit = n % p` using the quotient.
        let digitn = n - qn * p;
        let digitm = m - qm * p;

        ndiv |= digitn > p_half;
        mdiv |= digitm > p_half;

        // Set `n` to the quotient for the next iteration.
        n = qn;
        m = qm;
    }

    ndiv == mdiv
}

pub fn check_kummer_condition(
    n: u64,
    m: u64,
    primes: &[(u64, u64)],
    exclude_primes: &[u64],
) -> bool {
    let max_prime = 2 * m;
    let found_a_failure = primes
        .iter()
        .copied()
        .take_while(|(p, _)| *p <= max_prime)
        .filter(|(p, _)| !exclude_primes.contains(p))
        .any(|(p, m_inv)| !central_divide_match(n, m, p, m_inv));

    !found_a_failure
}

#[cfg(test)]
mod tests {
    use super::*;
    use primal::Sieve;

    fn run_distance_test(distance: u64, true_cases: &[u64]) {
        if true_cases.is_empty() {
            return;
        }
        let max_n = *true_cases.iter().max().unwrap();
        let sieve = Sieve::new(2 * (max_n as usize + distance as usize + 1));
        let primes_and_divisor: Vec<_> = sieve
            .primes_from(3)
            .map(|x| (x as u64, get_divisor(x as u64)))
            .collect();

        for &n in true_cases {
            let result = check_kummer_condition(n, n + distance, &primes_and_divisor, &[]);
            assert!(result, "Main check failed for n = {}", n);

            // If the main check passes, verify all intermediate pairs automatically.
            for i in 0..distance {
                for j in (i + 1)..=distance {
                    let u = n + i;
                    let v = n + j;
                    assert!(
                        check_kummer_condition(u, v, &primes_and_divisor, &[]),
                        "Intermediate check failed for (u, v) = ({}, {}) on base n = {}",
                        u,
                        v,
                        n
                    );
                }
            }
        }
    }

    fn run_gap_test(distance: u64, gap_prime: u64, gap_cases: &[u64]) {
        if gap_cases.is_empty() {
            return;
        }
        let max_n = *gap_cases.iter().max().unwrap();
        let sieve = Sieve::new(2 * (max_n as usize + distance as usize + 10));
        let primes_and_divisor: Vec<_> = sieve
            .primes_from(3)
            .map(|x| (x as u64, get_divisor(x as u64)))
            .collect();
        let ignored_prime_slice = &[gap_prime];

        for &n in gap_cases {
            // Assert the main condition holds.
            assert!(
                check_kummer_condition(n, n + distance, &primes_and_divisor, &[]),
                "Main gap check failed for n = {}",
                n
            );

            // Iterate through ALL intermediate pairs (excluding the main one).
            for i in 0..=distance {
                for j in (i + 1)..=distance {
                    if i == 0 && j == distance {
                        // Skip the main pair itself.
                        continue;
                    }

                    let u = n + i;
                    let v = n + j;
                    let result = check_kummer_condition(u, v, &primes_and_divisor, &[]);

                    let crosses_boundary = ([n, n + distance].contains(&u)
                        && ![n, n + distance].contains(&v))
                        || (![n, n + distance].contains(&u) && [n, n + distance].contains(&v));
                    assert_eq!(result, !crosses_boundary);

                    if crosses_boundary {
                        //  If it fails, assert it passes when ignoring the gap prime.
                        assert!(
                            check_kummer_condition(u, v, &primes_and_divisor, ignored_prime_slice),
                            "Gap pair ({}, {}) for base n = {} failed even with ignored prime {}",
                            u,
                            v,
                            n,
                            gap_prime
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn dist_zero_pairs() {
        // This test is simple and unique enough to not need a helper.
        let sieve = Sieve::new(1000);
        let primes_and_divisor: Vec<_> = sieve
            .primes_from(3)
            .map(|x| (x as u64, get_divisor(x as u64)))
            .collect();
        for n in 0..=500 {
            assert!(
                check_kummer_condition(n, n, &primes_and_divisor, &[]),
                "distance zero check failed for n = {}",
                n
            );
        }
    }

    #[test]
    fn dist_one_pairs() {
        const TRUE_CASES: &[u64] = &[
            87, 199, 237, 467, 607, 967, 1127, 1319, 1483, 1903, 1943, 2012, 2047, 2287, 2348,
            2359, 2464, 2479, 2495, 2507, 2623, 2645, 2719, 3349, 3467, 3514, 3568, 3629, 3633,
            3712, 3847, 3919, 4088, 4224, 4287, 4360, 4479, 4927, 4987, 5087, 5167, 5224, 5669,
        ];
        run_distance_test(1, TRUE_CASES);
    }

    #[test]
    fn dist_two_pairs() {
        const TRUE_CASES: &[u64] = &[
            10003, 17374, 47487, 111547, 121602, 129784, 133161, 142239, 142781, 143762, 152190,
            213425, 233332, 250711, 253273, 266843, 288062, 291786, 295135, 303772, 306008, 356277,
        ];
        run_distance_test(2, TRUE_CASES);
    }

    #[test]
    fn dist_two_pairs_with_gap() {
        const GAP_CASES: &[u64] = &[2381725, 129320551, 136226152, 177560668, 177687550];
        run_gap_test(2, 3, GAP_CASES);
    }

    #[test]
    fn dist_three_pairs() {
        const TRUE_CASES: &[u64] = &[3894942, 4505065, 6218569, 7506679, 8879450];
        run_distance_test(3, TRUE_CASES);
    }

    #[test]
    #[ignore]
    fn dist_three_pairs_with_gap() {
        const GAP_CASES: &[u64] = &[1_488_831_402];
        run_gap_test(3, 5, GAP_CASES);
    }
    #[test]
    #[ignore]
    fn dist_four_pairs() {
        const TRUE_CASES: &[u64] = &[94_961_106, 320_592_237, 530_571_772, 413_000_786];
        run_distance_test(4, TRUE_CASES);
    }
    #[test]
    #[ignore]
    fn dist_four_pairs_with_gap() {
        const GAP_CASES: &[u64] = &[39_561_491_884];
        run_gap_test(4, 5, GAP_CASES);
    }
    #[test]
    #[ignore]
    fn dist_five_pairs() {
        const TRUE_CASES: &[u64] = &[15_555_748_327, 16_981_964_421];
        run_distance_test(5, TRUE_CASES);
    }
}
