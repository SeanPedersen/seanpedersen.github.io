---
title: 'Popping the AGI Bubble'  
date: '2025-07-26'
---
It is becoming clearer and clearer that scaling up LLM's will not solve their fundamental limits like hallucinations (making up bullshit) and context rot (decreasing performance with growing noisy input context). While LLM companies are still slashing benchmarks (often by training on test data directly or guessing the test data) - users of LLM's know the truth: LLM's are a useful tool but nowhere near a robust AGI (that can reliably solve complex tasks without human supervision) and there is nothing in sight suggesting this will change soon.

## Fundamental Limits

### Auto-Regressive Architectures

Current LLM's face a core architectural constraint: they generate text sequentially, one token at a time. As Yann LeCun argues, this creates exponential error accumulation. Each prediction depends on all previous tokens, so early mistakes cascade through long sequences, causing models to derail from coherent long-formed reasoning (an issue well known among LLM users - some call it context rot).

This sequential generation prevents LLM's from forming thoughts holistically. Unlike human cognition, which manipulates abstract concepts as complete structures, current models must linearize everything into word sequences. They cannot think about problems in abstract space before committing to specific text.

The result is a fundamental bottleneck. Tasks requiring sustained logic or complex reasoning remain difficult regardless of scale improvements (making autonomous agents doing long complex tasks a real challenge). Simply adding more parameters or training data cannot solve this architectural limitation. Transformer based LLM's will thus never overcome the hallucination problem for long-context tasks (known as context rot).

True progress requires a paradigm shift toward architectures that form and manipulate complete conceptual structures rather than sequential tokens. Until then, LLM's remain sophisticated stochastic pattern matchers, staying far away from reliable reasoning.

### Model Collapse

LLM's trained on their own outputs suffer from model collapse - their performance degrades. Thus the trend of more and more LLM generated content (slop) being published on the web, will degrade their performance in the long run.

### Fractured Embeddings

The knowledge representation (weight matrices) in neural networks is fractured / entangled leading to issues like adversarial examples and hallucinations.

## Problem Classes

Current LLM can solve low complexity tasks with lots of training data available (text, code, video, audio) and high-complexity tasks which are easy to verify (so big training data with high-signal can be generated) like chess, programming and math - by interpolating on existing data. It is easy to verify who won a chess or go game and thus possible to generate high-signal training data. The same is true for certain classes of programming problems (does it compile? does it run without errors? does it pass tests? does it produce same output as an existing program? does it better on benchmark X?).

It is much harder to generate meaningful rewards for more abstract programming tasks though (is the UI design polished? is the user experience sensible?) - this data can only be inferred from expensive human usage.

So we have two types of problems: easy to verify problems and thus also easy to produce big high-signal training data and hard to verify problems (needs humans in the loop) and thus hard to produce big high-signal training data. For the former we are in Alpha Zero territory (innovative super-intelligence using RL) and the latter we are in Alpha Go territory (interpolate and synthesize on human generated data).

### Real Value

LLM + RL solveable examples:
- discover novel algorithms for things that can be easily verified / benchmarked
  - faster matrix-multiplication (<https://www.nature.com/articles/s41586-022-05172-4>)

LLM solveable examples:
- coding in a popular programming language on medium sized code base
- summarizing complicated technical topics (with lots of training data available)
- generating (interpolating) texts, audio and video

And anything which has tolerance for errors (hallucinations).

### Tasks out of Reach

High complexity (very long context / completely novel) tasks with minimal training data available (often limited by needing humans to produce data).

Any critical task that has no tolerance for high error rates.

LLM (+ RL) unsolveable examples:
- creating completely new (not merely interpolating) mathematical theorems and proofs in unexplored domains
- coming up with novel funny jokes

## Signals for AGI

- no benchmark (problem) we can design where humans beat AI
  - coming up with novel funny jokes
- extreme and robust (human like) generalisation ability
  - LLM's possess clearly no human like intelligence as they make obvious logical and factual mistakes (very brittle; context dependent problem solving ability) and hallucinate without being capable to self-correct through self-inspection -> LLM's output will always have to be validated using human supervision for critical tasks (just like for self-driving cars)
- self-awareness (introspection) and ability to self-modify (update facts / beliefs)

## References

- <https://wonderfall.dev/autoregressive/>
- Collapse of Self-trained Language Models - <https://arxiv.org/abs/2404.02305v1>
- The wall confronting large language models - <https://arxiv.org/abs/2507.19703>
- Cats Confuse Reasoning LLM (study on context rot phenomenon) - <https://arxiv.org/abs/2503.01781>
- <https://blog.jxmo.io/p/superintelligence-from-first-principles>
- <https://blog.jxmo.io/p/we-should-stop-talking-about-agi>
- <https://garymarcus.substack.com/p/dear-elon-musk-here-are-five-things>
- This video will change your mind about the AI hype - <https://www.youtube.com/watch?v=uB9yZenVLzg>
- Questioning Representational Optimism in Deep Learning: The Fractured Entangled Representation Hypothesis - <https://arxiv.org/abs/2505.11581>

#AI
