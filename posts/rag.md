# Retrieval Augmented Generation

RAG works by augmenting the context window of an LLM with text relevant to the user's query to provide a useful response (reduce hallucinations). Thus if one wants to master RAG, one has to master [search](/posts/search-is-hard).

## RAG Approaches
- Simple RAG: basic keyword search / semantic search
- Complex RAG: advanced chunking (e.g. late-interaction / enrich with LLM -> infer questions a chunk can answer or summarize a long document) semantic search
- Graph RAG: construct knowledge graph to uncover hidden (multi-step) connections for complex queries (how did x evolve over time? how does y compare to z? explain u in terms of v, etc.)
- Agentic RAG:
  - let LLM create search queries based on user query
  - let LLM make final selection from result candidates

## Granularity Problem
Different queries require different granularity (number of chunks provided as context) -> who is the CEO of Apple vs summarize the bible

**Solution:**
relevant segment extraction (RSE) - <https://d-star.ai/dynamic-retrieval-granularity>

just use paragraph-sized chunks (~200 tokens), prepend global document context (title, section) -> embed and store position in document (line number / page position)

embed query -> find nearest chunks -> compute continuous chunk segments in documents

## Hallucination Problem

How to reduce hallucinations? How to increase the response rate of "I do not know based on available information" -> search needs to be capable to indicate high uncertainty / unrelated results only -> leading to I do not know response

Fact check using semantic search and fine-tuned LLM: <https://github.com/stanford-oval/WikiChat>

## RAG Pipeline

Based on user input query -> use LLM to generate relevant search queries -> use semantic search to find cadidates (below a similarity threshold) -> Reranker to filter (fast) -> use LLM (slow) to filter for relevancy -> use filtered results to generate final answer

Key Points:
- agentic search - agent decides how to search: hybrid search (semantic + keyword), custom SQL query, etc.
  - LLM should transform user query before search, as user query will not always be the right format to answer the question it entails. f.e. what are the trends in tech? -> is very abstract, LLM can transform it into a format that will match the answer document semantics
- embedding chunking strategy
- graph-enhanced retrieval (supermemory etc.)
- domain specific benchmark (use powerful commercial LLM's to eval answers of local LLM's) to work data driven

UX:
- show progress (searching / filtering / analyzing)
- show user citations (used results)

## RAG vs Fine-Tuning

When to pick what:

| Dimension | RAG | Fine-Tuning |
|---|---|---|
| Corpus volatility | Fast changing (docs added daily) | Static / slow changing |
| Knowledge type | Factual lookup, citations needed | Style, tone, format, reasoning patterns |
| Traceability | Sources are visible to user | Black-box, hard to attribute |
| Update cost | Re-index (cheap, minutes) | Re-train (expensive, hours-days) |
| Hallucination risk | Lower (grounded in retrieved text) | Higher (memorized, can drift) |
| Cold start | Works with zero training data | Needs labeled examples |
| Latency | Higher (search + LLM) | Lower (single forward pass) |
| Best for | Q&A over private docs, support, news | Domain language, niche jargon, structured output |

Use **RAG** when:
- corpus changes faster than you can retrain
- citations / source attribution are required (legal, medical, support)
- knowledge is too large to fit in weights or context
- you need to gate access (per-user permissions on documents)

Use **Fine-Tuning** when:
- corpus is small, stable and dense with niche language (medical codes, legal jargon, internal product names)
- you need a specific output format / style the base model fails at
- latency budget is tight (no time for retrieval)
- task is reasoning-heavy and pattern-based, not fact-based

**Hybrid (usually the right answer for production):**
- fine-tune embedding model on domain pairs to improve retrieval quality
- fine-tune the generator LLM for output format + citation behavior
- keep RAG for the actual facts -> get the best of both
- <https://x.com/_avichawla/status/1964939267823775997>

## Local RAG

Use dspy to optimise system prompts.

