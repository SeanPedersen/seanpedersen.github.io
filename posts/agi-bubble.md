---
title: 'Popping the AGI Bubble'  
date: '2025-07-26'
---
It is becoming clearer and clearer that scaling up LLM's will not solve their fundamental limits like hallucinations (making up bullshit) and context rot (decreasing performance with growing noisy input context). While LLM companies are still slashing benchmarks - users of LLM's know the truth: LLM's are a useful tool but nowhere near a robust AGI and there is nothing in sight suggesting this will change soon.

## Fundamental Limits

### Auto-Regressive Architectures

Current LLM's face a core architectural constraint: they generate text sequentially, one token at a time. As Yann LeCun argues, this creates exponential error accumulation. Each prediction depends on all previous tokens, so early mistakes cascade through long sequences, causing models to derail from coherent long-formed reasoning (an issue well known among LLM users - some call it context rot).

This sequential generation prevents LLM's from forming thoughts holistically. Unlike human cognition, which manipulates abstract concepts as complete structures, models must linearize everything into word sequences. They cannot think about problems in abstract space before committing to specific text.

The result is a fundamental bottleneck. Tasks requiring sustained logic or complex reasoning remain difficult regardless of scale improvements (making autonomous agents doing long complex tasks a real challenge). Simply adding more parameters or training data cannot solve this architectural limitation. Transformer based LLM's will thus never overcome the hallucination problem for long-context tasks (known as context rot).

True progress requires a paradigm shift toward architectures that form and manipulate complete conceptual structures rather than sequential tokens. Until then, LLM's remain sophisticated stochastic pattern matchers, staying far away from reliable reasoning.

### Fractured Embeddings

The knowledge representation (weight matrices) in neural networks is fractured / entangled leading to issues like adversarial examples and hallucinations.

## Real Value

Low complexity tasks with lots of training data available (text, video, audio) that can be solved by interpolating on existing data and high-complexity tasks which are easy to verify (so big training data with high-signal can be generated) like chess, programming and math.

- coding in a popular programming language
- summarizing complicated technical topics (with lots of training data available)
- generating (interpolating) texts, audio and video

## Tasks out of Reach

High complexity (very long context / completely novel) tasks with minimal training data available.

## Signals for AGI

- no benchmark (problem) we can design where humans beat AI
  - coming up with novel funny jokes
- extreme and robust (human like) generalisation ability
  - LLM's possess clearly no human like intelligence as they make obvious logical and factual mistakes and hallucinate without being capable to self-correct through self-inspection -> LLM's output will always have to be validated for critical tasks
- self-awareness (introspection) and ability to self-modify (update facts / beliefs)

## References

- <https://wonderfall.dev/autoregressive/>
- <https://blog.jxmo.io/p/superintelligence-from-first-principles>
- <https://blog.jxmo.io/p/we-should-stop-talking-about-agi>
- <https://garymarcus.substack.com/p/dear-elon-musk-here-are-five-things>
- This video will change your mind about the AI hype
 - <https://www.youtube.com/watch?v=uB9yZenVLzg>
- Questioning Representational Optimism in Deep Learning: The Fractured Entangled Representation Hypothesis - <https://arxiv.org/abs/2505.11581>

#AI
