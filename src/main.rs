// In main.rs

use clap::Parser;
use kummer::{check_kummer_condition, get_divisor};
use log::{LevelFilter, info};
use rayon::prelude::*;
use std::time::Instant; // Import log macros and LevelFilter

/// A program to search for numbers 'n' where the pair (n, n + k) satisfies the Kummer condition.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The offset 'k' to check for the primary pair (n, n + k).
    k: u64,

    /// Optional upper limit for the search (exclusive).
    #[arg(short, long)]
    limit: Option<u64>,

    /// Suppress informational output.
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    quiet: bool,

    /// Increase verbosity. Use -v for debug messages.
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() {
    let args = Args::parse();

    // Set up the logger
    let log_level = if args.quiet {
        LevelFilter::Off
    } else {
        match args.verbose {
            0 => LevelFilter::Info,  // Default
            1 => LevelFilter::Debug, // -v
            _ => LevelFilter::Trace, // -vv, -vvv, etc.
        }
    };
    env_logger::Builder::new().filter_level(log_level).init();

    let k = args.k;
    let mut current_n = 0u64;
    let mut chunk_size = 1u64 << 16;

    info!(
        "Starting search with k={} and initial chunk size {}.",
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

        let current_end = args.limit.map_or(end_n, |l| l.min(end_n));

        info!(
            "Processing chunk: n = {} to {} (size {})...",
            start_n,
            current_end - 1,
            current_end - start_n
        );

        let chunk_timer = Instant::now();
        let max_sieve_val = 2 * (current_end.saturating_add(k));
        let sieve = primal::Sieve::new(max_sieve_val as usize);

        let primes_and_divisor: Vec<_> = sieve
            .primes_from(3)
            .map(|x| (x as u64, get_divisor(x as u64)))
            .collect();

        (start_n..current_end).into_par_iter().for_each(|n| {
            if check_kummer_condition(n, n + k, &primes_and_divisor, &[]) {
                // The primary output, sent to stdout
                println!("{}", n);

                let intermediate_results: Vec<bool> = (1..k)
                    .map(|j| check_kummer_condition(n, n + j, &primes_and_divisor, &[]))
                    .collect();
                info!(
                    "  -> Found n={}: intermediate checks for k=1..{}: {:?}",
                    n,
                    k - 1,
                    intermediate_results
                );
            }
        });

        info!("...-> Chunk processed in {:?}", chunk_timer.elapsed());

        current_n = current_end;
        chunk_size = chunk_size.saturating_mul(2);
    }

    info!("Search complete.");
}
