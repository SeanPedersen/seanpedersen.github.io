---
title: 'Postgres as Vector DB - a benchmark'
date: '2025-10-08'
---
There is a flood of vector databases - which ones are actually useful? IMO extending a relational DBMS with ACID compliance and existing datasets, is for most use cases the ideal choice. Using a dedicated vector DB like (Chroma, Turbopuffer, [LanceDB](https://github.com/lancedb/lancedb) etc.) only makes sense for narrow use cases where no complicated meta-data filters are needed (e.g. just simple RAG). 

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

Hierarchical navigable small world (HNSW) is a popular ANN index - delivering good retrieval and QPS performance but at the cost of longer index build times and using more RAM for it (did crash for 1 million vectors on my 16GB RAM machine).

### IvfFlat

InVerted File Flat (IvfFlat) is less RAM hungry than HNSW. With caveats: IvfFlat performance degrades as vectors are deleted and added (because centroids are not updated), thus needing regular index rebuilds (if new vectors get inserted regularly).

"As data gets inserted or deleted from the index, if the index is not rebuilt, the IVFFlat index in pgvector can return incorrect approximate nearest neighbors due to clustering centroids no longer fitting the data well"

## [PGVectorScale](https://github.com/timescale/pgvectorscale)

### DiskANN

A promising ANN index that uses RAM and disk (needs fast disk - SSD / NVMe) to scale to billions of vectors, promising low RAM usage while still providing decent QPS. The problem is that the pgvectorscale index building implementation is [single core right now](https://github.com/timescale/pgvectorscale/issues/38) - leading to very long index generation times.

PGVectorScale supports pre-filtering using bitfields with manual meta-data table setup (complicated / bad dev ux).

## [VectorChord](https://github.com/tensorchord/VectorChord)

### [VChordRQ](https://docs.vectorchord.ai/vectorchord/usage/indexing.html)

A custom ANN index with superior performance (combining IVF ANN index with RaBitQ quantization). Supports pre-filtering (easy to use).

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

For 450K text embeddings 1024D float32 vectors - measure the recall@100.

| Method | Query Latency (ms) | Retrieval Recall | Speedup vs Baseline | Index Build Time (s) | Index Size (MB) |
|--------|-------------------|---------------------|---------------------|---------------------|-----------------|
| Baseline (Brute Force) | 1400.93 | 100.00% | 1.00x | - | - |
| **VectorChord (vchordrq)** | 468.64 | 100.00% | 2.99x | 1383.62 | 2229 |
| **pgvectorscale (DiskANN)** | 6.39 | 2.00% | 219.22x | 550.29 | 254 |
| **pgvector (HNSW)** | 611.54 | 100.00% | 2.29x | 1235.13 | 3555 |
| **pgvector (IVFFlat)** | 411.62 | 100.00% | 3.40x | 968.53 | 3561 |

**Note:** At 450K vectors, all approximate indices show strong speedups. HNSW, IVFFlat, and VectorChord achieve ~100% recall with 2-3.5x speedups. DiskANN has the fastest build time and best speed up (200x) but with significantly lower recall (2%).

## Optimizing Vector & Index Storage

When things scale up one should strive for efficient vector storage using:
- Vector Dimensionality Reduction
    - Matryoshka Embeddings: superior performance vs PQ as it learns the reduction in training and not post-training
    - Product Quantization (PQ)
    - PCA / UMAP
- Scalar Quantization (Reduce bit representation)
    - FP16: half precision from FP32 
    - binary: reduce to single bit
      - In pgvector: binary quantization will reduce any positive value to 1, and any zero or negative value to 0
      - Mean-based thresholding: binarize each dimension using its corpus-wide mean as the threshold, ensuring ~50/50 bit distribution per dimension for better information preservation and more discriminative binary codes.

We can see that using IvfFlat index on binary representation with a top-K factor of 10x (overfetching), then reranking in higher precision (float32 or float16) results excellent recall, low vector storage costs (float16) and very fast retrieval latencies.

A good trade-off seems to be using the first 512D (Matryoshka dims.), storing them as float16 and using ivf+binary(L500,P93,10x) delivering 13.59 ms retrieval latency with a 93% recall. Float16 for 512D vector storage is 322.3MB (4x reduction compared to 1024D float32) and index size is only 26.4MB (~33x reduction compared to vchordrq 1024D float16 index).

The table below was computed on an 8 core Intel server using 300K CLIP Matryoshka text embeddings.

| Dim   | Storage  | Index                               | Latency (ms)   | Recall  (%) | Build   (s)  | Storage (MB)   | Index  (MB)    |
|-------|----------|----------------------------------|-------|-------|--------|-------|--------|
| 256   | float32  | vchordrq                            |       0.77 |     36.0 |    30.55 |      322.3 |      379.1 |
| 256   | float16  | vchordrq                            |       0.65 |     49.0 |    18.78 |      161.1 |      200.5 |
| 256   | float32  | ivfflat(L500,P31)                   |      48.94 |     88.0 |     8.59 |      322.3 |      337.1 |
| 256   | float16  | ivfflat(L500,P31)                   |      50.60 |     88.0 |    10.14 |      161.1 |      158.4 |
| 256   | float32  | hnsw+binary(ef1000,10x)             |       9.25 |     83.0 |   228.73 |      322.3 |      156.2 |
| 256   | float16  | hnsw+binary(ef1000,10x)             |       7.56 |     82.0 |   233.40 |      161.1 |      156.2 |
| 256   | float32  | ivf+binary(L500,P93,10x)            |      13.54 |     83.0 |     2.06 |      322.3 |       17.3 |
| 256   | float16  | ivf+binary(L500,P93,10x)            |      12.96 |     82.0 |     2.23 |      161.1 |       17.2 |
| 256   | float32  | exact-binary(10x)                   |     117.95 |     82.0 |     0.00 |      322.3 |        0.0 |
| 256   | float16  | exact-binary(10x)                   |      87.77 |     83.0 |     0.00 |      161.1 |        0.0 |
| 256   | float32  | exact-binary(1x)                    |      39.80 |     35.0 |     0.00 |      322.3 |        0.0 |
| 256   | float16  | exact-binary(1x)                    |      38.73 |     34.0 |     0.00 |      161.1 |        0.0 |
| 256   | float32  | exact                               |      49.61 |     88.0 |     0.00 |      322.3 |        0.0 |
| 256   | float16  | exact                               |      39.24 |     88.0 |     0.00 |      161.1 |        0.0 |
||
| 512   | float32  | vchordrq                            |      25.56 |     96.0 |    40.21 |      644.5 |      844.3 |
| 512   | float16  | vchordrq                            |      25.57 |     97.0 |    26.32 |      322.3 |      397.8 |
| 512   | float32  | ivfflat(L500,P31)                   |      29.54 |     96.0 |    17.79 |      644.5 |      783.9 |
| 512   | float16  | ivfflat(L500,P31)                   |      27.04 |     97.0 |    14.50 |      322.3 |      337.1 |
| 512   | float32  | hnsw+binary(ef1000,10x)             |      15.13 |     94.0 |   209.95 |      644.5 |      160.4 |
| 512   | float16  | hnsw+binary(ef1000,10x)             |       7.73 |     94.0 |   212.17 |      322.3 |      160.4 |
| 512   | float32  | ivf+binary(L500,P93,10x)            |      21.15 |     93.0 |     3.87 |      644.5 |       26.3 |
| 512   | float16  | ivf+binary(L500,P93,10x)            |      13.59 |     93.0 |     3.72 |      322.3 |       26.4 |
| 512   | float32  | exact-binary(10x)                   |      57.11 |     92.0 |     0.00 |      644.5 |        0.0 |
| 512   | float16  | exact-binary(10x)                   |     106.79 |     94.0 |     0.00 |      322.3 |        0.0 |
| 512   | float32  | exact-binary(1x)                    |      32.77 |     49.0 |     0.00 |      644.5 |        0.0 |
| 512   | float16  | exact-binary(1x)                    |      45.30 |     47.0 |     0.00 |      322.3 |        0.0 |
| 512   | float32  | exact                               |     324.18 |     96.0 |     0.00 |      644.5 |        0.0 |
| 512   | float16  | exact                               |      47.51 |     96.0 |     0.00 |      322.3 |        0.0 |
||
| 1024  | float32  | vchordrq                            |      29.76 |    100.0 |    68.99 |     1289.1 |     1465.4 |
| 1024  | float16  | vchordrq                            |      28.08 |    100.0 |    38.46 |      644.5 |      882.5 |
| 1024  | float32  | ivfflat(L500,P31)                   |      29.09 |    100.0 |    39.74 |     1289.1 |     2347.7 |
| 1024  | float16  | ivfflat(L500,P31)                   |      27.11 |    100.0 |    33.48 |      644.5 |      784.0 |
| 1024  | float32  | hnsw+binary(ef1000,10x)             |      12.63 |    100.0 |   201.75 |     1289.1 |      181.0 |
| 1024  | float16  | hnsw+binary(ef1000,10x)             |      13.32 |    100.0 |   200.80 |      644.5 |      181.0 |
| 1024  | float32  | ivf+binary(L500,P93,10x)            |      16.60 |    100.0 |     6.61 |     1289.1 |       44.9 |
| 1024  | float16  | ivf+binary(L500,P93,10x)            |      18.17 |    100.0 |     6.52 |      644.5 |       44.9 |
| 1024  | float32  | exact-binary(10x)                   |      59.38 |    100.0 |     0.00 |     1289.1 |        0.0 |
| 1024  | float16  | exact-binary(10x)                   |      62.07 |    100.0 |     0.00 |      644.5 |        0.0 |
| 1024  | float32  | exact-binary(1x)                    |      29.04 |     55.0 |     0.00 |     1289.1 |        0.0 |
| 1024  | float16  | exact-binary(1x)                    |      33.51 |     56.0 |     0.00 |      644.5 |        0.0 |
| 1024  | float32  | exact                               |     480.70 |    100.0 |     0.00 |     1289.1 |        0.0 |
| 1024  | float16  | exact                               |     337.50 |    100.0 |     0.00 |      644.5 |        0.0 |

## Show me the code

Check out the code [here](https://github.com/SeanPedersen/vector-db-benchmark/tree/main)

## Conclusion

VectorChord is a good choice - providing superior performance and better developer experience (pre-filtering and better default settings). The vchordrq index is for most use cases the ideal choice as it delivers great performance and handles data distribution drift better than diskann indices. Using an ANN index only starts to make sense for big numbers of vectors (over 1 million).

## References

- The gold standard ANN benchmark: <https://github.com/erikbern/ann-benchmarks>
- <https://docs.vectorchord.ai/vectorchord/usage/indexing.html>
- <https://blog.vectorchord.ai/vectorchord-store-400k-vectors-for-1-in-postgresql#heading-ivf-vs-hnsw>
- <https://blog.vectorchord.ai/3-billion-vectors-in-postgresql-to-protect-the-earth>
- <https://drscotthawley.github.io/blog/posts/2023-06-12-RVQ.html>
- <https://www.tigerdata.com/blog/nearest-neighbor-indexes-what-are-ivfflat-indexes-in-pgvector-and-how-do-they-work>
- <https://jkatz05.com/post/postgres/pgvector-scalar-binary-quantization/>
- <https://medium.com/nlp-experiment/product-quantization-d66fdb860047>
- <https://medium.com/@bavalpreetsinghh/pgvector-hnsw-vs-ivfflat-a-comprehensive-study-21ce0aaab931>

#programming #machine-learning
