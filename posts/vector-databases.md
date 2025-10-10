---
title: 'Vector Databases - a benchmark'
date: '2025-10-08'
---
There is a flood of vector databases - which ones are actually useful? IMO extending a relational DBMS with ACID compliance and existing datasets, is for most use cases the ideal choice. Using a dedicated vector DB like (Chroma, Turbopuffer, [LanceDB](https://github.com/lancedb/lancedb) etc.) only makes sense for narrow use cases where no complicated meta-data filters are needed (e.g. just simple RAG). 

So let's have a look how we can store and search vectors using Postgres: There are three extensions for Postgres: pgvector, pgvectorscale and vectorchord.

## Benchmark

The benchmark should reflect realistic usage - right now it just measures index built and query times.

In the future I want to extend it:
- measure insertion perormance after inital index is built
- use real data embedding vectors -> simulate data distribution shift
- simulate realistic complex SQL queries involving categorical and range filtering
- benchmark vector scales: 100K, 1M, 10M, 100M, 1B

### [PGVector](https://github.com/pgvector/pgvector)

Standard extension to store vectors with common medium scale ANN indices (HNSW & IvfFlat).

#### [HNSW](https://github.com/nmslib/hnswlib)

The most popular ANN index - delivering good retrieval and QPS performance but using lot of RAM for it (did crash for 1 million vectors on my 16GB RAM machine).

#### IvfFlat

Less RAM hungry than HNSW. With caveats: ivfflat performance degrades as vectors are deleted and added.

"As data gets inserted or deleted from the index, if the index is not rebuilt, the IVFFlat index in pgvector can return incorrect approximate nearest neighbors due to clustering centroids no longer fitting the data well"

### [PGVectorScale](https://github.com/timescale/pgvectorscale)

Extends PGVector with [diskann](https://github.com/microsoft/DiskANN) index which is designed to scale to billions of vectors - by smartly using RAM with fast disk storage (SSD / NVME).

#### DiskANN

A promising ANN index that uses disk (needs SSD), promising low RAM usage while still providing decent QPS. The problem is that the pgvectorscale index building implementation is [single core right now](https://github.com/timescale/pgvectorscale/issues/38) - leading to very long index generation times.

PGVectorScale supports pre-filtering using bitfields with manual table setup (complicated / bad dev ux).

### [VectorChord](https://github.com/tensorchord/VectorChord)

#### vchordrq

A custom ANN index with superior performance (combining IVF ANN index with RaBitQ quantization). Supports pre-filtering (easy to use).

#### vchordg (DiskANN)

A novel addition (not prodution ready yet): custom implementation of DiskANN index combined with RaBitQ quantization.

- <https://blog.vectorchord.ai/vectorchord-05-new-rabitq-empowered-diskann-index-and-continuous-recall-measurement?source=more_articles_bottom_blogs>

## ANN Benchmark Results

For 450K text embeddings 1024D float32 vectors - measure the recall@100.

| Method | Query Latency (ms) | Retrieval Recall | Speedup vs Baseline | Index Build Time (s) | Index Size (MB) |
|--------|-------------------|---------------------|---------------------|---------------------|-----------------|
| Baseline (Brute Force) | 1400.93 | 100.00% | 1.00x | - | - |
| **VectorChord (vchordrq)** | 468.64 | 100.00% | 2.99x | 1383.62 | 2229 |
| **pgvectorscale (DiskANN)** | 6.39 | 2.00% | 219.22x | 550.29 | 254 |
| **pgvector (HNSW)** | 611.54 | 100.00% | 2.29x | 1235.13 | 3555 |
| **pgvector (IVFFlat)** | 411.62 | 100.00% | 3.40x | 968.53 | 3561 |

**Note:** At 450K vectors, all approximate indices show strong speedups. HNSW, IVFFlat, and VectorChord achieve ~100% precision with 2-3.5x speedups. DiskANN has the fastest build time and best speed up (200x) but with significantly lower precision (2%).

### Show me the code

Check out the code [here](https://github.com/SeanPedersen/vector-db-benchmark/tree/main)

## Conclusion

VectorChord is the clear winner - providing superior performance and better developer experience (pre-filtering and better defaul settings). The vchordrq index is for most use cases the ideal choice as it delivers great performance and handles data distribution drift better than diskann indices. Using an ANN index only starts to make sense for huge numbers of vectors (over 10 million).

## References

- <https://docs.vectorchord.ai/vectorchord/usage/indexing.html>
- <https://blog.vectorchord.ai/vectorchord-store-400k-vectors-for-1-in-postgresql#heading-ivf-vs-hnsw>
- <https://drscotthawley.github.io/blog/posts/2023-06-12-RVQ.html>
- <https://deepwiki.com/timescale/pgvectorscale/1-overview>
- <https://www.tigerdata.com/blog/nearest-neighbor-indexes-what-are-ivfflat-indexes-in-pgvector-and-how-do-they-work>

#programming #machine-learning
