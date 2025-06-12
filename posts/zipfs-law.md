---
title: "Zipf's Law"
date: '2025-05-10'
---

## Explain the basic idea

Zipf's Law describes a power law distribution that appears across numerous natural phenomena. The concept is elegantly simple: when you rank items by their frequency or size, the relationship follows a predictable pattern.

Here's how it works: if you arrange items in descending order by their frequency, the Nth item will occur approximately 1/N times as often as the most frequent item. Mathematically, this means:

- The 1st ranked item has frequency X₁
- The 2nd ranked item has frequency X₁/2
- The 3rd ranked item has frequency X₁/3
- The Nth ranked item has frequency X₁/N

This creates a smooth, curved distribution when plotted on a graph, revealing an underlying order in what might initially appear random.

## Occurences

### Human Language

The most famous application of Zipf's Law is in linguistics. In English, "the" is the most common word, appearing about 7% of the time. "Of" (the second most common) appears roughly half as often at 3.5%. "And" (third) appears about 2.3% of the time. This pattern continues remarkably consistently across languages.
This discovery led linguists to propose Zipf's Law as a litmus test for determining whether a language is artificial or naturally evolved. Real human languages follow this distribution almost universally, while constructed languages often deviate from it.

### City Populations

Urban demographics also follow Zipf's Law. In the United States, New York City is the largest with about 8.3 million people. Los Angeles (second largest) has roughly 4 million—close to half. Chicago (third) has about 2.7 million. This pattern holds across countries and time periods.

### Passwords

Even in cybersecurity, Zipf's Law emerges. Password frequency distributions follow this pattern, with the most common passwords appearing exponentially more often than less common ones. This has significant implications for security analysis and breach prevention.

### The Deeper Implication

These occurrences hint at an emergent deeper pattern governing self-organizing natural systems. When complex systems evolve without central planning—whether languages, cities, or digital behaviors—they tend to organize themselves according to this mathematical principle.

## Relativation: it occurs also for randomly generated words

"In conclusion, Zipf's law is not a deep law in natural language as one might first have thought. It is very much related the particular representation one chooses, i.e., rank as the independent variable." -> Hinting again at a deeper pattern: the observer is part of the observation - the world is inherently subjective and will always look different depending how you look at it.

TODO: Demonstrate with some python code.

## Closer examination: or does it?

"It is shown that real texts fill the lexical spectrum much more efficiently and regardless of the word length, suggesting that the meaningfulness of Zipf’s law is high." -> Seems Zipf's law is after all not so easily explained...?

## References

- <https://eva.fing.edu.uy/pluginfile.php/211986/mod_resource/content/1/li1992.pdf>
- <https://chance.dartmouth.edu/chance_news/for_chance_news/ChanceNews12.03/RandomZipf.pdf>
- <https://www.youtube.com/watch?v=fCn8zs912OE&pp=ygUQemlwZnMgbGF3IHZzYXVjZQ>

#idea #programming
