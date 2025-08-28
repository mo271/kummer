// Copyright 2025 Google LLC

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     https://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clap::Parser;
use kummer::{check_kummer_condition, get_divisor};

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
    let primes_and_divisor: Vec<_> = sieve
        .primes_from(3)
        .map(|x| (x as u64, get_divisor(x as u64)))
        .collect();
    println!(
        "Checking pair of central binomials ({}, {}) for odd primes factors <= {} ...",
        n, m, max_prime
    );

    let condition_met = check_kummer_condition(n, m, &primes_and_divisor, &args.exclude_primes);

    if condition_met {
        print!("✅ Success! Their binomial coefficients have the same prime factors.");
    } else {
        print!("❌ Nope. Their binomial coefficients have the different prime factors.");
    }
    if !args.exclude_primes.is_empty() {
        println!(" (Not checking the primes from {:?})", args.exclude_primes)
    }
}
