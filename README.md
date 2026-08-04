# Kummer


This repository contains a Rust library and two command-line tools for investigating divisibility properties of **central binomial coefficients**.

The primary goal of these tools is to find pairs of numbers $(n, k)$ such that the central binomial coefficients $\binom{2n}{n}$ and $\binom{2(n+k)}{n+k}$ share the exact same set of prime factors.

* **`kummer`:** A parallel search tool to find qualifying $n$ values for a given difference $k$.
* **`check_pair`:** A simple program to verify if any specific pair $(n, n+k)$ satisfies the condition.


## Background

From [Kummer's Theorem](https://en.wikipedia.org/wiki/Kummer%27s_theorem) one easily obtains the condition:

> For all primes $p$, the central binomial coefficient $\binom{2n}{n}$ is divisible by $p$ if and only if in the base $p$ representation of $n$ there is a digit $\ge\frac{p}{2}$. 

To check if two central binomials $\binom{2n}{n}$ and $\binom{2m}{n}$ have the same set of prime factors, we go through all primes $p$ with $p\le \max\{2n, 2m\}$, and go through the base $p$ digits of both $m$ and $n$ (simultaneously) and if there ever is a $p$ where the condition is different for $n$ and $m$, we stop, otherwise we have found a pair with the same prime factors.

The idea for this search came from AlphaProof finding a counterexample to a statement on https://www.erdosproblems.com/730, see comments there.

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

## Some results from running this

### Pairs $(n, n + 2)$

AlphaProof found the example $n = 10003$, where the pair $(n, n + 2)$ satisfies the binomial condition.  
A search yields many more such examples, with $10003$ indeed being the smallest:
$10003, 17374, 47487, 111547, 121602, 129784, 133161, 142239, 142781, 143762, 152190, 213425, 233332, 250711, 253273, 266843, 288062, 291786, 295135, 303772, 306008, 356277$.

For all of those the pair $(n, n + 1)$ (and hence also $(n + 1, n + 2)$) satisfies the binomial condition.

There are also pairs $(n, n + 2)$ such that $(n, n + 1)$ (and hence also $(n + 1, n + 2)$) does not satisfy the binomial condition. The smallest such are:

$2381725, 129320551, 136226152, 177560668, 177687550$.

For those the prime factors for the central binomial in the middle only differ by a prime factor of $3$ from the ones for the outer ones. See Section “Transitivity failures for $(n, n + 2)$” for a proof that this failure must occur in this form for $p = 3$.

### Pairs $(n, n + 3)$

There are numbers $n$ where all numbers $n, n + 1, n + 2, n + 3$ have central binomials with the same prime factors. The smallest such numbers are:

$3894942, 4505065, 6218569, 7506679, 8879450$.

There are numbers where $(n, n + 3)$ satisfy the binomial condition, but it is not satisfied for $(n, n + 1)$ nor $(n, n + 2)$ (and hence also not for $(n + 1, n + 3)$ and $(n + 2, n + 3)$).  

$1488831402$ and $8549304052$ are examples.  

In those cases, the binomial condition is satisfied for $(n + 1, n + 2)$. The prime factors for the central binomials in the middle only differ by a prime factor of $5$ from the ones for the outer ones. ($1488831402$ in base $5$ is $11022120101102$).

$1723472893$ is an example where $(n, n + 3)$ satisfies the binomial condition and also $(n, n + 1)$, but not $(n, n + 2)$.

### Pairs $(n, n + 4)$

There are numbers $n$ where all numbers $n, n + 1, n + 2, n + 3, n + 4$ have central binomials with the same prime factors. The smallest such numbers are:

$94961106, 320592237, 530571772, 413000786$.

There are numbers where $(n, n + 4)$ satisfy the binomial condition, but it is not satisfied for $(n, n + 1)$ nor $(n, n + 2)$ nor $(n, n + 3)$ (and hence also not for $(n + 1, n + 4)$, $(n + 2, n + 4)$ and $(n + 3, n + 4)$).  

$39561491884$ is such an example.  

In this case, the binomial condition is satisfied for $(n + 1, n + 2)$ and $(n + 2, n + 3)$ (and hence also by $(n + 1, n + 3)$ by transitivity). The prime factors for the central binomials in the middle only differ by a prime factor of $5$ from the ones for the outer ones.

### Pairs $(n, n + 5)$

There are numbers $n$ where all numbers $n, n + 1, n + 2, n + 3, n + 4, n + 5$ have central binomials with the same prime factors. The smallest such numbers are:

$15555748327, 16981964421$.

### Pairs $(n, n + 6)$

There are no such pairs for $n \le 137438887936$.

### Transitivity failures for $(n, n + 2)$

*(discussion by Salvatore Mercuri)*

Here we discuss a necessary condition for which the binomial condition can occur of gap size $2$ outside of transitivity. Specifically, this is the case where the binomial condition is satisfied for $(n, n + 2)$ but not for $(n, n + 1)$ and $(n + 1, n + 2)$. The proof of this makes use of the Kummer condition:

**Lemma.** An odd prime $p$ does not divide the central binomial coefficient $\binom{2n}{n}$ if and only if the base $p$ digits of $n$ are all $\le \lfloor p/2 \rfloor$.

**Proposition.** If the binomial condition is satisfied for $(n, n + 2)$, then it is also satisfied for $(n, n + 1)$ and $(n + 1, n + 2)$ unless $n \equiv 1 \pmod 9$, in which case it is possible for the prime $3$ to divide the inner central binomial coefficient at $n + 1$ but not the two outer central binomial coefficients at $n$ and $n + 2$.

**Proof.**  
Note that $2$ is always a divisor of the central binomial coefficients so we may in the following only consider odd primes. Let $S_n$ be the set of prime divisors of the central binomial coefficients at $n$. By assumption $S_n = S_{n + 2} =: S$.  

*Claim:* $S \subseteq S_{n + 1}$.

Take an odd prime $p \notin S_{n + 1}$. By the Lemma, we must have that all base $p$ digits of $n + 1$ are $\le \lfloor p/2 \rfloor$. If the last digit $a_0$ is non-zero, then the last digit of $n$ is $a_0 - 1$, so this also satisfies the Lemma and so $p \notin S$. If the last digit $a_0$ is zero, then the last digit of $n$ is $p - 1$, which exceeds $\lfloor p/2 \rfloor$, but the base $p$ digits of $n + 1$ end in $1$, so these digits also satisfy the Lemma and so $p \notin S$.

*Claim:* $S_{n + 1} \subseteq S$ if $n \not\equiv 1 \pmod 3$.

Take an odd prime $p \notin S$ and $p \neq 3$. The base $p$ digits of $n$ are all $\le \lfloor p/2 \rfloor$. Let $a_0$ be the last base $p$ digit of $n$. Because $p > 3$, the last digits of $n + 1$ and $n + 2$ are, respectively, $a_0 + 1$ and $a_0 + 2$. We must also have $a_0 + 2 \le \lfloor p/2 \rfloor$. Hence $a_0 + 1 \le \lfloor p/2 \rfloor$ and the Lemma is satisfied for $n + 1$, so $p \notin S_{n + 1}$. Now consider $p = 3$. We have either $n \equiv 0 \pmod 3$ or $n \equiv 1 \pmod 3$ by the Lemma, and so by assumption $n \equiv 0 \pmod 3$, hence $n + 1$ then satisfies the Lemma, so $3 \notin S_{n + 1}$.

*Claim:* If $3 \notin S$ and $n \equiv 1 \pmod 3$, then we must moreover have $n \equiv 1 \pmod 9$.

If $n \equiv 4$ or $7 \pmod 9$, then $n+2 \equiv 0$ or $3 \pmod 9$. In base $3$, the two least significant digits of $n$ would end in $11$ or $21$, and the digits of $n + 2$ would end in $01$ or $12$. In both cases, $n+2$ contains a digit $\ge 2 = \lfloor 3/2 \rfloor + 1$, which contradicts the Kummer condition for $3 \notin S_{n+2}$. Thus, we must have $n \equiv 1 \pmod 9$. This completes the proof.


## Disclaimer

This is not an officially supported Google product. This project is not
eligible for the [Google Open Source Software Vulnerability Rewards
Program](https://bughunters.google.com/open-source-security).