Existing Solutions:
- https://github.com/Cinnamon/kotaemon: Chat with PDFs
- https://www.onyx.app/: Integrated solution connecting to many clouds
- https://github.com/stanford-oval/WikiChat: Minimal hallucinations
- https://github.com/D-Star-AI/dsRAG: Advanced chunking for text
- https://github.com/weaviate/elysia: Agentic RAG (can create custom DB queries + semantic search etc.)
- https://github.com/HKUDS/LightRAG
- https://github.com/infiniflow/ragflow
- https://github.com/langgenius/dify
- https://github.com/datastax/ragstack-ai
- https://github.com/neuml/txtai
- [Perplexica](https://github.com/ItzCrazyKns/Perplexica)
- Eliminates RAG: https://github.com/microsoft/KBLaM

Retrieval:
- https://github.com/VectifyAI/PageIndex
- https://github.com/yichuan-w/LEANN

Memory Systems:
- https://github.com/campfirein/cipher
- https://github.com/supermemoryai/supermemory
- https://github.com/getzep/graphiti

## Benchmarks

You can not improve, what you can not see - running evals for your RAG apps is crucial to ship useful products (bad example: [Elysium by weaviate](https://github.com/weaviate/elysia/issues/48#issuecomment-3286114697)).

### Eval the pipeline in stages

A RAG system has two failure modes that compound: bad retrieval and bad generation. Eval them separately or you will not know what to fix.

**1. Retrieval metrics** (does the right chunk make it into context?)
- **Recall@k**: out of the top-k retrieved chunks, did at least one ground-truth chunk appear? (most important - if the answer is not in context, generation can not save you)
- **MRR (Mean Reciprocal Rank)**: 1 / rank of first relevant chunk - rewards putting the right chunk near the top
- **nDCG@k**: rewards ranking quality when there are multiple relevant chunks of varying importance
- **Context precision**: fraction of retrieved chunks that are actually relevant - low precision wastes tokens and confuses the LLM

**2. Generation metrics** (given good context, is the answer good?)
- **Faithfulness / groundedness**: every claim in the answer is supported by the retrieved context (catches hallucinations)
- **Answer relevancy**: answer addresses the actual question, not a tangent
- **Citation accuracy**: do the cited chunks actually contain the cited claim?
- **Answer correctness**: matches a gold-standard answer (semantic similarity, not exact match)

**3. End-to-end metrics**
- **Task success rate**: did the user get what they needed? (the only metric that really matters)
- **"I do not know" rate**: how often does the system correctly refuse vs hallucinate when no answer exists - test with adversarial out-of-corpus queries
- **Latency p50 / p95**: real users care
- **Cost per query**: tokens in + tokens out + embedding + reranker calls

### Building a domain benchmark

Generic benchmarks (BEIR, MTEB) tell you about general retrieval quality. They do not predict performance on your private corpus. Always build a domain eval set:

1. **Collect 50-200 real queries** from logs, support tickets, or user interviews - synthetic queries miss the long tail of how users actually phrase things
2. **Generate ground truth** - for each query annotate (a) which chunks contain the answer (b) the gold answer text. Use a strong LLM (Claude Opus, GPT-5) to draft, human to verify
3. **Cover edge cases** - multi-hop questions, queries with no answer in corpus, ambiguous queries, queries that need recent docs
4. **Freeze the set** - if you keep changing it you can not compare runs over time
5. **Track regressions per query** - aggregate scores hide that v2 fixed 5 queries but broke 3 others

### LLM-as-judge

For groundedness, relevancy, correctness - use a strong LLM to score outputs of your (cheaper, smaller) production LLM. Cheap, scales, correlates with human judgment when prompted well.

Caveats:
- judge model bias (LLMs prefer their own family's outputs)
- ask for binary judgments + reason, not 1-10 scores (more reliable)
- spot-check with humans on 10-20% of cases to keep the judge honest

### Tools

- [Ragas](https://github.com/explodinggradients/ragas) - faithfulness, answer relevancy, context precision/recall out of the box
- [TruLens](https://github.com/truera/trulens) - feedback functions and tracing
- [DeepEval](https://github.com/confident-ai/deepeval) - pytest-style RAG evals
- [promptfoo](https://github.com/promptfoo/promptfoo) - eval harness with regression tracking
- [KITE](https://github.com/D-Star-AI/KITE/tree/main) - retrieval benchmark
- run evals in CI - block PRs that regress recall@k or faithfulness beyond a threshold

## References
- <https://arxiv.org/abs/2311.11944> (RAG benchmark survey)
- <https://arxiv.org/abs/2509.11552>
- <https://d-star.ai/solving-the-out-of-context-chunk-problem-for-rag>
- <https://d-star.ai/dynamic-retrieval-granularity>
- <https://maven.com/p/acfa67/improving-rag-embedding-models-representation-learning>
- <https://softwaredoug.com/blog/2025/12/09/rag-users-want-affordances-not-vectors>
- https://mksg.lu/blog/gemini-rag-cloudflare-workers

#ML #coding
