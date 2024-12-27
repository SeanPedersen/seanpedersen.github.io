---
title: 'On the Structure of Neural Embeddings'
date: '2024-12-27'
---
A small collection of insights on the structure of embeddings (latent spaces) produced by deep neural networks.

**Manifold Hypothesis**: High-dimensional data sampled from natural (real-world) processes lies in a low-dimensional manifold.

**Hierarchical Organization**: Features organize hierarchically across layers - earlier layers capture low-level (small context) features while deeper layers represent increasingly abstract (large context) concepts.

**Linear Hypothesis**: Neural networks represent features as linear directions in their activation space.

**Superposition Hypothesis**: Neural nets represent more “independent” features than a layer has neurons (dimensions) by representing features as a linear combination of neurons.

**Universality Hypothesis**: Circuits reappear across different models for the same data.

**Neural Collapse**: After extensive training, do last-layer features for each class converge to their class means. The classifier's weights align with these class means, within-class variation becomes negligible compared to between-class variation.

**Adversarial Vulnerability**: Small changes in input space can cause large shifts in embeddings and therefore also in predictions made from them, suggesting the learned manifolds have irregular geometric properties.
