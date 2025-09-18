---
title: 'Functional Programming'
date: '2025-06-26'
---
A programming paradigm gaining traction with good reasons. The selling point of functional programming are easy to reason about code through simple chains of data transformations.

## Pure Functions vs Dirty Methods

The core difference between functional and other programming paradigms lies in how we handle state and behavior. Pure functions are predictable. Give them the same input, and they always return the same output. No surprises, no hidden state changes, no mysterious side effects.

```python
# Pure function
def add(a, b):
    return a + b

# Dirty method using an object
class Counter:
    def __init__(self):
        self.count = 0
    
    def increment(self):
        self.count += 1
        return self.count
```

The `add` function is pure. Call it with 2 and 3, and you get 5 every time. The `increment` method on the Counter object is dirty because it modifies the object's internal state (this is a toy example but in a complex code base, dirty methods mutating left and right cause many headaches). Its behavior depends on how many times you've called it before.

## Why Pure Functions Matter

Pure functions make reasoning about your programs and  debugging easier. When something breaks, you know the problem is either in the inputs or the function itself. You don't have to trace through a web of state changes across your entire application (like you may be used to from object-oriented programming).

Testing becomes straightforward too. No need to set up complex scenarios or mock external dependencies. Just pass in some data and verify the output.

Pure functions also are easy to cache. Since they always return the same output for the same input, you can store results and reuse them later. This technique, called memoization, can dramatically improve performance for expensive computations.

```python
from functools import lru_cache

@lru_cache(maxsize=128)
def fibonacci(n):
    if n < 2:
        return n
    return fibonacci(n-1) + fibonacci(n-2)
```

Without caching, calculating fibonacci(40) would require millions of recursive calls. With memoization, the function remembers previous results and runs much faster.

## Immutability as a Foundation

Functional programming treats data as immutable. Instead of changing existing data, you create new data. This might sound wasteful, but it prevents many common bugs. When functions don't mutate their inputs, they stay pure and predictable (just inputs mapped to outputs).

```python
# Imperative approach
numbers = [1, 2, 3]
numbers.append(4)  # Mutates original list

# Functional approach
numbers = [1, 2, 3]
new_numbers = numbers + [4]  # Creates new list
```

When data cannot change unexpectedly, your code becomes more predictable. Multiple parts of your program can safely reference the same data without worrying about modifications.

## Higher Order Functions

Functions in functional programming are first class citizens. You can pass them around like any other value. This enables powerful patterns like map, filter, and reduce.

```python
numbers = [1, 2, 3, 4, 5]
doubled = list(map(lambda x: x * 2, numbers))
evens = list(filter(lambda x: x % 2 == 0, numbers))
total = sum(numbers)
```

These operations create new data rather than modifying existing data. Each step in the chain is independent and testable.

## Practical Benefits

Functional programming shines in scenarios where data flows through multiple transformations. APIs that process requests, data pipelines that clean and analyze information, and user interfaces that respond to events all benefit from functional approaches.

The paradigm also scales well. Pure functions can run in parallel without coordination because they don't share mutable state. This makes functional programs naturally suited for concurrent execution.

## Getting Started

You don't need to abandon your current programming language to try functional programming. Most modern languages support functional features. Start by writing pure functions where possible. Use map, filter, and reduce instead of loops. Avoid mutating data.

The transition takes practice, but the payoff is code that's easier to understand, test, and maintain. Your future self will thank you for the clarity. I can recommend [Erlang](https://seanpedersen.github.io/posts/erlang) and Elixir as functional programming languages (many prefer Elixir syntax).

## Conclusion

Functional programming offers a different way to think about code. By focusing on pure functions and immutable data, you can write programs that are more predictable and easier to reason about. The paradigm won't solve every problem, but it provides valuable tools for managing complexity in software development.

#programming
