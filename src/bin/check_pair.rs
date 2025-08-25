use clap::Parser;
use kummer::check_kummer_condition;

/// A simple program to check the Kummer condition for a pair of numbers (n, n + k).
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The first number in the pair (n).
    n: u64,

    /// The offset for the second number (k).
    k: u64,

    /// A prime number to exclude from the check (can be used multiple times).
    #[arg(short, long = "exclude", value_name = "PRIME")]
    exclude_primes: Vec<u64>,
}
fn main() {
    let args = Args::parse();

    let n = args.n;
    let k = args.k;
    let m = n + k;

    let max_prime = 2 * m;
    let sieve = primal::Sieve::new(max_prime as usize);
    println!(
        "Checking pair of central binomials ({}, {}) for odd primes factors <= {} ...",
        n, m, max_prime
    );

    let condition_met = check_kummer_condition(n, m, &sieve, &args.exclude_primes);

    if condition_met {
        print!("✅ Success! Their binomial coefficients have the same prime factors.");
    } else {
        print!("❌ Nope. Their binomial coefficients have the different prime factors.");
    }
    if !args.exclude_primes.is_empty() {
        println!(" (Not checking the primes from {:?})", args.exclude_primes)
    }
}
