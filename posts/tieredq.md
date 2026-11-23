# TieredQ - Beating RaBitQ with Variance-Aware Bit Allocation

While building my own ANN index for [digger.so](https://digger.so/), I was evaluating existing vector quantization algorithms to reduce RAM usage and storage costs. After playing around with [TurboQuant](https://research.google/blog/turboquant-redefining-ai-efficiency-with-extreme-compression/) and [RaBitQ](https://github.com/VectorDB-NTU/RaBitQ-Library) on some text embeddings I had the following idea for a new vector quantization approach:

Why compress all dimensions of a vector uniformly (lowering all dimensions to the same number of bits)? My intuition was that using a custom amount of bits per dimension based on its importance (variance) could improve retrieval performance (with respect to compressed vector data size).

After some experiments my vector quantization technique TieredQ was born. In my latest six-dataset benchmark, it achieves higher recall than RaBitQ and Qdrant's TurboQuant implementation (TQ+) at matched storage and serving RAM budgets. At a Recall@10 target of 0.95, it uses about 36–52% less serialized storage on the OpenAI, Gemini and MS MARCO datasets than the smallest evaluated reference configurations reaching that target.

So a clear win recall compression ratio wise - but not always throughput QPS and index build time.

[TieredQ Github Repo](https://github.com/SeanPedersen/TieredQ)

## How it works

TieredQ core algorithm:
1. PCA
2. Variance-aware greedy bit allocation per dimension (principal component)
3. Scalar quantization per dimension with allocated bit budget

TieredQ first subtracts a learned centroid from each vector, rotates the residual with PCA, and then hands out bits greedily to each dimension (0-12 bits per dim based on variance), wherever each extra bit reduces the expected error most.

So TieredQ exploits the skewed eigenvalue spectrum found in most embeddings. Where the variance of embeddings is concentrated in the leading PCA directions rather than spread evenly.

## Benchmark

The [refreshed benchmark for issue #4](https://github.com/SeanPedersen/TieredQ/issues/4#issuecomment-6677173430) compares TieredQ against the official RaBitQ library and Qdrant TQ+ across six datasets. It measures flat compressed top-10 search against exact FP32 ground truth, without an ANN index or FP32 reranking. TieredQ uses its default flat fast index with fine tiers enabled, including their query-time rescoring costs.

| Dataset | Dimensions | Base vectors | Metric |
| --- | ---: | ---: | --- |
| SIFT1M | 128 | 1,000,000 | L2 |
| GloVe-200 | 200 | 1,000,000 | Cosine |
| OpenAI-1536 100K | 1,536 | 100,000 | Cosine |
| Gemini-768 | 768 | 99,000 | Cosine |
| OpenAI-1536 1M | 1,536 | 999,000 | Cosine |
| MS MARCO-1M | 1,024 | 1,000,000 | Cosine |

Each dataset uses 1,000 queries; MS MARCO uses real dev questions, while Gemini uses held-out document vectors. Recall is reported as the mean over three seeds. The reference configurations use 1, 2 and 4 nominal bits per dimension; comparisons account for actual storage or measured RAM rather than nominal bit widths alone.

[![Recall@10 versus serialized bits per original dimension for TieredQ, RaBitQ and Qdrant TQ+ across six datasets](/images/tieredq-recall-storage.png)](/images/tieredq-recall-storage.png)

The horizontal axis includes the complete serialized index: vector codes, metadata, padding and shared model state, amortized over the base vectors. Lower is smaller; higher Recall@10 is better. Error bars show standard deviation across seeds, and the dotted line marks 0.95 recall. Click the image to view it at full size.

**TieredQ achieved higher mean Recall@10 in all 108 resource-matched comparisons.** Disk, steady serving RAM and peak serving RAM were matched independently:

| Matched resource | Higher recall | Recall advantage |
| --- | ---: | ---: |
| Serialized storage | 36/36 | +1.42–34.37 percentage points |
| Steady serving RAM | 36/36 | +1.41–34.37 percentage points |
| Peak serving RAM | 36/36 | +1.01–27.46 percentage points |

At **Recall@10 ≥ 0.95**, the smallest evaluated TieredQ configurations used about **36–52% less serialized storage** than the smallest evaluated qualifying reference configurations on OpenAI, Gemini and MS MARCO. TieredQ also reached 0.95 on SIFT and GloVe, where neither reference reached it within the evaluated 1/2/4-bit configurations. These are savings at a common recall threshold: reference recall can overshoot 0.95. They do not establish a universal 40% saving in RAM or disk usage.

Speed depends on the comparison. At equal recall, **eight-thread, batch-32 throughput was 0.76–3.36× RaBitQ and 3.32–13.36× Qdrant TQ+**, using each reference's fastest evaluated budget reaching the target. At equal 4-bit steady RAM budgets, RaBitQ was faster on all six datasets, while TieredQ delivered higher recall. Build speed was also mixed: RaBitQ's 1-bit builds were especially fast; TieredQ was competitive at higher budgets.

The benchmark ran on an Intel i9-7980XE with 64 GiB RAM, using library revision [`c74b4e3`](https://github.com/SeanPedersen/TieredQ/commit/c74b4e354b92d848a004e0f50199c3f7db7e0ca4). Timing covers query preparation, scanning, fine-tier rescoring and top-k, with matching thread/batch settings and three repeats. Full recall, storage, serving RAM, build cost, QPS and p50/p95 latency measurements are in the [benchmark report](https://github.com/SeanPedersen/TieredQ/blob/main/docs/benchmark-report.md), with raw measurements and provenance in the [results bundle](https://github.com/SeanPedersen/TieredQ/tree/main/benchmark/results/issue4-current-20261105).

## Limits
- Training and drift: PCA requires representative training data; its allocation can become less suitable as the distribution changes.
- Execution cost: initial compression is more costly; variable bit widths complicate packing and fast distance kernels.
- Objective mismatch: minimizing reconstruction MSE does not always maximize nearest-neighbor recall.
- Dataset dependence: a flat PCA spectrum offers less opportunity for variance-aware allocation.

## Previous Work: Transform Coding for ANN

After building TieredQ, I looked into whether variance-aware bit allocation had been explored before and found the following earlier related work.

### Transform Coding

TieredQ belongs to a family of compression techniques called **transform coding**, which has been around for decades. The basic idea is straightforward: transform the data into a representation where information is concentrated in fewer dimensions, then allocate more bits to the dimensions that matter most.

This is closely related to techniques used in JPEG, audio compression, and other signal-processing applications. PCA (also known as the Karhunen–Loève Transform in this context) is a natural choice because it decorrelates dimensions and orders them by variance.

For approximately Gaussian data, allocating bits according to the variance of each transformed dimension is well motivated by classical rate–distortion theory.

### Transform Coding for Nearest Neighbor Search

**Brandt (2010): [Transform Coding for Fast Approximate Nearest Neighbor Search in High Dimensions](https://research.adobe.com/publication/transform-coding-for-fast-approximate-nearest-neighbor-search-in-high-dimensions/)**

Brandt proposed using PCA together with non-uniform bit allocation and scalar quantization to produce compact vector representations for nearest neighbor search.

This is conceptually very close to TieredQ: rather than spending the same number of bits on every dimension, the algorithm allocates more bits to components carrying more information.

**Park et al. (2014): [Optimized Transform Coding for Approximate KNN Search](https://bmva-archive.org.uk/bmvc/2014/papers/paper001/index.html)**

This follow-up work improved transform coding by identifying a weakness in variance-based bit allocation: dimensions with similar variance don't necessarily have the same quantization error.

Instead of relying solely on a theoretical variance model, Optimized Transform Coding (OTC) uses actual quantization distortion to make better allocation decisions. The underlying optimization idea of TieredQ is therefore not new either.

### SAQ: Code Adjustment and Dimension Segmentation

**Li et al. (2025): [SAQ: Pushing the Limits of Vector Quantization through Code Adjustment and Dimension Segmentation](https://arxiv.org/abs/2509.12086)**

SAQ (accepted at SIGMOD) is the closest modern relative of TieredQ, and it also targets RaBitQ. It rests on the same observation: after PCA, a few leading dimensions hold most of the variance, so they should get more bits. The authors report up to 80% lower quantization error and over 80x faster encoding than Extended RaBitQ.

What it shares with TieredQ:
- PCA rotation to concentrate variance in the leading dimensions.
- A total bit quota per vector, spent unevenly: more bits for high-variance dimensions, fewer (or none) for the tail.
- Per-dimension scalar quantization of the PCA-projected values.
- The same weak spot: with a flat eigenvalue spectrum there is little to gain. In that case SAQ falls back to one segment, which matches its uniform base method (CAQ).

Where it differs:
- **Granularity:** SAQ groups neighboring dimensions into segments, and every dimension in a segment gets the same bits. Segment sizes are multiples of 64 to fit cache lines and SIMD. TieredQ picks bits per dimension (0-12), so its allocation is finer. The SAQ authors note that this coarse step hurts on low-dimensional data like DEEP.
- **Allocation search:** SAQ models the error of a plan (relative error shrinks by about half per extra bit) and finds the best plan with dynamic programming in O(D² · quota), under one second on their datasets. TieredQ uses a greedy search: give the next bit to the dimension where it cuts the expected error most.
- **Encoding:** SAQ quantizes each segment after a random orthonormal rotation (as RaBitQ does), then refines the codes with a coordinate-descent-like step ("code adjustment"). This keeps RaBitQ's accuracy while cutting encoding cost from O(2^B · D log D) to O(D) per vector. TieredQ rounds each PCA coordinate directly and has no refinement step.
- **Search-time tricks:** SAQ has a multi-stage distance estimator. It scores the high-variance segments first and prunes candidates early using distance bounds. It also ships SIMD kernels. TieredQ's query path is simpler (fast flat scan plus fine-tier rescoring).
- **Evaluation:** SAQ measures quantization error and graph-based ANN throughput against LVQ, PCA, PQ-style methods and Extended RaBitQ. My main benchmark is a flat scan against RaBitQ and Qdrant TQ+, so I ran a separate head-to-head with SAQ (see below).

#### TieredQ vs SAQ

In [issue #60](https://github.com/SeanPedersen/TieredQ/issues/60) I built IVF indexes with both on Arxiv-nomic-768 (1.34M rows, 2,048 lists, 32 threads, rows already in RAM):

| bits (SAQ / TieredQ) | SAQ build | TieredQ build | recall@100 at nprobe 256 (SAQ / TieredQ) |
| --- | ---: | ---: | --- |
| 1 / 1.25 | 44.7 s | 17.7 s | 0.849 / 0.912 |
| 2 / 2 | 42.2 s | 19.5 s | 0.951 / 0.951 |
| 2 / 2.5 | 42.2 s | 21.5 s | 0.951 / 0.969 |
| 3 / 3 | 42.1 s | 22.8 s | 0.976 / 0.976 |
| 4 / 4 | 47.0 s | 27.0 s | 0.985 / 0.983 |

TieredQ builds 1.7-2.5x faster. Assignment costs the same (about 3.7 s). The gap is encoding: SAQ's construct step takes 29-34 s, longer than TieredQ's whole build at every bit width. At equal bits, recall ties at 2 and 3 bits and SAQ is 0.002 ahead at 4 bits. At the lower budgets TieredQ reaches more recall with less RAM than SAQ's width (rows 1/1.25 and 2/2.5). This is one dataset and one index type, so it shows a trend, not a general ranking.

### How TieredQ Differs from RaBitQ & TurboQuant

The biggest difference from **RaBitQ and TurboQuant** is that TieredQ explicitly learns where the dataset's information is concentrated and distributes its bit budget accordingly.

RaBitQ relies on randomized rotation and binary quantization, while TurboQuant uses randomized rotation and scalar quantization together with techniques for controlling inner-product estimation error. Both emphasize theoretical guarantees and efficient, broadly applicable quantization.

TieredQ instead makes a different tradeoff: **spend more effort learning the structure of the dataset to make better use of every stored bit.**

Compared with earlier transform coding methods, TieredQ is less a fundamentally new algorithm and more a modern optimized implementation of an established idea, evaluated against a new generation of ANN quantizers and embedding distributions.

## Conclusion

Modern embeddings can have highly non-uniform variance distributions. That creates an opportunity for techniques that allocate their precision unevenly instead of treating all dimensions equally.

**On these six datasets, TieredQ achieves higher recall at matched storage and serving RAM budgets than RaBitQ and Qdrant TQ+.** At the 0.95 recall target, its serialized storage savings are about 36–52% on OpenAI, Gemini and MS MARCO, relative to the smallest evaluated qualifying reference configurations.

That doesn't mean TieredQ is universally better. PCA requires dataset-specific training and the learned transform introduces additional costs. So TieredQ is a good choice in RAM limited use cases.

## References
- https://github.com/SeanPedersen/TieredQ
- [TurboQuant: Online Vector Quantization with Near-optimal Distortion Rate](https://arxiv.org/abs/2504.19874)
  - https://research.google/blog/turboquant-redefining-ai-efficiency-with-extreme-compression/
  - [Qdrant TurboQuant ref](https://github.com/IvanPleshkov/turboquant-qdrant-showcase) and Qdrant prod ref
  - https://ivanpleshkov.dev/blog/turboquant/
- [RaBitQ: Quantizing High-Dimensional Vectors with a Theoretical Error Bound for Approximate Nearest Neighbor Search](https://arxiv.org/abs/2405.12497)
- https://github.com/VectorDB-NTU/RaBitQ-Library
- https://github.com/RyanCodrai/turbovec
- [Some RaBitQ vs TurboQuant drama](https://dev.to/gaoj0017/turboquant-and-rabitq-what-the-public-story-gets-wrong-1i00)
- https://github.com/VectorDB-NTU/rabitq-turboquant-comparison/
- [SAQ: Pushing the Limits of Vector Quantization through Code Adjustment and Dimension Segmentation](https://arxiv.org/abs/2509.12086) ([code](https://github.com/howarlii/saq/))
- [Quantization Beyond Uniform Bit Allocation](https://arxiv.org/abs/2608.19388) (found this very similar ref while writing this post up at 21. Oct 2026)

#ML #coding #math
