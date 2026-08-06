---
date: '2025-09-18'
icon: "/images/icons/erlang.svg"
---
# Erlang

Erlang is a powerful and elegant functional programming language developed in Sweden at Ericsson during the 1980s that excels at distributed, concurrent and fault-tolerant systems. Erlang processes are lightweight and share no memory (mutable state) and thus can scale easily across CPU cores and distributed machines. 

Processes communicate by sending messages to one another. Each process has a mailbox, and receive expressions use pattern matching to select and process messages. Erlang/OTP supervision trees provide structured error management based on the “let it crash” philosophy (supervisor processes restart crashed children), helping developers build robust production applications.

Erlang is not made for math heavy computations but can easily call C or other faster languages for that.

The BEAM (Erlang VM) powers big apps like WhatsApp and Discord, demonstrating its commercial viability.

[Elixir](/posts/elixir) and [Gleam](https://gleam.run/) are notable modern dialects of Erlang that transpile straight to Erlang. Elixir offers syntactic sugar and a thriving open-source community with popular frameworks like Phoenix. Gleam is a type-safe alternative.

## Example Code

### Quicksort
Very concise using list comprehension (filter list elements with condition):
```erlang
quicksort([]) -> [];
quicksort([Pivot | T]) ->
    quicksort([X || X <- T, X < Pivot]) ++
    [Pivot] ++
    quicksort([X || X <- T, X >= Pivot]).
```

## Fibonacci
Head vs tail recursion
```erlang
-module(fibo).
-author("Sean Pedersen").
-compile(export_all).

% Head recursion is inefficient: builds up exponentially growing function calls O(2^N)
fib(1) -> 1;
fib(2) -> 1;
fib(N) when N > 2 -> fib(N-1) + fib(N-2).

% Tail recursion (efficient: behaves like a loop)
% The tail recursive optimization lets the function call stack not grow at all
fib_tail(N) -> fib_tail(N, 0, 1).
fib_tail(1, _First, Second) -> Second;
fib_tail(N, First, Second) when N > 1 -> fib_tail(N-1, Second, First+Second).
```

### Universal Server
Just message passing (with functions)
```erlang
-module(uniserver).
-author("Joe Armstrong").

% mailbox of the universal server waits for a {become, F} message and then it becomes an F server
universal_server() ->
    receive
       {become, F} ->
           F()
    end.

% waits for an integer, sends back the factorial of an integer and waits again
factorial_server() ->
    receive
       {From, N} ->
           From ! factorial(N),
           factorial_server()
    end.


factorial(0) -> 1;
factorial(N) -> N * factorial(N-1).


% creates a universal server, send it a “become a factorial server” message, then send an integer, wait for the response and print the response
test() ->
    Pid = spawn(fun universal_server/0),
    Pid ! {become, fun factorial_server/0},
    Pid ! {self(), 42},
    receive
        X -> X
    end.
```

## References

- https://joearms.github.io/published/2013-11-21-My-favorite-erlang-program.html
- <http://www.erlang.org/>
- <https://learnyousomeerlang.com/content>
- <https://www.erlang-in-anger.com/>
- Erlang project build manager: <http://www.rebar3.org>

#coding
