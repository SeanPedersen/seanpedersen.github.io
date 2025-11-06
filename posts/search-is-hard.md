---
title: 'Search is hard'
date: '2025-08-16'
---
Search is the process of finding useful matches in a (large) set of objects (search corpus). A search engine is only as good as the representations it uses to match queries to the objects. To find things fast, one orders the search corpus by structuring the objects in a way (a graph) that makes it easy to navigate - this is called an index. A useful search engine understands the users intent and mirrors how humans associate information (semantics, syntax, sound, visual), so the results feel intuitive.

## Use Cases

- Search Interfaces (for books, web pages, images, etc.)
- Recommender Systems (for similar movies, products, etc.)
- Retrieval Augemented Generation (provide relevant context for LLM's)

## Representations

The key of every search system is to build useful representations of your search corpus, so you can (quickly) find what you search for. Thus one should spend spend good time on finding the right representation for the search problem and how to enrich it with useful information.

### Granularity (chunking)

Depending on the use case or query, different levels of information granularity of the search corpus are useful. Searching for a whole book / chapter / paragraph / sentence / word / letter or video / scene / image / objects / textures / colors.

Tricks:
- Use LLM to enrich your search corpus with relevant metadata (do the work before the search)
    - Ask LLM to provide a descriptive title and a short summary for each text document
    - Prepend title and summary / section to every text chunk before embedding

### Token Based Representations

- Word level: BM25 (normalized TF-IDF)
- Character level: Substring search (tri-gram index)

### Neural Representations

- Multi-Modal (shared) Embedding Spaces (CLIP)
- Dense vs Sparse ([SDR](https://seanpedersen.github.io/posts/sparse-distributed-representations))
- Query vs document prefix for embedding models optimized for search
- Visual Document Embeddings
    - Instead of embedding sequences of tokens (often words), embed images of pages directly -> this helps to process complex documents containing tables, graphs and imagery
    - <https://huggingface.co/collections/nomic-ai/nomic-embed-multimodal-67e5ddc1a890a19ff0d58073>
    - <https://huggingface.co/jinaai/jina-embeddings-v4>
- Text Embeddings (chunking strategies)
    - relevant segment extraction (RSE) - <https://d-star.ai/dynamic-retrieval-granularity>: just use paragraph-sized chunks (~200 tokens), prepend global document context (title, section) -> embed and store position in document (line number / page position). embed query -> find nearest chunks -> compute continuous chunk segments in documents
    - late-interaction: do not compress tokens into single vector, instead vectorize every token (using contextual embedding model like ColBERT, so token vectors contain contextual information) and match using the MaxSim operator
        - Pros: much more granular results (excel at out-of-domain, long-context), good interpretability because of token-level matching
        - Cons: heavy computation -> search takes longer, storing cost -> n token vectors per document vs 1 document vector
    - embed a document by computing sentence level dense embeddings. Transform these sentence embeddings into SDR that are stacked to preserve information. Now match a query with the same technique to find relevant docs.
    - use LLM to enrich representations
      - summarize document and embed it (harnesses LLM to solve long-context problem)
      - infer questions from the chunk that it answers and embed them

## Searching

Once our set of objects is transformed into useful representations (vectors), we can finally search. We need to transform the user query into the same representation to compare it with the existing set of vectors. The most common metric for this is the cosine distance (angle between two vectors -> 0: identical / very similar vectors, 1: orhogonal / very different vectors).

## Reranker Models

Serve the purpose to run a complex (big) model (specialised or LLM) to filter the results (candidates) of the vector similarity search further by computing another similarity score using the reranker model with the query for each candidate.

## Modern Search Pipelines

Depending on your requirements (accuracy and time to response) pipelines of different complexity are possible.

**Simple**
fast but may contain irrelevant results

- Document -> chunk -> embed (index)
- Query -> embed -> search on chunks -> results

**Reranker**

- Document -> chunk -> embed (index)
- Query -> embed -> search on chunks -> rerank (filter) -> results

**LLM**

- Document -> LLM (generate descriptive title + summary) -> chunk -> embed (index)
- Query -> expand / rewrite query using LLM -> embed -> search on chunks -> rerank (filter) -> results

## Scaling Things Up

All of the following techniques trade retrieval accuracy for speed / storage costs. For production use cases, using a [Vector DB](https://seanpedersen.github.io/posts/vector-databases) is the right choice.

- Approximate Nearest Neighbor (Search Index)
    - HNSW (high RAM usage) and DiskANN (low RAM usage, needs SSD)
    - <https://github.com/erikbern/ann-benchmarks>
- Vector Dimensionality Reduction
    - PCA / UMAP
    - Matryoshka Embeddings
- Vector Quantization
    - Reduce bit representation (f.e. to INT8 instead of FP32)

## Evaluation Metrics

As you see there are many knobs to tune a modern search pipeline and thus we need hard evaluation metrics to judge the quality of our search pipeline. Creating a custom dataset that mirrors our real world search use case as closely and diversely as possible is essential to improving our search.

**Precision@K**

What it measures: Of the first K results returned, what percentage are actually relevant?

Formula: Precision@K = (Number of relevant results in top K) / K

Example:

- Search returns 10 results (K=10)
- 7 of those 10 are relevant to the query
- Precision@10 = 7/10 = 0.7 (70%)

When to use: When you care about the quality of results shown to users. High precision means users see fewer irrelevant results, improving their experience.

Limitations: Doesn't consider how many total relevant documents exist in your corpus. A system could have high precision but miss many relevant documents.


**Recall@K**

What it measures: Of all the relevant documents that exist in your corpus, what percentage appear in the top K results?

Formula: Recall@K = (Number of relevant results in top K) / (Total relevant documents in corpus)

Example:

- Your corpus contains 50 documents relevant to a query
- Top 10 results include 7 relevant documents
- Recall@10 = 7/50 = 0.14 (14%)

When to use: When it's important not to miss relevant information. Critical in medical, legal, or research contexts where missing a relevant document could have serious consequences.

Limitations: Requires knowing the total number of relevant documents in your corpus, which is often impractical for large datasets.


**F1@K**

What it measures: Harmonic mean of Precision@K and Recall@K, providing a single score that balances both metrics.

Formula: F1@K = 2 × (Precision@K × Recall@K) / (Precision@K + Recall@K)

Example:

- Precision@10 = 0.7
- Recall@10 = 0.14
- F1@10 = 2 × (0.7 × 0.14) / (0.7 + 0.14) = 0.233

When to use: When you need to balance precision and recall. Useful for comparing systems with different precision/recall trade-offs.

Key insight: F1 score is closer to the lower of the two values, so it penalizes systems that optimize one metric at the expense of the other.

## Ideas to Explore

User steered semantic search by selecting N matches and finding the common subspace in their embeddings

## References

- [https://github.com/frutik/awesome-search](https://github.com/frutik/awesome-search)
-   [https://softwaredoug.com/blog/2022/07/16/what-is-presentation-bias-in-search](https://softwaredoug.com/blog/2022/07/16/what-is-presentation-bias-in-search)
-   [https://softwaredoug.com/blog/2024/06/25/what-ai-engineers-need-to-know-search](https://softwaredoug.com/blog/2024/06/25/what-ai-engineers-need-to-know-search)
-   [https://news.ycombinator.com/item?id=15231302](https://news.ycombinator.com/item?id=15231302)
-   [https://binal.pub/2023/04/ranking-anything-with-gpt4/](https://binal.pub/2023/04/ranking-anything-with-gpt4/)
-   [https://arxiv.org/abs/2312.02724](https://arxiv.org/abs/2312.02724)
-   [https://python.useinstructor.com/blog/2024/10/23/building-an-llm-based-reranker-for-your-rag-pipeline/](https://python.useinstructor.com/blog/2024/10/23/building-an-llm-based-reranker-for-your-rag-pipeline/)
- [Building a web search engine from scratch in two months with 3 billion neural embeddings](https://blog.wilsonl.in/search-engine/)
- [On the Theoretical Limitations of Embedding-Based Retrieval](https://arxiv.org/abs/2508.21038)

#programming #machine-learning
