# Computational Complexity

Computational complexity is the study of how much computational resources an algorithm requires as its input grows.

The two primary resources we measure are:

- **Time complexity:** How the number of operations grows with input size.
- **Space complexity:** How the memory / disk requirements grow with input size.

Rather than measuring execution time in seconds, we use asymptotic notation (Big O) to describe how an algorithm scales.

## Time Complexity

Big O notation describes the upper bound of an algorithm's growth rate.

For example, an algorithm with O(n) complexity grows linearly: doubling the input size approximately doubles the number of operations.

### Common Time Complexities

| Complexity | Name | Algorithm example |
|---|---|---|
| O(1) | Constant | Array index access |
| O(log n) | Logarithmic | Binary search |
| O(n) | Linear | Linear search |
| O(n log n) | Linearithmic | Merge sort, average-case quicksort |
| O(n²) | Quadratic | Insertion sort |
| O(n³) | Cubic | Naive matrix multiplication |
| O(2ⁿ) | Exponential | Brute-force subset enumeration |
| O(n!) | Factorial | Brute-force travelling salesperson |

Note: Quicksort has average-case complexity O(n log n), but worst-case complexity O(n²).

## Big O vs. Real-World Execution Speed

Big O notation describes how an algorithm's computational cost grows as the input size increases. However, it does not describe the actual execution time on a particular machine.

Two algorithms with different Big O complexities can have very different performance characteristics in practice.

### Constant Factors Matter

Consider two algorithms:

- Algorithm A: O(n²), requiring approximately 2n² operations.
- Algorithm B: O(n log n), requiring approximately 500n log₂(n) operations.

Although Algorithm B has better asymptotic complexity, Algorithm A may be significantly faster for small inputs because its individual operations are cheaper and it has a smaller constant factor.

For example:

| Input size (n) | Algorithm A (2n²) | Algorithm B (500n log₂ n) |
|---:|---:|---:|
| 10 | 200 | 16,610 |
| 100 | 20,000 | 332,193 |
| 1,000 | 2,000,000 | 4,982,892 |
| 10,000 | 200,000,000 | 66,438,562 |

In this example, the O(n²) algorithm is faster for smaller inputs, but eventually the O(n log n) algorithm overtakes it.

This crossover point is important when choosing algorithms for real applications.

### Hardware and Implementation Matter

Actual execution time depends on more than the number of abstract operations.

Some important factors include:

- **CPU instructions:** Integer arithmetic, floating-point operations, and division have different costs.
- **Cache locality:** Sequential memory access is often much faster than random memory access.
- **Memory allocation:** Dynamic allocation and garbage collection introduce overhead.
- **Branch prediction:** Conditional statements can cause CPU pipeline stalls when predictions are incorrect.
- **Parallelism:** Some algorithms can distribute work across multiple CPU cores or GPUs more effectively.
- **Compiler optimizations:** Different implementations of the same algorithm can produce substantially different machine code.

For example, an O(n²) algorithm operating on a contiguous array can sometimes outperform an O(n log n) algorithm that repeatedly allocates memory and accesses scattered data.

### Practical Algorithm Selection

This leads to an important distinction:

| Perspective | What matters |
|---|---|
| Theoretical complexity | Asymptotic growth rate |
| Practical performance | Actual execution time |
| Hardware efficiency | CPU, memory, cache and parallelism |
| Implementation quality | Data structures, compiler and optimizations |

Big O is most useful for understanding how algorithms behave as input sizes grow. Benchmarking is necessary to determine which implementation is actually faster on a particular system.

**The best algorithm is not always the one with the best Big O complexity. It is the one that performs best under the constraints of the actual problem.**

## Complexity Classes

Complexity classes group computational problems according to the resources needed to solve them.

Unlike Big O notation, which describes an algorithm's growth rate, complexity classes describe the difficulty of entire problem families.

### Important Complexity Classes

| Class | Description | Example |
|---|---|---|
| P | Problems solvable in polynomial time | Shortest path |
| NP | Problems whose proposed solutions can be verified in polynomial time | Hamiltonian cycle |
| NP-complete | Problems in NP to which every NP problem can be reduced in polynomial time | SAT, 3-SAT |
| NP-hard | Problems at least as hard as every problem in NP | Optimization version of TSP |
| PSPACE | Problems solvable using polynomial space | Generalized geography |

## P vs NP

One of the biggest open questions in computer science is:

**Does P = NP?**

In simpler terms:

If a solution to a problem can be verified efficiently, can that solution also be found efficiently?

### P: Problems That Are Easy to Solve

P contains decision problems that can be solved in polynomial time by a deterministic algorithm.

Examples include:
- Determining whether a path exists between two vertices.
- Determining whether a number is prime.
- Finding whether a graph is connected.

### NP: Problems That Are Easy to Verify

NP contains decision problems for which a proposed solution can be verified in polynomial time.

For example, given a proposed Hamiltonian cycle, we can efficiently verify that it visits every vertex exactly once and returns to its starting point.

However, finding such a cycle may be much harder.

### The Big Question

There are two possibilities:

- **P = NP:** Every efficiently verifiable problem can also be solved efficiently.
- **P ≠ NP:** Some problems are efficiently verifiable but cannot be solved efficiently.

The question remains unresolved.

## Why Does P vs NP Matter?

A proof that P = NP could have enormous implications for optimization, automated reasoning, scheduling, and cryptography.

On the other hand, proving P ≠ NP would establish that some problems are fundamentally harder to solve than to verify.

The P vs NP problem is one of the seven Millennium Prize Problems, with a million-dollar prize for a correct solution.

## Conclusion

Computational complexity gives us a mathematical framework for understanding the scalability and limitations of algorithms.

Big O notation helps us compare algorithm efficiency, while complexity classes help us understand the fundamental difficulty of computational problems.

At the heart of this field remains one of its most fascinating questions:

**Is finding a solution fundamentally harder than verifying one?**

#coding #math
