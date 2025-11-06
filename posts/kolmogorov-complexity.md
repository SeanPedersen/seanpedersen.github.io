---
title: 'Kolmogorov Complexity'
date: '2021-05-04'
---
## Introduction

This text is devoted to my favorite CS concept I know of (so far): Kolmogorov complexity. It is right up there with graphs, trees and recursion. Its beauty stems from its simple definition and powerful connections to many research fields I fancy. And it is defined as follows: The length of the smallest program producing a given sequence of bits.

This might sound rather unimpressive to the uninitiated reader but hold on, behind this simple concept lies a whole world of deep implications and applications.

Knowing the Kolmogorov complexity of a sequence means knowing every algorithmic pattern there can be known about it. It is the process of removing all computable redundancies existing in the data, arriving at the shortest algorithm giving rise to these patterns using computation (space, time and energy).

So how do we compute it?

Sadly it is incomputable in praxis. What a bummer.

Computing the Kolmogorov complexity for an arbitrary sequence X of N bits length is a really tough nut to crack, since the search space of possible programs producing X is growing exponentially with respect to the input size N. And the real deal breaker: it entails the notorious halting problem, which is incomputable. The most obvious upper bound for K(X) is obviously the length N of X itself.

We can still implement a program that generates 2^N different programs and filter out all invalid programs for the chosen programming language. We are left with an enormous list of valid programs with a length below N.

Sadly we can not just execute this at least finite amount of programs, filter their outputs for the given sequence and be done by sorting the valid leftover programs by length. Some programs will take ages to terminate and some may even never - we could analyze this finite set of programs using formal verification techniques to determine if each programs halts or not but this endeavor would be impractical since the set grows exponentially.

(Previously I wrote: "and we can not tell them apart, as Alan Turing proved with his infamous halting problem" -> which is wrong! Turing proved that there exists no algorithm that can determine if an **arbitrary** program halts or not. While the halting problem is undecidable in general, for a small, fixed set of programs, we can manually prove (or disprove) halting using mathematical reasoning or formal methods. However, this requires a separate proof per program and the number of programs up to length n grows as ~2^N - making it impractical beyond toy sizes (N ≲ 15). For real approximation, we must use timeouts.)

## Kolmogorov complexity extensions

Kolmogorov complexity alone is simple and beautiful but as we learned it is not practical to compute. So let us try extending it by some practical properties of computer programs like their total execution time and maximum allocated space (e.g. RAM). This yields closely related complexity measures that should be easier to approximate and thus be more relevant for practical applications since we finally can compute them.

Notation:

- **x**: Input sequence
- **L(x)**: Length of x
- **P(x)**: Set of all programs computing x
- **p**: Element of P(x) (a single program)
- **L(p)**: Length of p
- **T(p)**: Total execution time of p
- **S(p)**: Maximum size of allocated space used by p

Complexity Measures:

- ```K(x) = min(L(p) for every p of P(x))```
- ```KT(x) = min(L(p) * T(p) for every p of P(x))```
- ```KTS(x) = min(L(p) * T(p) * S(p) for every p of P(x))```

K(x) is the normal Kolmogorov complexity function. KT(x) is an extension that weights the programs execution time into the measure. KTS(x) also considers the space requirements of the program.

Ok what now? Let’s do what computer scientists like to do and cheat our way out using the power of good enough heuristics / approximations. One easy way to deal with programs that run too long or even infinitely is to discard them after exactly 42 seconds.

This seemingly dirty trick is actually quite useful, it filters out programs that run too long. By doing so we lose the guarantee to find the true Kolmogorov complexity, since a really short program might execute very long until it finally outputs X (the input sequence). Sidenote: There is a paper and it also makes intuitive sense that the source length of a program correlates positively with its execution time which helps us a lot. So we lose the guarantee to find true K(X) but are still left with short (not necessarily shortest anymore!) programs that execute in our desired maximum time of 42 seconds.

Another nice fact is that lazily searching for the shortest programs is still optimal (within our approximating limits) since we are done when we find the first (thus shortest) program producing the input sequence X within our time limit, unless we want to continue searching for a potentially faster program (although it becomes less likely to find faster executing programs as their size grows).

So have I just described a computable version of K(X)? No but a practical approximation. We search for the shortest program K(X, T=<42) that produces X within a predefined time / computational resource limit. Start with the shortest program and complexify it iteratively spanning a tree of all possible programs. Programs that do not halt or simply execute too long are simply discarded which is nice since we want efficient programs anyway.

## Computing data complexities in the wild

Common lossless data compression tools like ZIP are only approximations of the true Kolmogorov complexity for their inputs. The length of the compressed data plus the length of the program decompressing the data is an upper bound of the true Kolmogorov complexity.

“Kolmogorov complexity is incomputable: there exists no algorithm that, when input an arbitrary sequence of data, outputs the shortest program that produces the data.” - Wikipedia

## Applications of Kolmogorov complexity approximations

Compression Distance (CD) takes two objects A,B (e.g. text files) as arguments and defines a similarity score by using kolmogorov complexity or in praxis an approximation (e.g. compression algorithms) c(X) on A, B and both concatenated A+B. CD is then defined as:
```CD(A,B) = len(c(A+B)) / ( len(c(A)) + len(c(B)) )```

If A & B are very similar c(A+B) will not be much larger than either c(A) or c(B) producing a small CD(A,B), vice versa if A & B are dissimilar c(A+B) will be much larger than c(A) or c(B) producing a big CD(A,B).

TODO:

- [Solomonoff Induction](https://www.lesswrong.com/tag/solomonoff-induction) and [AIXI](https://www.lesswrong.com/tag/aixi): Kolmogorov Complexity as Occam's razor for AGI
- [Hutter Prize](http://prize.hutter1.net/): Compression (prediction) as key ingredient to AGI

#programming
