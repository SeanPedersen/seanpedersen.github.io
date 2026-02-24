---
date: '2024-07-18'
---
# Overcoming the limits of current LLM


Large language models (LLM) have been all the rage for quite some time now. Looking beyond the hype though, they have severe limitations: hallucinations, lack of confidence estimates and lack of citations.

Hallucination refers to the phenomenon where LLM generates content that sounds convincing / factual but is actually ungrounded or plain wrong.

A confidence estimate assigns a confidence number to a prediction and is useful to estimate factuality. Though a high confidence score for a wrong answer is worse than no confidence score to begin with… So that may be the reason I have not seen any in commercial products. Though OpenAI released this: <https://openai.com/index/teaching-models-to-express-their-uncertainty-in-words/>

Citations are just sources on which a text is based. This can be achieved via so-called RAG techniques (just search over a text corpus and hope to find relevant documents, which are added to the query and cited). Good examples are <https://perplexity.ai> and <https://github.com/stanford-oval/WikiChat>. RAG can be useful to detect if a prompt is in or out of the language models training data distribution, possibly useful to reduce hallucinations as demonstrated in WikiChat by Stanford.

A better LLM chatbot would address ideally all three of these limitations. So what are possible pathways to achieve them?

Hallucinations are certainly the toughest nut to crack and their negative impact is basically only slightly lessened by good confidence estimates and reliable citations (sources).

The impact of contradictions in the training data: Current language models are incapable of "self-inspection" to uncover logical inconsistencies in their training data. Only in the input context window should they be able to find logical inconsistencies.

## Bootstrapping consistent LLM

TL;DR: Exclude contradicting training data -> curate the training data using LLM's.

A LLM that selects its own training data to bootstrap itself: Curate a small dataset that is coherent, logical and truthful as good as possible, train the base model. Then use this base model to select or reject more training data to train on. This may alleviate inconsistencies. Researchers at MIT actually did something very similar: <https://news.mit.edu/2023/large-language-models-are-biased-can-logic-help-save-them-0303> (paper: <https://arxiv.org/abs/2303.05670>)

The consistency bootstrapping could work something like this: manually curate a high-quality (consistent) text corpus based on undisputed, well curated wikipedia articles and battle tested scientific literature. Train a base LLM on this curated training data. Use this base model to classify new text documents as consistent or inconsistent by using RAG on the curated training corpus. Gradually extend the training data with consistent text documents and finally train a big consistent LLM.

One could spin this idea even further and train several models with radically different world views by curating different training corpi that represent different sets of beliefs / world views.

I hope to see more research exploring this consistent data bootstrapping approach for LLM.

## Other Hallucination Mitigation Strategies

**Entropy Based**: Analyze the entropy (variance) of the logits. High-variance should indicate uncertainty. <https://github.com/xjdr-alt/entropix>

**Semantic Entropy Based**: Sample multiple answers (high temperature) and estimate confidence based on the variance of the resulting answers embedding vectors.

-> These approaches have a major flaw though (as pointed out by this HN commenter):

> ..., there is a real problem with this approach that lies at the mathematical level.
> 
> For any given input text, there is a corresponding output text distribution (e.g. the probabilities of all words in a sequence which the model draws samples from).
> 
> The approach of drawing several samples and evaluating the entropy and/or disagreement between those draws is that it relies on already knowing the properties of the output distribution. It may be legitimate that one distribution is much more uniformly random than another, which has high certainty. Its not clear to me that they have demonstrated the underlying assumption.
> 
> Take for example celebrity info, "What is Tom Cruise known for?". The phrases "movie star", "katie holmes", "topgun", and "scientology" are all quite different in terms of their location in the word vector space, and would result in low semantic similarity, but are all accurate outputs.
> 
> On the other hand, "What is Taylor Swift known for?" the answers "standup comedy", "comedian", and "comedy actress" are semantically similar but represent hallucinations. Without knowing the distribution characteristics (e.g multivariate moments and estimates) we couldn't say for certain these are correct merely by their proximity in vector space.
> 
> As some have pointed out in this thread, knowing the correct distribution of word sequences for a given input sequence is the very job the LLM is solving, so there is no way of evaluating the output distribution to determine its correctness.
> 
> There are actual statistical models to evaluate the amount of uncertainty in output from ANNs (albeit a bit limited), but they are probably not feasible at the scale of LLMs. Perhaps a layer or two could be used to create a partial estimate of uncertainty (e.g. final 2 layers), but this would be a severe truncation of overall network uncertainty.
> 
> Another reason I mention this is most hallucinations I encounter are very plausible and often close to the right thing (swapping a variable name, confabulating a config key), which appear very convincing and "in sample", but are actually incorrect.

Quoted from HN user program_whiz - <https://news.ycombinator.com/item?id=40769496>

## Self-Improving LLM via RL

A promising "self-reflection" RL method for LLM's trained in verifiable domains (programming, math, etc.):

> We explore a method for improving the performance of large language models through self-reflection and reinforcement learning. By incentivizing the model to generate better self-reflections when it answers incorrectly, we demonstrate that a model's ability to solve complex, verifiable tasks can be enhanced even when generating synthetic data is infeasible and only binary feedback is available. Our framework operates in two stages: first, upon failing a given task, the model generates a self-reflective commentary analyzing its previous attempt; second, the model is given another attempt at the task with the self-reflection in context. If the subsequent attempt succeeds, the tokens generated during the self-reflection phase are rewarded. Our experimental results show substantial performance gains across a variety of model architectures, as high as 34.7% improvement at math equation writing and 18.1% improvement at function calling. Notably, smaller fine-tuned models (1.5 billion to 7 billion parameters) outperform models in the same family that are 10 times larger. Our novel paradigm is thus an exciting pathway to more useful and reliable language models that can self-improve on challenging tasks with limited external feedback.

