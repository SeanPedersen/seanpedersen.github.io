---
title: 'The Future of Programming'
date: '2020-12-29'
---
I want to briefly introduce you into what I believe to be the future of programming.

The current state of programming is in my opinion best described as a big rapidly evolving cluster fuck of programming languages and frameworks. What is needed is a way to reduce the cluster fuck without compromising on the ongoing forces of innovation.

Content addressable code is the first concept I want to highlight. It is awesome and long overdue in language design. It works by identifying code module versions not by name + version number or another URL but instead uses a hash of the code it self (potentially using a Merkle Tree). This brings plenty of advantages: no more build and dependency conflicts, easy code version caching and allows to verify code authenticity on your machine (just compute the hash).

So why stop with content addressable code? I not only want to know if two functions are syntactically equivalent (content based) but I also want to know if they share the same behavior. Two functions share the same behavior if and only if they map the same inputs to the same outputs. In praxis it is often not feasible to test that for all possible inputs and outputs due to time constraints we mere human mortals have. But we can still get use out of this behavior addressable concept by using predefined test cases that capture essential and edge cases as well as some automated fuzzing of possible inputs. When behavior based addressing is working, the next and final level is adding automated run time and space usage analysis for equivalently behaving functions to the landscape of programming language infrastructure. This will allow developers to find quickly the most efficient code available to mankind for their desired code behavior (function). This is the future of programming and will greatly accelerate technological progress of humanity in all areas involving software.

## A practical Example

Sorting a list of integers is a canonical function that probably has been implemented millions of times in thousands of programming languages.

So how would I go about finding the best implementation in existing language eco systems? First I would use the built in sorting implementation and hope the language designers put some work in to make it fast. But I am unsure about that, so I start benchmarking the code. I have a feeling there is room to improve. I start researching alternative implementations and run further my own benchmarks. Finally I find an implementation that consistently is faster for my use cases. I add the code as a dependency and if I have a great day, I might even write some unit tests to make sure it will keep doing what it is supposed to do, even when upgrading the code in the future. Cool that works and is probably done by many folks repeatedly in the same eco system (wasting humanities time).

But there are many points we can do better using behavior addressable code modules. Think about it. We need one individual that feels our new shiny language eco system has the need for a function that sorts a list of integers. That individual needs to sit down and define an interface describing the input and output domains of the function. For a simple sorting function, it would look something like: sort(List\<Integer\>) -> List\<Integer\>. Pretty simple stuff. Next the individual would define some hard test cases that verify the desired behavior of the function: assert(sort([2,3,1] == [1,2,3]), and so on. Now that the function interface and behavior via test cases is nailed down, anyone can start implementing and submitting functions to it. On submission of a new function, the following happens: First the function is verified to implement the specified interface, second its behavior is verified using the existing test cases, third its code is hashed to make it content addressable and finally its run time and memory profile is measured. And all of that happens automatically.

Now imagine yourself again needing the best available sorting function for your project. What changed? A whole lot. Now you can search an existing code module index for the function you need (sorting integers). Then you will be presented with all of the existing functions implementing your desired behavior, verified for correctness and benchmarked for run time and memory usage. You will quickly find the sweet spot implementation that ticks off all your needs (fastest / lowest memory usage peak) and import it using its content addressable hash. Life can be so easy after all.

I would love to see this being worked on and challenge any reader to build a prototype and share it with the world. Existing language eco systems that would benefit greatly in my opinion (in no particular order) are:

- Python (PyPI)
- Javascript (npm)
- Rust (cargo)
- Elixir (hex)

Update 2024: There seems to be a content addressable language on the block (<https://scrapscript.org/>)

References:

- Content addressable code (<https://www.unisonweb.org/docs/tour>)

#programming
