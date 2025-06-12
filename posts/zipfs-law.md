---
title: "Zipf's Law"
date: '2025-05-10'
---

## Basic idea

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

```python
import random
import matplotlib.pyplot as plt
from collections import Counter
import numpy as np
from scipy import stats

def generate_li_uniform_text(num_chars=100000, alphabet=['a', 'b', 'c', 'd', 'e', '_']):
    """Generate random text with uniform character probabilities"""
    text = ''.join(random.choices(alphabet, k=num_chars))
    words = [word for word in text.split('_') if word]
    return words

def generate_li_biased_text(num_chars=100000):
    """Generate random text with biased character probabilities"""
    alphabet = ['a', 'b', 'c', 'd', 'e', '_']
    probabilities = [0.3, 0.2, 0.15, 0.1, 0.05, 0.2]  # biased weights
    
    text = ''.join(random.choices(alphabet, weights=probabilities, k=num_chars))
    words = [word for word in text.split('_') if word]
    return words

def calculate_zipf_slope(ranks, frequencies):
    """Calculate Zipf slope using linear regression on log-log scale"""
    log_ranks = np.log10(ranks)
    log_freqs = np.log10(frequencies)
    
    # Remove infinite values
    valid = np.isfinite(log_ranks) & np.isfinite(log_freqs)
    if np.sum(valid) < 2:
        return None, None
    
    slope, _, r_value, _, _ = stats.linregress(log_ranks[valid], log_freqs[valid])
    return slope, r_value**2

def plot_li_experiments():
    """Generate and plot both Li experiments"""
    print("Generating Li's experiments...")
    
    # Generate data
    uniform_words = generate_li_uniform_text()
    biased_words = generate_li_biased_text()
    
    print(f"Generated {len(uniform_words)} uniform words, {len(biased_words)} biased words")
    
    # Get frequency distributions
    uniform_counts = Counter(uniform_words).most_common(200)
    biased_counts = Counter(biased_words).most_common(200)
    
    # Prepare data for plotting
    uniform_ranks = np.array(range(1, len(uniform_counts) + 1))
    uniform_freqs = np.array([count for _, count in uniform_counts])
    
    biased_ranks = np.array(range(1, len(biased_counts) + 1))
    biased_freqs = np.array([count for _, count in biased_counts])
    
    # Calculate slopes
    uniform_slope, uniform_r2 = calculate_zipf_slope(uniform_ranks, uniform_freqs)
    biased_slope, biased_r2 = calculate_zipf_slope(biased_ranks, biased_freqs)
    
    # Create plots
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(14, 6))
    
    # Uniform case
    ax1.loglog(uniform_ranks, uniform_freqs, 'go-', alpha=0.7, markersize=4, 
               label='Uniform probabilities')
    
    if uniform_slope:
        # Add fitted line
        fitted_line = uniform_freqs[0] * (uniform_ranks ** uniform_slope)
        ax1.loglog(uniform_ranks, fitted_line, 'r--', alpha=0.8, 
                   label=f'Slope: {uniform_slope:.3f} (R²={uniform_r2:.3f})')
    
    ax1.set_xlabel('Rank')
    ax1.set_ylabel('Frequency')
    ax1.set_title('Uniform Character Probabilities\n(Shows Step Pattern)')
    ax1.grid(True, alpha=0.3)
    ax1.legend()
    
    # Biased case
    ax2.loglog(biased_ranks, biased_freqs, 'bo-', alpha=0.7, markersize=4, 
               label='Biased probabilities')
    
    if biased_slope:
        # Add fitted line
        fitted_line = biased_freqs[0] * (biased_ranks ** biased_slope)
        ax2.loglog(biased_ranks, fitted_line, 'r--', alpha=0.8, 
                   label=f'Slope: {biased_slope:.3f} (R²={biased_r2:.3f})')
    
    ax2.set_xlabel('Rank')
    ax2.set_ylabel('Frequency')
    ax2.set_title('Biased Character Probabilities\n(Smooth Zipf Curve)')
    ax2.grid(True, alpha=0.3)
    ax2.legend()
    
    plt.tight_layout()
    plt.show()
    
    # Print results
    print(f"\nResults:")
    print(f"Uniform case: slope = {uniform_slope:.4f}, R² = {uniform_r2:.4f}")
    print(f"Biased case:  slope = {biased_slope:.4f}, R² = {biased_r2:.4f}")
    print(f"\nKey finding: Uniform probabilities create 'steps' in the distribution,")
    print(f"while biased probabilities create a smooth Zipf-like curve.")

# Run the simplified experiment
plot_li_experiments()
```

![random-words-zipfs-law](/images/random-words-zipfs-law.png)

## Closer examination: or does it?

"It is shown that real texts fill the lexical spectrum much more efficiently and regardless of the word length, suggesting that the meaningfulness of Zipf’s law is high." -> Seems Zipf's law is after all not so easily explained...?

## References

- Li, W. (1992). [Random Texts Exhibit Zipfs-Law-Like Word Frequency Distribution](https://eva.fing.edu.uy/pluginfile.php/211986/mod_resource/content/1/li1992.pdf). IEEE Transactions on Information Theory.
- RAMON FERRER i CANCHO and RICARD V. SOLE. [Zipf's Law and Random Texts](https://chance.dartmouth.edu/chance_news/for_chance_news/ChanceNews12.03/RandomZipf.pdf). Advances in Complex Systems.
- Vsauce. [The Zipf Mystery](https://www.youtube.com/watch?v=fCn8zs912OE). YouTube.
- Reddit Discussion. [Random texts exhibit Zipf's-law-like word frequency distribution](https://www.reddit.com/r/voynich/comments/ehrbvm/random_texts_exhibit_zipfslawlike_word_frequency/). r/voynich.

#idea #programming
