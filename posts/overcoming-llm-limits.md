---
title: 'Overcoming the limits of current LLM'
date: '2024-07-18'
---

Large language models (LLM) have been all the rage for quite some time now. Looking beyond the hype though, they have severe limitations: hallucinations, lack of confidence estimates and lack of citations.

Hallucination refers to the phenomenon where LLM generates content that sounds convincing / factual but is actually ungrounded or plain wrong.

A confidence estimate assigns a confidence number to a prediction and is useful to estimate factuality. Though a high confidence score for a wrong answer is worse than no confidence score to begin with… So that may be the reason I have not seen any in commercial products. Though OpenAI released this: <https://openai.com/index/teaching-models-to-express-their-uncertainty-in-words/>

Citations are just sources on which a text is based. This can be achieved via so-called RAG techniques (just search over a text corpus and hope to find relevant documents, which are added to the query and cited). Good examples are <https://perplexity.ai> and <https://github.com/stanford-oval/WikiChat>. RAG can be useful to detect if a prompt is in or out of the language models training data distribution, possibly useful to reduce hallucinations as demonstrated in WikiChat by Stanford.

A better LLM chatbot would address ideally all three of these limitations. So what are possible pathways to achieve them?

Hallucinations are certainly the toughest nut to crack and their negative impact is basically only slightly lessened by good confidence estimates and reliable citations (sources).

The impact of contradictions in the training data: Current language models are incapable of "self-inspection" to uncover logical inconsistencies in their training data. Only in the input context window should they be able to find logical inconsistencies.

## Bootstrapping consistent LLM

TL;DR: Exclude contradicting training data -> supervise the training process.

A LLM that selects its own training data to bootstrap itself: Curate a small dataset that is coherent, logical and truthful as good as possible, train the base model. Then use this base model to select or reject more training data to train on. This may alleviate inconsistencies. Researchers at MIT actually did something very similar: <https://news.mit.edu/2023/large-language-models-are-biased-can-logic-help-save-them-0303> (paper: <https://arxiv.org/abs/2303.05670>)

The consistency bootstrapping could work something like this: manually curate a high-quality (consistent) text corpus based on undisputed, well curated wikipedia articles and battle tested scientific literature. Train a base LLM on this curated training data. Use this base model to classify new text documents as consistent or inconsistent by using RAG on the curated training corpus. Gradually extend the training data with consistent text documents and finally train a big consistent LLM.

One could spin this idea even further and train several models with radically different world views by curating different training corpi that represent different sets of beliefs / world views.

I hope to see more research exploring this consistent data bootstrapping approach for LLM.

## Other Hallucination Mitigation Strategies

**Entropy Based**: Analyze the entropy (variance) of the logits. High-variance should indicate uncertainty. <https://github.com/xjdr-alt/entropix>

**Semantic Entropy Based**: Sample multiple answers (high temperature) and estimate confidence based on the variance of the resulting answers embedding vectors.

-> These approaches have a major flaw though (as pointed out by this HN commenter): “..., there is a real problem with this approach that lies at the mathematical level.

For any given input text, there is a corresponding output text distribution (e.g. the probabilities of all words in a sequence which the model draws samples from).

The approach of drawing several samples and evaluating the entropy and/or disagreement between those draws is that it relies on already knowing the properties of the output distribution. It may be legitimate that one distribution is much more uniformly random than another, which has high certainty. Its not clear to me that they have demonstrated the underlying assumption.

Take for example celebrity info, "What is Tom Cruise known for?". The phrases "movie star", "katie holmes", "topgun", and "scientology" are all quite different in terms of their location in the word vector space, and would result in low semantic similarity, but are all accurate outputs.

On the other hand, "What is Taylor Swift known for?" the answers "standup comedy", "comedian", and "comedy actress" are semantically similar but represent hallucinations. Without knowing the distribution characteristics (e.g multivariate moments and estimates) we couldn't say for certain these are correct merely by their proximity in vector space.

As some have pointed out in this thread, knowing the correct distribution of word sequences for a given input sequence is the very job the LLM is solving, so there is no way of evaluating the output distribution to determine its correctness.

There are actual statistical models to evaluate the amount of uncertainty in output from ANNs (albeit a bit limited), but they are probably not feasible at the scale of LLMs. Perhaps a layer or two could be used to create a partial estimate of uncertainty (e.g. final 2 layers), but this would be a severe truncation of overall network uncertainty.

Another reason I mention this is most hallucinations I encounter are very plausible and often close to the right thing (swapping a variable name, confabulating a config key), which appear very convincing and "in sample", but are actually incorrect.” - <https://news.ycombinator.com/item?id=40769496>

## Outlook

The endgame would be language models that actively seek new knowledge aka craft their own data sets to update their knowledge / weights on. (This seems related: <https://x.com/koltregaskes/status/1818265121090666904>)

## References

- Great talk on current limits of LLM: <https://www.youtube.com/watch?v=s7_NlkBwdj8>
- <https://dblalock.substack.com/p/models-generating-training-data-huge>
- <https://news.mit.edu/2023/large-language-models-are-biased-can-logic-help-save-them-0303>
- <https://arxiv.org/abs/2303.05670>
- <https://www.anthropic.com/research/mapping-mind-language-model>
- <https://bharathpbhat.github.io/2021/04/04/getting-confidence-estimates-from-neural-networks.html>
- <https://gist.github.com/yoavg/4e4b48afda8693bc274869c2c23cbfb2>

#machine-learning
