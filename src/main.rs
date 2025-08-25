use std::time::Instant;

use clap::Parser;
use kummer::check_kummer_condition;
use rayon::prelude::*;

/// A program to search for numbers 'n' where the pair (n, n + k) satisfies the Kummer condition.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The offset 'k' to check for the primary pair (n, n + k).
    k: u64,

    /// Optional upper limit for the search (exclusive). Runs indefinitely if not set.
    #[arg(short, long)]
    limit: Option<u64>,
}

fn main() {
    let args = Args::parse();
    let k = args.k;

    let mut current_n = 0u64;
    let mut chunk_size = 1u64 << 16;

    println!(
        "Starting search with k={} and initial chunk size {}. Press Ctrl+C to stop.",
        k, chunk_size
    );

    loop {
        let start_n = current_n;
        let end_n = start_n.saturating_add(chunk_size);

        if let Some(l) = args.limit
            && start_n >= l
        {
            break;
        }

        // Cap the end of the chunk if a limit is set.
        let current_end = args.limit.map_or(end_n, |l| l.min(end_n));

        println!(
            "Processing chunk: n = {} to {} (size {})...",
            start_n,
            current_end - 1,
            current_end - start_n
        );
        let chunk_timer = Instant::now();
        // Create a sieve that is large enough for this entire chunk.
        let max_sieve_val = 2 * (current_end.saturating_add(k));
        let sieve = primal::Sieve::new(max_sieve_val as usize);

        (start_n..current_end).into_par_iter().for_each(|n| {
            if check_kummer_condition(n, n + k, &sieve, &[]) {
                let intermediate_results: Vec<bool> = (1..k)
                    .map(|j| check_kummer_condition(n, n + j, &sieve, &[]))
                    .collect();

                println!(
                    "Found n={}: intermediate checks for k=1..{}: {:?}",
                    n,
                    k - 1,
                    intermediate_results
                );
            }
        });
        println!("...-> Chunk processed in {:?}", chunk_timer.elapsed());
        current_n = current_end;
        chunk_size = chunk_size.saturating_mul(2);
    }

    println!("Search complete.");
}
