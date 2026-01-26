---
date: '2025-09-18'
icon: "/images/icons/erlang.svg"
---
# Erlang

Erlang is a powerful functional programming language invented in Sweden at Ericsson in 1987, that excels at concurrency and fault-tolerance. Erlang processes are light-weight and share  no memory and thus can scale easily across machines and CPU cores. Processes communicate via messages which are received in a queue that is pattern matched against the state loop (actor model). The process supervision tree allows fine-grained error management (let it crash), leading to robust production-ready apps.

Erlang is not made for math heavy computations though but can easily call C or other faster languages for that.

The BEAM (ErlangVM) powers big apps like WhattsApp and Discord, demonstrating its commercial viability.

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

Message passing:
```erlang
% Ping pong across two machines/nodes in the same local network
% pingpong.erl must be compiled in working dir of the erlang shells on both machines!
% ---STARTUP INSTRUCTIONS---
% node1:
% $ erl -compile pingpong
% $ erl -sname pong -setcookie we_have_cookies
% node2:
% $ erl -compile pingpong
% $ erl -sname ping -setcookie we_have_cookies
% pong@node2> pingpong:start(ping@welle).

-module(pingpong).
-export([start/1,  ping/3, pong/0]).

ping(0, PongName, PongNode) ->
    {PongName, PongNode} ! finished,
    io:format("Ping finished~n", []);

ping(N, PongName, PongNode) ->
    {PongName, PongNode} ! {ping, self()},
    receive
        pong ->
            io:format("Ping received pong~n", [])
    end,
    ping(N - 1, PongName, PongNode).

pong() ->
    receive
        finished ->
            io:format("Pong finished~n", []);
        {ping, PingPID} -> % Note that PingPID is enough to reply (PID contains info about its node)
            io:format("Pong received ping~n", []),
            PingPID ! pong,
            pong()
    end.

start(PingNode) ->
    PongName = pong,
    register(PongName, spawn(pingpong, pong, [])),
    % Spawn function ping/3 remotely on PingNode machine
    spawn(PingNode, pingpong, ping, [3, PongName, node()]).
```

## References

- <http://www.erlang.org/>
- <https://learnyousomeerlang.com/content>
- <https://www.erlang-in-anger.com/>
- Erlang project build manager: <http://www.rebar3.org>

#coding
