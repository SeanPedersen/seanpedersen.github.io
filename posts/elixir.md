---
title: 'Elixir'
date: '2025-11-14'
---
Elixir is [Erlang](https://seanpedersen.github.io/posts/erlang) with syntax sugar and transpiled to Erlang code.

Parallel function process map:
```elixir
defmodule Utils do
    def pmap(collection, func) do
        collection
        |> Enum.map(fn item -> Task.async(fn -> func.(item) end) end)
        |> Enum.map(fn task -> Task.await(task) end)
    end
end
```

Fibonacci base, memoized and tail recursion:
```elixir
defmodule Fibo do
    # Exponential time complexity: O(2^n)
    # Linear space complexity due to call stack: O(n)
    def fibonacci(0), do: 0
    def fibonacci(1), do: 1
    def fibonacci(n) when n > 1 do
        fibonacci(n - 1) + fibonacci(n - 2)
    end

    # Memoized Fibonacci using caching to avoid redundant calculations
    # Time complexity reduced to O(n)
    # Space complexity O(n) for cache map
    def fibo_cached(n), do: fibo_cached(n, %{}) |> elem(0)

    defp fibo_cached(0, cache), do: {0, cache}
    defp fibo_cached(1, cache), do: {1, cache}

    defp fibo_cached(n, cache) when n > 1 do
        case Map.fetch(cache, n) do
        {:ok, result} ->
            {result, cache}
        :error ->
            {res1, cache1} = fibo_cached(n - 1, cache)
            {res2, cache2} = fibo_cached(n - 2, cache1)
            result = res1 + res2
            {result, Map.put(cache2, n, result)}
        end
    end

    # Tail recursive Fibonacci
    # Time complexity O(n)
    # Constant space complexity O(1)
    def fibo_tail(n), do: fibo_tail(n, 0, 1)
    defp fibo_tail(0, a, _b), do: a
    defp fibo_tail(n, a, b) when n > 0 do
        fibo_tail(n - 1, b, a + b)
    end
end
```

## References
- <https://elixir-lang.org/>

#programming
