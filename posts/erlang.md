---
date: '2025-09-18'
icon: "/images/icons/erlang.svg"
---
# Erlang

Erlang is a powerful functional programming language invented in Sweden at Ericsson in 1987, that excels at concurrency and fault-tolerance. Erlang processes are light-weight and share  no memory and thus can scale easily across machines and CPU cores. Processes communicate via messages which are received in a queue that is pattern matched against the state loop (actor model). The process supervision tree allows fine-grained error management (let it crash), leading to robust production-ready apps.

Erlang is not made for math heavy computations though but can easily call C or other faster languages for that.

The BEAM (ErlangVM) powers big apps like WhatsApp and Discord, demonstrating its commercial viability.

[Elixir](/posts/elixir) and [Gleam](https://gleam.run/) are notable modern dialects of Erlang that transpile straight to Erlang. Elixir offers syntactic sugar and a thriving open-source community with popular frameworks like Phoenix. Gleam is a type-safe alternative.

## Show me some code

Quicksort using list comprehension:
```erlang
quicksort([]) -> [];
quicksort([Pivot | T]) ->
    quicksort([X || X <- T, X < Pivot]) ++
    [Pivot] ++
    quicksort([X || X <- T, X >= Pivot]).
```

Fibonacci:
```erlang
-module(fibo).
-author("Sean Pedersen").
-compile(export_all).

% Head recursion (inefficient: builds up exponentially growing function call stack: 2^N)
fib(1) -> 1;
fib(2) -> 1;
fib(N) when N > 1 -> fib(N-1) + fib(N-2).

% Tail recursion (efficient: behaves like a loop)
% The tail recursive optimization lets the function call stack not grow at all)
fib_(N) -> fib(N, 0, 1).
fib(1, _First, Second) -> Second;
fib(N, First, Second) when N > 1 -> fib(N-1, Second, First+Second).
```

Message passing (with functions):
```erlang
-module(uniserver).
-author("Joe Armstrong").

% it just sits and waits for a {become, F} message and then it becomes an F server
universal_server() ->
    receive
       {become, F} ->
           F()
    end.

% waits for an integer and sends back the factorial of an integer
factorial_server() ->
    receive
       {From, N} ->
           From ! factorial(N),
           factorial_server()
    end.


factorial(0) -> 1;
factorial(N) -> N * factorial(N-1).


% creates a universal server, sends it a “become a factorial server” message, then I'll sent it an integer, wait for the response and print the response
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
