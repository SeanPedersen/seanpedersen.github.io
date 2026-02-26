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

RAG vs Fine-Tune:
- RAG for fast changing document corpus
- Fine-Tune for static corpus with complex reasoning or novel (niche / specific) language
    - Fine-tune both LLM and embedding model to improve the whole pipeline
- Best: hybrid approach

- <https://x.com/_avichawla/status/1964939267823775997>

Evaluation:
- always create RAG specific benchmark for your problem domain to validate progress

## Local RAG

Use dspy to optimise system prompts.

Existing Solutions:
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

## 
## Benchmarks
You can not improve, what you can not see - running evals for your RAG apps is crucial to ship useful products (bad example: [Elysium by weaviate](https://github.com/weaviate/elysia/issues/48#issuecomment-3286114697)).

- https://arxiv.org/abs/2311.11944
- https://github.com/D-Star-AI/KITE/tree/main
- https://arxiv.org/abs/2509.11552

## References
- <https://d-star.ai/solving-the-out-of-context-chunk-problem-for-rag>
- <https://d-star.ai/dynamic-retrieval-granularity>
- <https://maven.com/p/acfa67/improving-rag-embedding-models-representation-learning>
- <https://softwaredoug.com/blog/2025/12/09/rag-users-want-affordances-not-vectors>

#ML #coding
