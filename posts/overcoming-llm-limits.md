---
title: 'Overcoming the limits of current LLM'
date: '2024-07-18'
---

Large language models (LLM) have been all the rage for quite some time now. Looking beyond the hype though, they have severe limitations: hallucinations, lack of confidence estimates and lack of citations.

Hallucination refers to the phenomenon where LLM generates content that sounds convincing / factual but is actually ungrounded or plain wrong.

A confidence estimate assigns a confidence number to a prediction and is useful to estimate factuality. A high confidence score for a wrong answer is worse than no confidence score to begin with… So that may be the reason I have not seen any in commercial products.

Citations are just sources on which a text is based. This can be achieved via so-called RAG techniques (just search over a text corpus and hope to find relevant documents, which are added to the query and cited). Good examples are <https://perplexity.ai> and <https://wikichat.genie.stanford.edu/>.

A better LLM chatbot would address ideally all three of these limitations. So what are possible pathways to achieve them?

Hallucinations are certainly the toughest nut to crack and their negative impact is basically only slightly lessened by good confidence estimates and reliable citations (sources).

The impact of contradictions in the training data. LLM are incapable of "self-inspection" on their training data to find logical inconsistencies in it but in the input context window they should be able to find logical inconsistencies.

Bootstrapping consistent LLM: exclude contradicting training data -> supervise the training process. A LLM that selects its own training data to bootstrap itself. Curate a small dataset that is coherent, logical and truthful as good as possible, train the base model. Then use this base model to select or reject more training data to train on. This may alleviate inconsistencies. Researchers at MIT actually did something very similar: <https://news.mit.edu/2023/large-language-models-are-biased-can-logic-help-save-them-0303> (paper: <https://arxiv.org/abs/2303.05670>)

The consistency bootstrapping could work something like this: manually curate a high-quality (consistent) text corpus based on undisputed, well curated wikipedia articles and battle tested scientific literature. Train a base LLM on this curated training data. Use this base model to classify new text documents as consistent or inconsistent by using RAG on the curated training corpus. Gradually extend the training data with consistent text documents and finally train a big consistent LLM.

One could spin this idea even further and train several models with radically different world views by curating different training corpi that represent different sets of beliefs / world views.

I hope to see more research exploring this consistent data bootstrapping approach for LLM.

References:

- <https://dblalock.substack.com/p/models-generating-training-data-huge>
- <https://news.mit.edu/2023/large-language-models-are-biased-can-logic-help-save-them-0303>
- <https://arxiv.org/abs/2303.05670>
- <https://www.anthropic.com/research/mapping-mind-language-model>
- <https://bharathpbhat.github.io/2021/04/04/getting-confidence-estimates-from-neural-networks.html>