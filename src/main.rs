use rayon::prelude::*;

use kummer::check_kummer_condition;

fn main() {
    let limit = 1_600_0000_u64;

    // The sieve is created once, large enough for the entire run.
    let max_sieve_size = (2 * (limit + 1)) as usize;
    let sieve = primal::Sieve::new(max_sieve_size);
    println!("Sieve created. Starting parallel search...");

    let results: Vec<(u64, bool)> = (0..limit)
        .into_par_iter()
        .filter_map(|n| {
            if check_kummer_condition(n, n + 2, &sieve, &[]) {
                let second_cond = check_kummer_condition(n, n + 1, &sieve, &[]);
                // Return the results to be collected
                Some((n, second_cond))
            } else {
                None
            }
        })
        .collect();

    // Printing is done after all computation is finished.
    for (n, second_cond_result) in results {
        let n1 = second_cond_result;
        let n2 = check_kummer_condition(n, n + 2, &sieve, &[]);
        if !n1 {
            println!("{} {} {} ", n, n1, n2);
        }
    }

    println!("Search complete up to {limit}");
}
