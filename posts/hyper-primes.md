---
title: 'Hyper-Primes'
date: '2025-11-17'
---
A hyper-prime is a prime number p where all prime factors and all exponents in the prime factorization of p-1 are themselves hyper-primes.

## Definition

For a prime number p, compute N = p - 1 and find its prime factorization:
N = q₁^e₁ × q₂^e₂ × ... × qₖ^eₖ
The prime p is hyper-prime if and only if:
- Each prime factor qᵢ is hyper-prime
- Each exponent eᵢ is hyper-prime

By definition, 0 and 1 are hyper-primes.

## Table

|  Prime |            Factors of p-1 |       Exponents | HyperPrime |
|--------|---------------------------|-----------------|------------|
|      0 |                           |                 |       True |
|      1 |                           |                 |       True |
|      2 | 1                         | 1^0             |       True |
|      3 | 2                         | 2^1             |       True |
|      5 | 2 × 2                     | 2^2             |       True |
|      7 | 2 × 3                     | 2^1, 3^1        |       True |
|     11 | 2 × 5                     | 2^1, 5^1        |       True |
|     13 | 2 × 2 × 3                 | 2^2, 3^1        |       True |
|     17 | 2 × 2 × 2 × 2             | 2^4             |      False |
|     19 | 2 × 3 × 3                 | 2^1, 3^2        |       True |
|     23 | 2 × 11                    | 2^1, 11^1       |       True |
|     29 | 2 × 2 × 7                 | 2^2, 7^1        |       True |
|     31 | 2 × 3 × 5                 | 2^1, 3^1, 5^1   |       True |
|     37 | 2 × 2 × 3 × 3             | 2^2, 3^2        |       True |
|     41 | 2 × 2 × 2 × 5             | 2^3, 5^1        |       True |
|     43 | 2 × 3 × 7                 | 2^1, 3^1, 7^1   |       True |
|     47 | 2 × 23                    | 2^1, 23^1       |       True |
|     53 | 2 × 2 × 13                | 2^2, 13^1       |       True |
|     59 | 2 × 29                    | 2^1, 29^1       |       True |
|     61 | 2 × 2 × 3 × 5             | 2^2, 3^1, 5^1   |       True |
|     67 | 2 × 3 × 11                | 2^1, 3^1, 11^1  |       True |
|     71 | 2 × 5 × 7                 | 2^1, 5^1, 7^1   |       True |
|     73 | 2 × 2 × 2 × 3 × 3         | 2^3, 3^2        |       True |
|     79 | 2 × 3 × 13                | 2^1, 3^1, 13^1  |       True |
|     83 | 2 × 41                    | 2^1, 41^1       |       True |
|     89 | 2 × 2 × 2 × 11            | 2^3, 11^1       |       True |
|     97 | 2 × 2 × 2 × 2 × 2 × 3     | 2^5, 3^1        |       True |
|    101 | 2 × 2 × 5 × 5             | 2^2, 5^2        |       True |
|    103 | 2 × 3 × 17                | 2^1, 3^1, 17^1  |      False |
|    107 | 2 × 53                    | 2^1, 53^1       |       True |
|    109 | 2 × 2 × 3 × 3 × 3         | 2^2, 3^3        |       True |
|    113 | 2 × 2 × 2 × 2 × 7         | 2^4, 7^1        |      False |
|    127 | 2 × 3 × 3 × 7             | 2^1, 3^2, 7^1   |       True |
|    131 | 2 × 5 × 13                | 2^1, 5^1, 13^1  |       True |
|    137 | 2 × 2 × 2 × 17            | 2^3, 17^1       |      False |
|    139 | 2 × 3 × 23                | 2^1, 3^1, 23^1  |       True |
|    149 | 2 × 2 × 37                | 2^2, 37^1       |       True |
|    151 | 2 × 3 × 5 × 5             | 2^1, 3^1, 5^2   |       True |
|    157 | 2 × 2 × 3 × 13            | 2^2, 3^1, 13^1  |       True |
|    163 | 2 × 3 × 3 × 3 × 3         | 2^1, 3^4        |      False |
|    167 | 2 × 83                    | 2^1, 83^1       |       True |
|    173 | 2 × 2 × 43                | 2^2, 43^1       |       True |
|    179 | 2 × 89                    | 2^1, 89^1       |       True |
|    181 | 2 × 2 × 3 × 3 × 5         | 2^2, 3^2, 5^1   |       True |
|    191 | 2 × 5 × 19                | 2^1, 5^1, 19^1  |       True |
|    193 | 2 × 2 × 2 × 2 × 2 × 2 × 3 | 2^6, 3^1        |      False |
|    197 | 2 × 2 × 7 × 7             | 2^2, 7^2        |       True |
|    199 | 2 × 3 × 3 × 11            | 2^1, 3^2, 11^1  |       True |
|    211 | 2 × 3 × 5 × 7             | 2^1, 3^1, 5^1, 7^1 |       True |
|    223 | 2 × 3 × 37                | 2^1, 3^1, 37^1  |       True |
|    227 | 2 × 113                   | 2^1, 113^1      |      False |
|    229 | 2 × 2 × 3 × 19            | 2^2, 3^1, 19^1  |       True |
|    233 | 2 × 2 × 2 × 29            | 2^3, 29^1       |       True |
|    239 | 2 × 7 × 17                | 2^1, 7^1, 17^1  |      False |
|    241 | 2 × 2 × 2 × 2 × 3 × 5     | 2^4, 3^1, 5^1   |      False |
|    251 | 2 × 5 × 5 × 5             | 2^1, 5^3        |       True |
|    257 | 2 × 2 × 2 × 2 × 2 × 2 × 2 × 2 | 2^8             |      False |
|    263 | 2 × 131                   | 2^1, 131^1      |       True |
|    269 | 2 × 2 × 67                | 2^2, 67^1       |       True |
|    271 | 2 × 3 × 3 × 3 × 5         | 2^1, 3^3, 5^1   |       True |
|    277 | 2 × 2 × 3 × 23            | 2^2, 3^1, 23^1  |       True |
|    281 | 2 × 2 × 2 × 5 × 7         | 2^3, 5^1, 7^1   |       True |
|    283 | 2 × 3 × 47                | 2^1, 3^1, 47^1  |       True |
|    293 | 2 × 2 × 73                | 2^2, 73^1       |       True |
|    307 | 2 × 3 × 3 × 17            | 2^1, 3^2, 17^1  |      False |
|    311 | 2 × 5 × 31                | 2^1, 5^1, 31^1  |       True |
|    313 | 2 × 2 × 2 × 3 × 13        | 2^3, 3^1, 13^1  |       True |
|    317 | 2 × 2 × 79                | 2^2, 79^1       |       True |
|    331 | 2 × 3 × 5 × 11            | 2^1, 3^1, 5^1, 11^1 |       True |
|    337 | 2 × 2 × 2 × 2 × 3 × 7     | 2^4, 3^1, 7^1   |      False |
|    347 | 2 × 173                   | 2^1, 173^1      |       True |
|    349 | 2 × 2 × 3 × 29            | 2^2, 3^1, 29^1  |       True |
|    353 | 2 × 2 × 2 × 2 × 2 × 11    | 2^5, 11^1       |       True |
|    359 | 2 × 179                   | 2^1, 179^1      |       True |
|    367 | 2 × 3 × 61                | 2^1, 3^1, 61^1  |       True |
|    373 | 2 × 2 × 3 × 31            | 2^2, 3^1, 31^1  |       True |
|    379 | 2 × 3 × 3 × 3 × 7         | 2^1, 3^3, 7^1   |       True |
|    383 | 2 × 191                   | 2^1, 191^1      |       True |
|    389 | 2 × 2 × 97                | 2^2, 97^1       |       True |
|    397 | 2 × 2 × 3 × 3 × 11        | 2^2, 3^2, 11^1  |       True |
|    401 | 2 × 2 × 2 × 2 × 5 × 5     | 2^4, 5^2        |      False |
|    409 | 2 × 2 × 2 × 3 × 17        | 2^3, 3^1, 17^1  |      False |
|    419 | 2 × 11 × 19               | 2^1, 11^1, 19^1 |       True |

