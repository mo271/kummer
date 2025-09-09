# Kummer


This repository contains a Rust library and two command-line tools for investigating divisibility properties of **central binomial coefficients**.

The primary goal of these tools is to find pairs of numbers $(n, k)$ such that the central binomial coefficients $\binom{2n}{n}$ and $\binom{2(n+k)}{n+k}$ share the exact same set of prime factors.

* **`kummer`:** A parallel search tool to find qualifying $n$ values for a given difference $k$.
* **`check_pair`:** A simple program to verify if any specific pair $(n, n+k)$ satisfies the condition.


## Background

From [Kummer's Theorem](https://en.wikipedia.org/wiki/Kummer%27s_theorem) one easily obtains the condition:

> For all primes $p$, the central binomial coefficient $\binom{2n}{n}$ is divisible by $p$ if and only if in the base $p$ representation of $n$ there is a digit $\ge\frac{p}{2}$. 

To check if two central binomials $\binom{2n}{n}$ and $\binom{2m}{n}$ have the same set of prime factors, we go through all primes $p$ with $p\le \max\{2n, 2m\}$, and go through the base $p$ digits of both $m$ and $n$ (simultaneously) and if there ever is a $p$ where the condition is different for $n$ and $m$, we stop, otherwise we have found a pair with the same prime factors. 

## How to run it

First you need to [set up rust/cargo](https://www.rust-lang.org/tools/install) 

### The `kummer` binary


```
A program to search for numbers 'n' where the pair (n, n + k) satisfies the Kummer condition

Usage: kummer [OPTIONS] <K>

Arguments:
  <K>  The offset 'k' to check for the primary pair (n, n + k)

Options:
  -l, --limit <LIMIT>  Optional upper limit for the search (exclusive)
  -q, --quiet          Suppress informational output
  -v, --verbose...     Increase verbosity. Use -v for debug messages
  -h, --help           Print help
  -V, --version        Print version
```

For instance, we can find all pairs of binomial coefficients $\binom{2n}{n}$ and $\binom{2(n + 3)}{n + 3}$, where $n < 10000000$ (in less than 1s). It is important to pass the `--release` argument to cargo, because the default `debug` build is much slower.
```bash
$ cargo run --release --bin kummer -- 3 --limit 10000000 
    Finished `release` profile [optimized + debuginfo] target(s) in 0.06s
     Running `target/release/kummer 3 --limit 10000000`
[2025-09-09T08:43:06Z INFO  kummer] Starting search with k=3 and initial chunk size 65536.
[2025-09-09T08:43:06Z INFO  kummer] Processing chunk: n = 0 to 65535 (size 65536)...
[2025-09-09T08:43:06Z INFO  kummer] ...-> Chunk processed in 22.953999ms
[2025-09-09T08:43:06Z INFO  kummer] Processing chunk: n = 65536 to 196607 (size 131072)...
[2025-09-09T08:43:06Z INFO  kummer] ...-> Chunk processed in 9.530029ms
[2025-09-09T08:43:06Z INFO  kummer] Processing chunk: n = 196608 to 458751 (size 262144)...
[2025-09-09T08:43:06Z INFO  kummer] ...-> Chunk processed in 10.782848ms
[2025-09-09T08:43:06Z INFO  kummer] Processing chunk: n = 458752 to 983039 (size 524288)...
[2025-09-09T08:43:06Z INFO  kummer] ...-> Chunk processed in 16.282519ms
[2025-09-09T08:43:06Z INFO  kummer] Processing chunk: n = 983040 to 2031615 (size 1048576)...
[2025-09-09T08:43:06Z INFO  kummer] ...-> Chunk processed in 34.815798ms
[2025-09-09T08:43:06Z INFO  kummer] Processing chunk: n = 2031616 to 4128767 (size 2097152)...
3894942
[2025-09-09T08:43:06Z INFO  kummer]   -> Found n=3894942: intermediate checks for k=1..2: [true, true]
[2025-09-09T08:43:06Z INFO  kummer] ...-> Chunk processed in 84.528874ms
[2025-09-09T08:43:06Z INFO  kummer] Processing chunk: n = 4128768 to 8323071 (size 4194304)...
6218569
[2025-09-09T08:43:06Z INFO  kummer]   -> Found n=6218569: intermediate checks for k=1..2: [true, true]
4505065
[2025-09-09T08:43:06Z INFO  kummer]   -> Found n=4505065: intermediate checks for k=1..2: [true, true]
7506679
[2025-09-09T08:43:06Z INFO  kummer]   -> Found n=7506679: intermediate checks for k=1..2: [true, true]
[2025-09-09T08:43:06Z INFO  kummer] ...-> Chunk processed in 206.133486ms
[2025-09-09T08:43:06Z INFO  kummer] Processing chunk: n = 8323072 to 9999999 (size 1676928)...
8879450
[2025-09-09T08:43:06Z INFO  kummer]   -> Found n=8879450: intermediate checks for k=1..2: [true, true]
[2025-09-09T08:43:06Z INFO  kummer] ...-> Chunk processed in 127.393962ms
[2025-09-09T08:43:06Z INFO  kummer] Search complete.
```
The logging to stderr, also shows when it found a valid pair $(n, n + k)$ if the pairs $(n, n + 1), (n, n + 2), \dots, (n, n + k - 1)$ are valid, i.e. if the central binomials $\binom{2n}{n}$ and $\binom{2(n + 1)}{n + 1}$, etc. share the same set of prime factors.

If you are only interested in the sequence of numbers, you can get them by redirecting stderr: 

```bash
$ cargo run --release --bin kummer -- 3 --limit 10000000 2> /dev/null
3894942
4505065
7506679
6218569
8879450
```

### The `check_pair` binary

```
A simple program to check the Kummer condition for a pair of numbers (n, n + k)

Usage: check_pair [OPTIONS] <N> <K>

Arguments:
  <N>  The first number in the pair (n)
  <K>  The offset for the second number (k)

Options:
  -e, --exclude <PRIME>  A prime number to exclude from the check (can be used multiple times)
  -h, --help             Print help
  -V, --version          Print version
```

For instance you can check (in less than 1min) that for $n = 15555748327$, the central binomial coefficients $\binom{2n}{n}$ and $\binom{2(n + 5)}{n + 5}$ have the same set of prime factors. 

```
$ cargo run --release --bin check_pair -- 15555748327 5
Checking pair of central binomials (15555748327, 15555748332) for odd primes factors <= 31111496664 ...
✅ Success! Their binomial coefficients have the same prime factors.
```

Optionally, you can check if they have the same prime factors, except for an exceptional set of primes:
```bash
$ cargo run --release --bin check_pair -- 2381725 2
    Finished `release` profile [optimized + debuginfo] target(s) in 0.05s
     Running `target/release/check_pair 2381725 2`
Checking pair of central binomials (2381725, 2381727) for odd primes factors <= 4763454 ...
✅ Success! Their binomial coefficients have the same prime factors.⏎                                                                                                                            
$ cargo run --release --bin check_pair -- 2381725 1
    Finished `release` profile [optimized + debuginfo] target(s) in 0.05s
     Running `target/release/check_pair 2381725 1`
Checking pair of central binomials (2381725, 2381726) for odd primes factors <= 4763452 ...
❌ Nope. Their binomial coefficients have the different prime factors.⏎                                                                                                                          
$ cargo run --release --bin check_pair -- 2381725 1 --exclude 3
    Finished `release` profile [optimized + debuginfo] target(s) in 0.05s
     Running `target/release/check_pair 2381725 1 --exclude 3`
Checking pair of central binomials (2381725, 2381726) for odd primes factors <= 4763452 ...
✅ Success! Their binomial coefficients have the same prime factors. (Not checking the primes from [3])
```

In [./src/lib.rs](./src/lib.rs) there are a few interesting examples as test cases. Those can be run with 
```
time cargo test --release
```
or if you also want to test the ones that take a longer time with 
```
time cargo test --release -- --include-ignored
```

## Disclaimer

This is not an officially supported Google product. This project is not
eligible for the [Google Open Source Software Vulnerability Rewards
Program](https://bughunters.google.com/open-source-security).
