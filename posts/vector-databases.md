---
title: 'Postgres as Vector DB - a benchmark'
date: '2025-10-08'
---
There is a flood of vector databases - which ones are actually useful? IMO extending a relational DBMS with ACID compliance and existing datasets, is for most use cases the ideal choice. Using a dedicated vector DB like ([Chroma](https://www.trychroma.com/), [Vespa](https://github.com/vespa-engine/vespa), [Turbopuffer](https://turbopuffer.com/), [LanceDB](https://github.com/lancedb/lancedb), [Milvus](https://milvus.io/) etc.) only makes sense for narrow use cases where no complicated meta-data filters are needed (e.g. just simple RAG) and data synchronisation is no issue.

So let's have a look how we can store and search vectors using Postgres - there are three notable extensions for Postgres: pgvector, pgvectorscale and vectorchord.

## [PGVector](https://github.com/pgvector/pgvector)

Standard extension to store vectors with common medium scale ANN indices (HNSW & IvfFlat). Integration package for Python: <https://github.com/pgvector/pgvector-python/>

Store vectors (float 32 bit):

```sql
CREATE TABLE items (id bigserial PRIMARY KEY, embedding vector(1024));
```

Store half-precision (float 16 bit) vectors:

```sql
CREATE TABLE items (id bigserial PRIMARY KEY, embedding halfvec(1024));
```

### [HNSW](https://github.com/nmslib/hnswlib)

Hierarchical navigable small world (HNSW) is a popular graph based ANN index - delivering good retrieval and QPS performance but at the cost of longer index build times and using more RAM for it (did crash for 1 million vectors on my 16GB RAM machine).

### IvfFlat

Inverted File Flat (IvfFlat) is less RAM hungry than HNSW. With caveats: IvfFlat performance degrades as vectors are deleted and added (because centroids are not updated), thus needing regular index rebuilds (if new vectors get inserted regularly).

"As data gets inserted or deleted from the index, if the index is not rebuilt, the IVFFlat index in pgvector can return incorrect approximate nearest neighbors due to clustering centroids no longer fitting the data well"

#### IvfFlat Binary (with reranking)

Build IvfFlat index on binarized vectors -> on query: overfetch Kx10 nearest neighbors, then rerank using float16/32 vectors to get more accurate final K-NN.

```sql
-- Mean threshold binarization (performs better than pgvector's binarizer)
CREATE OR REPLACE FUNCTION binary_quantize_mean(vec halfvec) 
RETURNS varbit AS $$
    SELECT string_agg(
        CASE WHEN val >= mean_val THEN '1' ELSE '0' END, 
        ''
    )::varbit
    FROM unnest(vec::real[]) AS val,
         (SELECT AVG(v) AS mean_val FROM unnest(vec::real[]) AS v) AS stats;
$$ LANGUAGE sql IMMUTABLE PARALLEL SAFE;

-- Step 1: Add a stored generated binary column
ALTER TABLE table_name 
ADD COLUMN vector_bin bit(512) 
GENERATED ALWAYS AS (binary_quantize_mean(vector)::bit(512)) STORED;

-- Step 2: Create index on the pre-computed binary column
CREATE INDEX IF NOT EXISTS table_name_idx_ivfflat_bin 
ON table_name 
USING ivfflat (vector_bin bit_hamming_ops) 
WITH (lists = 100);

-- Query top 100 NN using reranking
SELECT t.id
FROM (
  SELECT id
  FROM table_name
  ORDER BY vector_bin <~> binary_quantize_mean(%s::halfvec(512))::bit(512)
  LIMIT 1000
) c
JOIN table_name t USING(id)
ORDER BY t.vector <=> %s::halfvec(512)
LIMIT 100;
```

## [PGVectorScale](https://github.com/timescale/pgvectorscale)

### DiskANN

A promising ANN index that uses RAM and disk (needs fast disk - SSD / NVMe) to scale to billions of vectors, promising low RAM usage while still providing decent QPS. The problem is that the pgvectorscale index building implementation is [single core right now](https://github.com/timescale/pgvectorscale/issues/38) - leading to very long index generation times.

PGVectorScale supports pre-filtering using bitfields with manual meta-data table setup (complicated / bad dev ux).

## [VectorChord](https://github.com/tensorchord/VectorChord)

### [VChordRQ](https://docs.vectorchord.ai/vectorchord/usage/indexing.html)

A custom ANN index with excellent performance (combining IVF ANN index with RaBitQ quantization). Supports pre-filtering (easy to use).

VChord can be configured to not copy all vectors into the index (which is the default and pgvector also does), reducing disk usage - it can be [enabled](https://docs.vectorchord.ai/vectorchord/usage/rerank-in-table.html) (slightly degrading performance).

VChord also supports efficient [range filters](https://docs.vectorchord.ai/vectorchord/usage/range-query.html) (limiting results by distance instead of K nearest neighbors).

### VChordG (DiskANN)

A novel addition (not prodution ready yet): custom implementation of DiskANN index combined with RaBitQ quantization.

- <https://blog.vectorchord.ai/vectorchord-05-new-rabitq-empowered-diskann-index-and-continuous-recall-measurement?source=more_articles_bottom_blogs>

## Benchmark

The benchmark should reflect realistic usage - right now it just measures index build and query times.

In the future I want to extend it:
- measure insertion perormance after initial index is built
- use real data embedding vectors -> simulate data distribution shift (insert many vectors after index was built)
- simulate realistic complex SQL queries involving categorical and range filtering
- benchmark vector scales: 100K, 1M, 10M, 100M, 1B

## ANN Benchmark Results

The table below was computed using 100K CLIP Matryoshka text embeddings.

| Index | Recall@100 (%) | Latency (ms) | Dim | Storage | Build (s) | Storage (MB) | Index (MB) |
|-------|------------|--------------|-----|---------|-----------|--------------|------------|
| binary_mean_ivf_rerank | 90.0 | 14.20 | 1024 | float32 | 2.12 | 429.7 | 14.6 |
| binary_mean_ivf_rerank | 92.0 | 15.10 | 1024 | float16 | 0.86 | 214.8 | 14.6 |
| binary_uint8_ivf_rerank | 100.0 | 37.67 | 1024 | float32 | 7.43 | 429.7 | 98.1 |
| binary_uint8_ivf_rerank | 100.0 | 39.94 | 1024 | float16 | 4.75 | 214.8 | 98.1 |
| binary_uint4_ivf_rerank | 100.0 | 17.00 | 1024 | float32 | 2.77 | 429.7 | 52.5 |
| binary_uint4_ivf_rerank | 100.0 | 17.06 | 1024 | float16 | 3.78 | 214.8 | 52.5 |
| vchordrq | 96.0 | 18.06 | 1024 | float32 | 12.34 | 429.7 | 50.2 |
| vchordrq | 97.0 | 25.38 | 1024 | float16 | 11.68 | 214.8 | 47.6 |
| diskann | 96.0 | 18.48 | 1024 | float32 | 156.78 | 429.7 | 55.8 |
||
| binary_mean_ivf_rerank | 84.0 | 13.90 | 512 | float32 | 0.84 | 214.8 | 8.5 |
| binary_mean_ivf_rerank | 84.0 | 11.70 | 512 | float16 | 0.77 | 107.4 | 8.5 |
| binary_uint8_ivf_rerank | 94.0 | 21.77 | 512 | float32 | 2.54 | 214.8 | 52.5 |
| binary_uint8_ivf_rerank | 94.0 | 20.03 | 512 | float16 | 3.47 | 107.4 | 52.5 |
| binary_uint4_ivf_rerank | 94.0 | 14.34 | 512 | float32 | 1.15 | 214.8 | 27.3 |
| binary_uint4_ivf_rerank | 94.0 | 12.03 | 512 | float16 | 1.23 | 107.4 | 27.4 |
| vchordrq | 94.0 | 27.55 | 512 | float32 | 7.70 | 214.8 | 41.5 |
| vchordrq | 94.0 | 23.27 | 512 | float16 | 7.51 | 107.4 | 40.0 |
| diskann | 94.0 | 31.49 | 512 | float32 | 159.26 | 214.8 | 55.8 |
||
| binary_mean_ivf_rerank | 73.0 | 11.88 | 256 | float32 | 0.47 | 107.4 | 5.4 |
| binary_mean_ivf_rerank | 73.0 | 9.72 | 256 | float16 | 0.33 | 53.7 | 5.4 |
| binary_uint8_ivf_rerank | 86.0 | 13.94 | 256 | float32 | 1.63 | 107.4 | 27.4 |
| binary_uint8_ivf_rerank | 86.0 | 11.89 | 256 | float16 | 1.26 | 53.7 | 27.3 |
| binary_uint4_ivf_rerank | 84.0 | 75.42 | 256 | float32 | 1.26 | 107.4 | 14.6 |
| binary_uint4_ivf_rerank | 84.0 | 13.22 | 256 | float16 | 1.15 | 53.7 | 14.6 |
| vchordrq | 86.0 | 22.14 | 256 | float32 | 7.22 | 107.4 | 36.6 |
| vchordrq | 86.0 | 16.66 | 256 | float16 | 7.16 | 53.7 | 35.9 |
| diskann | 86.0 | 25.21 | 256 | float32 | 152.00 | 107.4 | 48.8 |

## Optimizing Vector & Index Storage

When things scale up one should strive for efficient vector storage using:
- Vector Dimensionality Reduction
    - Product Quantization (PQ): reduces both dimensionality and bit representation
    - Matryoshka Embeddings: superior performance vs PQ as it learns the reduction in training and not post-training
    - PCA / UMAP
- Scalar Quantization (Reduce bit representation per dimension)
    - FP16: half precision from FP32 
    - binary: reduce to single bit
      - In pgvector: binary quantization will reduce any positive value to 1, and any zero or negative value to 0
      - Mean-based thresholding: binarize each dimension using its corpus-wide mean as the threshold, ensuring ~50/50 bit distribution per dimension for better information preservation and more discriminative binary codes.

We can see that using IvfFlat index on binary representation with a top-K factor of 10x (overfetching), then reranking in higher precision (float32 or float16) results in excellent recall, low vector storage costs (float16) and very fast retrieval latencies.

Though the reranking can lead to complications with queries using metadata filtering, so just using vchordrq index with 512D and float16 can also make sense.

## Show me the code

Check out the code [here](https://github.com/SeanPedersen/vector-db-benchmark/tree/main)

## Conclusion

VectorChord is a good choice - providing superior performance and better developer experience (pre-filtering and better default settings). The vchordrq index is for most use cases the ideal choice as it delivers great performance and handles data distribution drift better than diskann indices. Using an ANN index only starts to make sense for big numbers of vectors (over 1 million).

## References

- The gold standard ANN benchmark: <https://github.com/erikbern/ann-benchmarks>
- <https://vector-index-bench.github.io/overview.html>
- <https://docs.vectorchord.ai/vectorchord/usage/indexing.html>
- <https://blog.vectorchord.ai/vectorchord-store-400k-vectors-for-1-in-postgresql#heading-ivf-vs-hnsw>
- <https://blog.vectorchord.ai/3-billion-vectors-in-postgresql-to-protect-the-earth>
- <https://drscotthawley.github.io/blog/posts/2023-06-12-RVQ.html>
- <https://www.tigerdata.com/blog/nearest-neighbor-indexes-what-are-ivfflat-indexes-in-pgvector-and-how-do-they-work>
- <https://jkatz05.com/post/postgres/pgvector-scalar-binary-quantization/>
- <https://zilliz.com/learn/scalar-quantization-and-product-quantization>
- <https://medium.com/nlp-experiment/product-quantization-d66fdb860047>
- <https://medium.com/@bavalpreetsinghh/pgvector-hnsw-vs-ivfflat-a-comprehensive-study-21ce0aaab931>

#coding #ML
