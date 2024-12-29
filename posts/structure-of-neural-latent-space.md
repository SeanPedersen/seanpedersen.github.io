---
title: 'Structure of Neural Embeddings'
date: '2024-12-27'
---
A small collection of insights on the structure of embeddings (latent spaces) produced by deep neural networks.

**Manifold Hypothesis**: High-dimensional data sampled from natural (real-world) processes lies in a low-dimensional manifold.

- <https://colah.github.io/posts/2014-03-NN-Manifolds-Topology/>

**Hierarchical Organization**: Features organize hierarchically across layers - earlier layers capture low-level (small context) features while deeper layers represent increasingly abstract (large context) concepts.

- <https://colah.github.io/posts/2015-01-Visualizing-Representations/>

**Linear Hypothesis**: Neural networks represent features as linear directions in their activation space.

- <https://colah.github.io/posts/2014-07-NLP-RNNs-Representations/>
- <https://www.lesswrong.com/posts/tojtPCCRpKLSHBdpn/the-strong-feature-hypothesis-could-be-wrong>

**Superposition Hypothesis**: Neural nets represent more “independent” features than a layer has neurons (dimensions) by representing features as a linear combination of neurons (resulting in dense vectors).

- <https://transformer-circuits.pub/2022/toy_model/index.html>

**Universality Hypothesis**: Circuits reappear across different models for the same data.

- <https://www.lesswrong.com/posts/5CApLZiHGkt37nRQ2/an-111-the-circuits-hypotheses-for-deep-learning>
- <https://phillipi.github.io/prh/>

**Adversarial Vulnerability**: Small changes in input space can cause large shifts in embeddings and therefore also in predictions made from them, suggesting the learned manifolds have irregular geometric properties.

- <https://arxiv.org/abs/1412.6572>

**Neural Collapse**: After extensive training, class features in the final layer cluster tightly around their means, with the network's classification weights aligning with these mean directions. Within-class variation becomes minimal compared to between-class differences, effectively creating distinct, well-separated clusters for each class.

- <https://arxiv.org/abs/2008.08186>
