---
date: '2026-07-22'
---
# Approximate Vector Index Benchmark on arXiv Embeddings

The fastest high-recall index in this benchmark answered a query in 1.15 ms at 0.964 recall@10. A 13.4 MiB TurboVec index reached exact recall@10 after reranking 100 candidates with FP16 embeddings. That took 2.53 ms at the median on 100,000 vectors.

I benchmarked five implementations of [approximate nearest neighbor search](https://en.wikipedia.org/wiki/Nearest_neighbor_search) against 100,000 256-dimensional, unit-normalized paper embeddings from [arXiv](https://en.wikipedia.org/wiki/ArXiv) metadata. The test used 50 deterministic blended-vector queries, exact [cosine similarity](https://en.wikipedia.org/wiki/Cosine_similarity) ground truth, [k-nearest neighbors](https://en.wikipedia.org/wiki/K-nearest_neighbors_algorithm) with `k=10`, one warmup, and three measured passes. Reported latency does not include query embedding or metadata lookup.

## Results

| Algorithm | Build | Recall@10 | Search p50 | Search p95 | Index | Max RAM |
|---|---:|---:|---:|---:|---:|---:|
| zvec HNSW + RaBitQ | 72.3 s | 0.964 | 1.15 ms | 1.39 ms | 148.9 MiB | 617.1 MiB |
| zvec DiskANN + PQ | 77.5 s | 0.912 | 44.71 ms | 115.27 ms | 138.4 MiB | 1.10 GiB |
| TurboVec, 4-bit | 0.6 s | 0.880 | 2.37 ms | 3.09 ms | 13.4 MiB | 249.0 MiB |
| TurboVec, 4-bit + FP32 rerank, 100 candidates | 0.6 s | 1.000 | 2.69 ms | 3.10 ms | 13.4 MiB | 250.0 MiB |
| TurboVec, 4-bit + FP16 rerank, 100 candidates | 0.6 s | 0.996 | 2.53 ms | 2.70 ms | 13.4 MiB | 249.8 MiB |
| TurboVec, 4-bit + INT8 rerank, 100 candidates | 0.5 s | 0.958 | 2.50 ms | 3.94 ms | 13.4 MiB | 249.8 MiB |
| AsterVec | 36.9 s | 0.854 | 1.26 ms | 3.86 ms | 161.8 MiB | 299.2 MiB |
| aisaq DiskANN | 308.7 s | 0.978 | 20.58 ms | 28.50 ms | 164.8 MiB | 709.9 MiB |

The TurboVec rerank results were measured on 22 July 2026 with the same 100K corpus and protocol. Each fetches 100 candidates before scoring them with FP32, FP16, or INT8 vectors. The latency includes that work. The FP16 and INT8 vector files require 48.8 MiB and 24.4 MiB for 100K vectors.

zvec [HNSW](https://en.wikipedia.org/wiki/Hierarchical_navigable_small_world) plus RaBitQ gave the best speed and near-best recall. It is the natural default here when the index can stay in memory and the application does not need a very small artifact.

aisaq DiskANN found the most exact neighbors, at 0.978 recall@10. That result cost more than five minutes of build time and about 20 ms per median search. It is a reasonable choice when recall matters more than interactive latency.

TurboVec is the storage and build-time outlier. Its 4-bit index occupied 13.4 MiB, compared with roughly 138 to 165 MiB for the other valid indexes. It also built in well under a second. Its native recall was 0.880. FP16 reranking nearly recovered the full top ten, while FP32 reranking did so exactly.

AsterVec matched HNSW's median latency surprisingly closely, while using less peak RAM during the run. Its recall was the lowest in this group, at 0.854.

The zvec DiskANN plus product quantization configuration did not win a category in this run. Its p95 was especially variable at 115.27 ms, and it used the most memory. That does not make DiskANN a poor design. It means this configuration and this 100K workload were not its best fit.

## TurboVec reranking on the full corpus

I repeated the TurboVec comparison on all 3,094,252 embeddings. Each run used one untimed warmup query, then 50 queries over three measured passes. TurboVec searched the 4-bit index in every case. The reranked versions fetched 100 candidate IDs and scored only those vectors against a memory-mapped file.

| Algorithm | Recall@10 | Search p50 | Search p95 | Index | Rerank vectors |
|---|---:|---:|---:|---:|---:|
| TurboVec, 4-bit | 0.860 | 82.57 ms | 87.44 ms | 413.1 MiB | None |
| TurboVec, 4-bit + FP32 rerank, 100 candidates | 1.000 | 84.31 ms | 89.49 ms | 413.1 MiB | 2.95 GiB |
| TurboVec, 4-bit + FP16 rerank, 100 candidates | 1.000 | 84.12 ms | 89.88 ms | 413.1 MiB | 1.48 GiB |
| TurboVec, 4-bit + INT8 rerank, 100 candidates | 0.932 | 84.88 ms | 90.79 ms | 413.1 MiB | 0.74 GiB |

FP16 preserved exact recall while using half the FP32 rerank-vector storage. INT8 used one quarter of that storage, but recall fell to 0.932. The FP32, FP16, and INT8 files are 2.95 GiB, 1.48 GiB, and 0.74 GiB for the full corpus. All modes had a 3.108 GiB search peak RSS. Memory mapping kept process RSS stable because each query reads only 100 candidate vectors.

## Filtering changes the ranking

Vector search systems often need a metadata condition as well as semantic similarity. I ran a second test for the query `deep learning`, restricted to the 30,172 papers first published in June 2026. Ground truth was the exact top ten inside that filtered set. Each row below is the median of ten searches.

| Algorithm | Filter mode | Recall@10 | Search p50 |
|---|---|---:|---:|
| TurboVec | Native ID allowlist | 0.900 | 9.49 ms |
| TurboVec + FP16 rerank, 100 candidates | Native ID allowlist + FP16 rerank | 1.000 | 9.65 ms |
| AsterVec | Native payload filter | 0.700 | 160.61 ms |
| zvec DiskANN | Date-partitioned index | 1.000 | 43.51 ms |
| aisaq DiskANN | Post-filter, 1,000 candidates | 0.100 | 56.13 ms |

The most important result is not the winner. It is the gap between native or partition-aware filtering and post-filtering. aisaq retrieved 1,000 unfiltered candidates and then applied the date condition. Only one of the true top ten survived. zvec DiskANN searched a date-specific partition instead and recovered all ten.

TurboVec's native allowlist was the best fast filtered option in this test. Reranking 100 allowlisted candidates with FP16 vectors raised recall from 0.900 to 1.000 for 0.16 ms at the median. AsterVec's native payload filter kept a useful 0.700 recall, but its 160.61 ms latency was much higher than its unfiltered result.

## What I excluded and corrected

I excluded zvec IVF from the comparison. In zvec 0.5.1, changing `nprobe` from 1 through `n_list` produced identical IDs and latency, as though the search always visited one list. A one-list INT8 control reached 0.982 recall, so the issue is not the exact ground truth or the INT8 inputs. There is a standalone [upstream bug report](https://github.com/alibaba/zvec/issues/598) with a reproduction. I will reconsider IVF after that issue is fixed.

I also corrected the aisaq setup before reporting the table. The original adapter combined a weakly tested maximum-inner-product transformation with 32-byte compression for vectors on SSD. Since these embeddings are unit-normalized, L2 ranking gives the same ordering. I used that equivalent ranking, left SSD vectors uncompressed, set search complexity to 200, and used a beam width of 4. Recall rose from 0.370 to 0.978.

## Reproduce it

The benchmark is Python 3.11 and writes incremental machine-readable output, so an interrupted run keeps completed algorithms. Install dependencies with `uv sync --all-extras`, then run:

```bash
uv run ann-benchmark --limit 100000 --cleanup
```

Run the TurboVec rerank configuration:

```bash
uv run ann-benchmark --algorithms turbovec-rerank-100 --cleanup
```

Prepare FP16 or INT8 rerank vectors, then run the matching configuration:

```bash
uv run python prepare_rerank_embeddings.py --precision fp16
uv run ann-benchmark --algorithms turbovec-rerank-fp16-100 --cleanup
```

The full corpus has 3,094,252 vectors. Run it sequentially with `uv run ann-benchmark --cleanup` if there is enough time and disk space. The 100K measurements here should not be read as full-corpus conclusions.

The `results/latest.json`, `results/filtered-query.json`,
`results/full-turbovec-warm.json`, and `results/full-turbovec-rerank-100.json`
files in the benchmark checkout contain the raw measurements behind these tables.
Quantized rerank results are in `results/turbovec-quantized-rerank-smoke-warm.json`
and `results/full-turbovec-quantized-rerank-warm.json`. The filtered FP16 rerank
result is in `results/filtered-query-rerank-warm.json`.

## Conclusion

Use TurboVec with native ID allowlists and FP16 reranking when the original embedding file is available. It kept the small 4-bit index, matched exact recall on the full corpus, and recovered the exact filtered top ten with almost no median latency cost. FP16 is the best balance here. It uses half the storage of FP32 rerank vectors without losing full-corpus recall in this test.

Choose zvec HNSW plus RaBitQ when sub-millisecond unfiltered search matters more than index size. Do not use post-filtering when recall inside the filtered set matters. The aisaq result shows why: a large unfiltered candidate set can still miss nearly all of the relevant filtered neighbors.

TODO:
- check how RAM usage of diskann based indexes scales when using full dataset (3 mill)
- check how diskann params can reduce RAM usage
- measure data distribution drift recall impact (insert and delete drifting vectors)

## References

1. [Benchmark Github: Approximate vector search benchmark source and results](https://github.com/SeanPedersen/approximate-vector-search-benchmark)
2. [HNSW research paper](https://arxiv.org/abs/1603.09320)
3. [DiskANN: Fast accurate billion-point nearest neighbor search on a single node](https://www.microsoft.com/en-us/research/publication/diskann-fast-accurate-billion-point-nearest-neighbor-search-on-a-single-node/)
4. [Faiss wiki](https://github.com/facebookresearch/faiss/wiki)
5. [arXiv bulk data access and metadata information](https://info.arxiv.org/help/bulk_data.html)
6. [zvec IVF nprobe regression report](https://github.com/alibaba/zvec/issues/598)
7. [zvec source repository](https://github.com/alibaba/zvec)
8. [TurboVec source repository](https://github.com/RyanCodrai/turbovec)
9. [AsterVec source repository](https://github.com/NTU-Siqiang-Group/AsterVec)
10. [AiSAQ-DiskANN source repository](https://github.com/kioxia-jp/aisaq-diskann)
11. [Microsoft DiskANN source repository](https://github.com/microsoft/DiskANN)

#coding #ML
