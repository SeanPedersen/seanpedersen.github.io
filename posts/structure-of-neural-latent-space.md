---
title: 'Structure of Neural Embeddings'
date: '2024-12-27'
---
A small collection of insights on the structure of embeddings (latent spaces) produced by deep neural networks. Embeddings are a collection of vectors (list of numbers) arranged in a meaningful way - representing relationships via distances between each other.

## General Principles

**Manifold Hypothesis**: High-dimensional data sampled from natural (real-world) processes lies in a low-dimensional manifold.

- <https://colah.github.io/posts/2014-03-NN-Manifolds-Topology/>

**Hierarchical Organization**: Features organize hierarchically across layers - earlier layers capture low-level (small context) features while deeper layers represent increasingly abstract (large context) concepts.

- <https://colah.github.io/posts/2015-01-Visualizing-Representations/>

**Linear Hypothesis**: Neural networks represent features as linear directions in their activation space.

- <https://colah.github.io/posts/2014-07-NLP-RNNs-Representations/>
- <https://www.lesswrong.com/posts/tojtPCCRpKLSHBdpn/the-strong-feature-hypothesis-could-be-wrong>
- [The Geometry of Categorical and Hierarchical Concepts in Large Language Models](https://arxiv.org/abs/2406.01506)

**Superposition Hypothesis**: Neural nets represent more “independent” features than a layer has neurons (dimensions) by representing features as a linear combination of neurons (resulting in dense vectors).

- <https://transformer-circuits.pub/2022/toy_model/index.html>

**Entangled Representation Hypothesis**: Gradient descent optimised neural networks tend to develop redundant and fractured features instead of unified, modular representations that can be reused and controlled separately.

- [The Fractured Entangled Representation Hypothesis](https://arxiv.org/abs/2505.11581)
- <https://seanpedersen.github.io/posts/vae#beta-vae>

**Universality Hypothesis**: Neural circuits reappear across different models for the same data. Thus models trained on similar data sets (text, image, video etc.) share the same relative semantic structure in their embeddings (activations).

- <https://blog.jxmo.io/p/there-is-only-one-model>
- [Harnessing the Universal Geometry of Embeddings](https://arxiv.org/abs/2505.12540)
- [The Platonic Representation Hypothesis](https://arxiv.org/abs/2405.07987)
- <https://phillipi.github.io/prh/>

**Smoothness**: Formally known as Lipschitz continuity states that small changes in inputs cause small changes in output (latent) space. Well-trained, robust models should thus be insensitive to small input changes - which has its limits (see next point).

- [Some Fundamental Aspects about Lipschitz Continuity of Neural Network Functions](https://arxiv.org/abs/2302.10886v2)

**Adversarial Vulnerability**: Carefully crafted changes in input space can cause large shifts in embedding space and therefore also in predictions made from them, suggesting even networks trained for smoothness have some chaotic properties.

- [Explaining and Harnessing Adversarial Examples](https://arxiv.org/abs/1412.6572)
- [Subtle adversarial image manipulations influence both human and machine perception](https://www.nature.com/articles/s41467-023-40499-0)

**Neural Collapse**: After extensive training, class features in the final layer cluster tightly around their means, with the network's classification weights aligning with these mean directions. Within-class variation becomes minimal compared to between-class differences, effectively creating distinct, well-separated clusters for each class.

- [Prevalence of Neural Collapse during the terminal phase of deep learning training](https://arxiv.org/abs/2008.08186)

## Limits of Dense Embeddings

Most neural network architectures process data as dense vectors, making them hard to interpret for humans.

A more human interpretable embedding representation would be sparse (few dimensions are active) and spatially meaningful (position of dimensions encodes information).

This would make them easier to interpret for humans and potentially offer some other benefits: encode single-concept objects vs multi-concept objects, encode novelty (outliers), increase robustness and reduce storage / increase efficiency. But this is another [blog post](https://seanpedersen.github.io/posts/sparse-distributed-representations).

## Problems with Contrastive Embeddings

**Modality Gap**: Multi-modal training strategies like CLIP that unify text and image in a shared embedding space, suffer from the Modality Gap, where text and image vectors form distinct clusters instead of occupying a shared cluster in the embedding space (relative similarities are working though). This separation can limit the effectiveness of cross-modal retrieval and transfer tasks.

- <https://jina.ai/news/the-what-and-why-of-text-image-modality-gap-in-clip-models/?nocache=1>
- <https://seanpedersen.github.io/posts/closing-clip-modality-gap>
- [Connect, Collapse, Corrupt: Learning Cross-Modal Tasks with Uni-Modal Data](https://arxiv.org/abs/2401.08567)

**Dimensional Collapse**: A phenomenon in contrastive learning where the learned representations tend to occupy a lower-dimensional subspace than intended, effectively "collapsing" along certain dimensions. This results in embeddings that don't fully utilize the available embedding dimensions, leading to highly correlated dimensions rather than capturing independent features.

- [Understanding Dimensional Collapse in Contrastive Self-supervised Learning](https://arxiv.org/abs/2110.09348)
- [Dimensional Collapse in Video Representation Learning](https://www.static.tu.berlin/fileadmin/www/10002219/132153_-_digitale_Abschlussarbeiten/Dimensional_Collapse_in_Video_Representation_Learning_Publication_Paul_Kapust.pdf)

#machine-learning
