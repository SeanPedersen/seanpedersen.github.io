---
date: '2025-08-16'
icon: "/images/icons/search.svg"
---
# Search is hard

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

* **Word Level (BM25 / normalized TF-IDF)**
  * Tokenization: standard words
  * Purpose: relevance scoring for full-text search
  * Strengths: scales efficiently to very large text corpora
  * Limitation: cannot match partial words, abbreviations, or morphological variants

* **Subword Level (Corpus-Derived Subword Index – CDSI)**
  * Tokenization: greedy longest-match subwords mined from frequent corpus substrings
  * Purpose: partial-word and compound-word matching in full text
  * Strengths: captures abbreviations and morphemes (e.g., `"neural net"` matches `"neural network"`), smaller posting lists than tri-grams, deterministic and interpretable
  * Limitation: requires offline vocabulary build and periodic rebuilds

* **Character Level (Tri-gram Index)**
  * Tokenization: overlapping 3-character sequences
  * Purpose: substring and fuzzy search
  * Strengths: handles typos, partial matches, and short strings (filenames, codes, IDs)
  * Limitation: high space usage; mainly useful for short strings rather than full text

| Method              | Space       | Substring coverage | Fuzzy/typo tolerance | Best use case                             |
| ------------------- | ----------- | ------------------ | -------------------- | ----------------------------------------- |
| Tri-grams           | High        | All                | Limited              | Short strings, small-medium corpora       |
| Variable n-grams    | Medium      | Most               | Limited              | Medium-length strings                     |
| Prefix/Suffix       | Low-Med     | Start/End only     | No                   | Autocomplete, filenames                   |
| Permuterm           | Medium-High | Arbitrary          | No                   | Wildcard/substring search                 |
| Q-gram + Hashing    | Low         | Most               | Approximate          | Large-scale, fuzzy search                 |
| Suffix Array / Tree | Medium      | All                | Exact                | Exact substring matching, static datasets |


### Neural Representations

Create representations using deep neural networks - typically dense vectors (embeddings).

- Multi-Modal (shared) Embedding Spaces (CLIP)
- Dense vs Sparse embeddings ([SDR](/posts/sparse-distributed-representations))
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

All of the following techniques trade retrieval accuracy for speed / storage costs. For production use cases, using a [Vector DB](/posts/vector-databases) is the right choice.

- Approximate Nearest Neighbor (Search Index) - [benchmark](https://github.com/erikbern/ann-benchmarks)
    - HNSW: builds a hierachical graph, good data drift handling -> high RAM usage
    - IVFFlat: low RAM usage, bad data drift handling -> needs frequent rebuilds
    - DiskANN: low RAM usage, achieved by using disk (needs fast disk read)
    - TODO (eval): <https://github.com/yichuan-w/LEANN>
- Vector Dimensionality Reduction
    - Matryoshka Embeddings: dim. red. baked into training via loss function
    - PCA (linear)
    - t-SNE / UMAP (non-linear)
- Vector Quantization
    - Reduce bit representation (f.e. to INT8 instead of FP32)
    - [Near-lossless compression for unit-norm embedding vectors using spherical coordinates](https://jina.ai/embedding-compression.pdf) - [code](https://github.com/jina-ai/jzip-compressor)

Excellent article detailing an efficient vector search pipeline: <https://huggingface.co/blog/embedding-quantization>

## Diversification

It often makes sense to present the user for general queries not only the top nearest neighbors but a diverse set of results, so the ambiguity of the query is reflected. Ideally the user can then narrow down (load more relevant results) based on the diverse result set.

## Evaluation Metrics

As you see there are many knobs to tune a modern search pipeline and thus we need hard evaluation metrics to judge the quality of our search pipeline. Creating a custom dataset that mirrors our real world search use case as closely and diversely as possible is essential to improving our search. Relevant [post](https://x.com/bo_wangbo/status/2011075744978649199) - recommends optimizing for recall with K > 100.

### Precision@K

- What: Fraction of top K results that are relevant
- Formula: Relevant results in top K ÷ K
- Use when: You only care about the quality of what users see
- Limitation: Doesn't measure how many relevant documents were missed

### Recall@K

- What: Fraction of all relevant documents found in top K
- Formula: Relevant found in top K ÷ total relevant documents
- Use when: Missing relevant information is costly
- Limitation: Requires knowing all relevant documents

### F1 Score@K

- What: Balance between precision and recall
- Formula: 2 × (Precision × Recall) ÷ (Precision + Recall)
- Use when: Comparing systems with different precision/recall trade-offs
- Key insight: Penalizes systems that optimize only one metric

## Ideas to Explore

User steered semantic search by selecting N matches and finding the common subspace in their embeddings

## References

- [https://github.com/frutik/awesome-search](https://github.com/frutik/awesome-search)
- [What is Presentation Bias in search?](https://softwaredoug.com/blog/2022/07/16/what-is-presentation-bias-in-search)
- [What AI Engineers Should Know about Search](https://softwaredoug.com/blog/2024/06/25/what-ai-engineers-need-to-know-search)
- [HN Discussion: What every software engineer should know about search](https://news.ycombinator.com/item?id=15231302)
- [Ranking Anything with GPT4](https://binal.pub/2023/04/ranking-anything-with-gpt4/)
- [RankZephyr: Effective and Robust Zero-Shot Listwise Reranking is a Breeze!](https://arxiv.org/abs/2312.02724)
- [Applying Embedding-Based Retrieval to Airbnb Search](https://arxiv.org/abs/2601.06873)
- [LLM based reranker](https://python.useinstructor.com/blog/2024/10/23/building-an-llm-based-reranker-for-your-rag-pipeline/)
- [Building a web search engine from scratch in two months with 3 billion neural embeddings](https://blog.wilsonl.in/search-engine/)
- [On the Theoretical Limitations of Embedding-Based Retrieval](https://arxiv.org/abs/2508.21038)
- [Building a Simple Search Engine That Actually Works](https://karboosx.net/post/4eZxhBon/building-a-simple-search-engine-that-actually-works)
- [Full Unicode Search at 50× ICU Speed with AVX‑512](https://ashvardanian.com/posts/search-utf8/)

#coding #ML