- Reflect, Retry, Reward: Self-Improving LLMs via Reinforcement Learning - <https://arxiv.org/abs/2505.24726>

## LLM with Code World Model

> To improve code understanding beyond what can be learned from training on static code alone, we mid-train CWM on a large amount of observation-action trajectories from Python interpreter and agentic Docker environments, and perform extensive multi- task reasoning RL in verifiable coding, math, and multi-turn software engineering environments.

- [CWM: An Open-Weights LLM for Research on Code Generation with World Models](https://ai.meta.com/research/publications/cwm-an-open-weights-llm-for-research-on-code-generation-with-world-models/)

"Instead of doing code training by just predicting the next token in the source file, interleave that with interpreter state which also have to be predicted!" - [Lucas Beyer on X](https://x.com/giffmana/status/1971507878025445653)



## Importance of the Context Window

> In "The Reversal Curse", researchers at Vanderbilt, UK AISI, Apollo, NYU, Sussex, and Oxford discovered the following occurrences in a datapoint:
> 
> - George Washington was the First President of the United States
> - Kim Kardashian is the daughter of Kris Jenner
> - Donald Trump is the current President of the United States
> 
> If you took these sentences, and you DELETED any occurrance from the training data... where the sentence was reversed... e.g.
> - The First President of the United States was George Washington"
> - Kris Jenner is the mother of Kim Kardashian
> - The current president of the United States is Donald Trump
> 
> This created a problem for an LLM... because it's only sees the token frequencies in one direction (left to right).
> 
> This meant that an LLM can CORRECTLY answer prompts like: 
> - Who is George Washington? 
> - Who is Kim Kardashian the daughter of? 
> - What is Donald Trump's current occupation?
> 
> But the LLM could NOT answer prompts like: 
> - Who was the first president of the United States? 
> - Who is the daughter of Kris Jenner? 
> - Who is the current president of the United States?
> 
> It's a pretty crazy limitation... and its very telling about how LLMs work...
> 
> But things get really crazy when you change one more thing. If you first ask an LLM the first prompt (the one that works) and then keep that in the context window... it CAN correctly answer the second prompt!!
> 
> This describes where and how LLMs do logic.
> 
> - Training data -> Prediction: NO LOGIC (can't even reverse a simple relation) 
> - Context Window -> Prediction: LOGIC (can do complex reasoning)

- [Andrew Trask on X](https://x.com/iamtrask/status/1965522412243677522)

This phenomenon highlights why chain of thought (CoT) and RAG / context engineering are so successful: LLM can do much more complex logical inferences from its context window than one-shotting it directly from its weights (compressed training data).

## Fundamental Limits

Current LLMs face a core architectural constraint: they generate text sequentially, one token at a time. As Yann LeCun argues, this creates exponential error accumulation. Each prediction depends on all previous tokens, so early mistakes cascade through long sequences, causing models to derail from coherent long-formed reasoning (an issue well known among LLM users - some call it context rot).

This sequential generation prevents LLMs from forming thoughts holistically. Unlike human cognition, which manipulates abstract concepts as complete structures, models must linearize everything into word sequences. They cannot think about problems in abstract space before committing to specific text.

The result is a fundamental bottleneck. Tasks requiring sustained logic or complex reasoning remain difficult regardless of scale improvements (making autonomous agents doing long complex tasks a real challenge). Simply adding more parameters or training data cannot solve this architectural limitation. Transformer based LLM's will thus never overcome the hallucination problem for long-context tasks (known as context rot).

True progress requires a paradigm shift toward architectures that form and manipulate complete conceptual structures rather than sequential tokens. Until then, LLMs remain sophisticated stochastic pattern matchers, staying far away from reliable reasoning.

Geoffrey Hinton speaks of confabulations not hallucinations. He says LLMs are very human like in that aspect, as humans also constantly recall things inaccurately. I disagree: LLMs confabulate things a human expert never would. After delivering an accurate logical argument, they make up the dumbest shit - this is not human like at all.

## Outlook

The endgame would be language models that actively create and verify new knowledge using it to update their world-model / weights on (capable of self-modification).

## References

- Great talk on current limits of LLM: <https://www.youtube.com/watch?v=s7_NlkBwdj8>
- <https://dblalock.substack.com/p/models-generating-training-data-huge>
- <https://news.mit.edu/2023/large-language-models-are-biased-can-logic-help-save-them-0303>
- [Logic Against Bias: Textual Entailment Mitigates Stereotypical Sentence Reasoning](https://arxiv.org/abs/2303.05670)
- <https://www.anthropic.com/research/mapping-mind-language-model>
- <https://bharathpbhat.github.io/2021/04/04/getting-confidence-estimates-from-neural-networks.html>
- <https://gist.github.com/yoavg/4e4b48afda8693bc274869c2c23cbfb2>
- <https://blog.jxmo.io/p/we-should-stop-talking-about-agi>
- [The Reversal Curse: LLMs trained on "A is B" fail to learn "B is A"](https://arxiv.org/abs/2309.12288)

#ML