```python
import math
from collections import Counter

def prime_factors(n):
    factors = []
    while n % 2 == 0:
        factors.append(2)
        n //= 2
    f = 3
    while f * f <= n:
        while n % f == 0:
            factors.append(f)
            n //= f
        f += 2
    if n > 1:
        factors.append(n)
    return factors

def is_prime(num):
    if num < 2:
        return False
    if num in (2, 3):
        return True
    if num % 2 == 0:
        return False
    root = int(math.sqrt(num))
    for i in range(3, root + 1, 2):
        if num % i == 0:
            return False
    return True

def is_hyper_prime(n, memo={}):
    if n in memo:
        return memo[n]
    
    # Bootstrap base cases: 0, 1, 2 are hyper-primes by definition
    if n in (0, 1, 2):
        memo[n] = True
        return True
    
    # Must be prime to be hyper-prime
    if not is_prime(n):
        memo[n] = False
        return False
    
    # Factor n-1
    factors = prime_factors(n - 1)
    factor_counts = Counter(factors)
    
    # Check all prime factors are hyper-primes
    for factor in factor_counts.keys():
        if not is_hyper_prime(factor, memo):
            memo[n] = False
            return False
    
    # Check all exponents are hyper-primes
    for exponent in factor_counts.values():
        if not is_hyper_prime(exponent, memo):
            memo[n] = False
            return False
    
    memo[n] = True
    return True

def generate_primes(limit):
    primes = []
    for p in range(2, limit + 1):
        if is_prime(p):
            primes.append(p)
    return primes

def generate_hyper_primes(limit):
    hyperprimes = []
    memo = {}
    for p in range(2, limit + 1):
        if is_hyper_prime(p, memo):
            hyperprimes.append(p)
    return hyperprimes

def generate_hyper_primes_debug(limit):
    print(f"| {'Prime':>6} | {'Factors of p-1':>25} | {'Exponents':>15} | {'HyperPrime':>10} |")
    print(f"|{'-'*8}|{'-'*27}|{'-'*17}|{'-'*12}|")
    memo = {}
    for p in range(2, limit + 1):
        if not is_prime(p):
            continue
        factors = prime_factors(p - 1)
        counts = Counter(factors)
        exps_str = ', '.join(f"{prime}^{exp}" for prime, exp in counts.items())
        hyper = is_hyper_prime(p, memo)
        factors_str = ' × '.join(map(str, factors)) if factors else '1'
        print(f"| {p:6} | {factors_str:25} | {exps_str:15} | {str(hyper):>10} |")

# Example test up to 420
generate_hyper_primes_debug(420)
```

## References
- <https://en.wikipedia.org/wiki/Super-prime>

#math
