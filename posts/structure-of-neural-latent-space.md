---
date: '2024-12-27'
icon: "/images/icons/deep-learning.svg"
---
# Structure of Neural Embeddings

A small collection of insights on the structure of embeddings (latent spaces) produced by deep neural networks.

Embeddings represent semantic relationships between objects (like words or images) as points in a vector space, where related items are positioned close together and unrelated ones far apart (relatedness is defined by the loss function).

## General Principles

**Manifold Hypothesis**: High-dimensional data sampled from natural (real-world) processes lies on or near a low-dimensional manifold. The "near" matters: noise, measurement artifacts, discrete structure and multiple disconnected regions all complicate the clean manifold picture. A manifold is a space that looks like flat (Euclidean) space when zooming in on any of its points, f.e. Earth's (a 3D object) surface looks locally like flat 2D space (a manifold).

- <https://colah.github.io/posts/2014-03-NN-Manifolds-Topology/>
- [What is a manifold?](https://www.quantamagazine.org/what-is-a-manifold-20251103/)

**Hierarchical Organization**: Many deep networks show hierarchical feature organization, especially vision models: earlier layers often capture local, low-level (small context) patterns while deeper layers tend to encode more task-relevant and abstract (large context) features. In transformers this hierarchy is less clean, because attention and residual streams mix information across layers and some high-level features can appear earlier than the simple "early = low-level, late = abstract" story suggests.

- <https://colah.github.io/posts/2015-01-Visualizing-Representations/>

**Linear Representation Hypothesis**: Many model-relevant features appear to be represented approximately as directions or low-dimensional subspaces in activation space, making them accessible to linear probes or activation steering. Some semantic operations then look like vector arithmetic (the classic queen ≈ king - man + woman comes from word embedding spaces). This is not guaranteed for all concepts, and linear accessibility does not by itself prove a feature is causally used by the model.

- <https://colah.github.io/posts/2014-07-NLP-RNNs-Representations/>
- <https://www.lesswrong.com/posts/tojtPCCRpKLSHBdpn/the-strong-feature-hypothesis-could-be-wrong>
- [The Geometry of Categorical and Hierarchical Concepts in Large Language Models](https://arxiv.org/abs/2406.01506)

**Superposition Hypothesis**: Neural nets represent more "independent" features than a layer has neurons (dimensions) by encoding features as directions that are not aligned with individual neuron axes. Neurons are just the coordinate basis, so one feature can spread across many neurons and one neuron can take part in many features (polysemanticity). This works because real features tend to activate sparsely, even if the underlying representation looks dense.

- <https://transformer-circuits.pub/2022/toy_model/index.html>

**Entangled Representation Hypothesis**: Gradient descent optimised deep neural networks tend to develop redundant and fractured features instead of unified, modular representations that can be reused and controlled separately - explaining their brittleness (like adversarial examples and hallucinations).

- [The Fractured Entangled Representation Hypothesis](https://arxiv.org/abs/2505.11581) argues for fractured, redundant and entangled internal representations, but its direct evidence comes from a minimal single-image-generation setup comparing SGD-trained networks with evolved ones, so it should be treated as a speculative hypothesis rather than a general result for all deep networks.
- [Beta-VAE](/posts/vae#beta-vae) is one route toward disentangled representations, but unsupervised disentanglement is not guaranteed: Locatello et al. show it is impossible without inductive biases on both model and data ([Challenging Common Assumptions in Unsupervised Disentanglement](https://proceedings.mlr.press/v97/locatello19a.html)).

**Universality Hypothesis (representational convergence)**: Neural circuits reappear across different models trained on similar data, and models trained on the same modality (text, images, video, etc.) often learn representation spaces with partially aligned semantic neighborhoods, even if their coordinate systems differ. There is real evidence of convergence, especially with scale and similar data/objectives. But stronger claims about identical distances, angles or a single "universal" manifold are rebutted: [Revisiting the Platonic Representation Hypothesis: An Aristotelian View](https://arxiv.org/abs/2602.14486) challenges the stronger global-convergence reading of [PRH](https://arxiv.org/abs/2405.07987), while preserving a weaker local-neighborhood version.

- <https://blog.jxmo.io/p/there-is-only-one-model>
- [Harnessing the Universal Geometry of Embeddings](https://arxiv.org/abs/2505.12540)
- [Revisiting the Platonic Representation Hypothesis: An Aristotelian View](https://arxiv.org/abs/2602.14486) -> disproves: [The Platonic Representation Hypothesis](https://arxiv.org/abs/2405.07987)
- <https://phillipi.github.io/prh/>

**Smoothness (Lipschitz continuity)**: Small changes in inputs cause proportionally bounded changes in output (latent) space - formally, $\|f(x_1) - f(x_2)\| \le L \|x_1 - x_2\|$ for some constant $L$. Robust models should ideally be locally stable to perturbations that preserve task-relevant semantics, while staying sensitive to small but meaningful changes (one pixel can flip a digit, one word can flip sentiment, a small molecular change can change function). A low global Lipschitz constant is not automatically desirable either, since it can reduce expressivity.

- [Some Fundamental Aspects about Lipschitz Continuity of Neural Network Functions](https://arxiv.org/abs/2302.10886v2)

**Adversarial Vulnerability**: Carefully crafted small changes in input space can cause large shifts in embedding space and therefore also in predictions. This shows models can be highly sensitive in certain high-dimensional directions, even when the perturbation is tiny under human-perceptual metrics. Goodfellow et al. argue this comes largely from linear behavior in high-dimensional spaces rather than from chaotic nonlinear dynamics.

- [Explaining and Harnessing Adversarial Examples](https://arxiv.org/abs/1412.6572)
- [Subtle adversarial image manipulations influence both human and machine perception](https://www.nature.com/articles/s41467-023-40499-0)

**Neural Collapse**: In supervised classification, during the terminal phase of training (often after training error is already near zero), penultimate / last-layer features for each class collapse toward their class mean. The class means and the classifier weights converge toward a symmetric simplex equiangular tight frame (ETF), and prediction starts to behave like nearest-class-center classification. Within-class variation becomes minimal compared to between-class differences, creating distinct, well-separated clusters for each class.

- [Prevalence of Neural Collapse during the terminal phase of deep learning training](https://arxiv.org/abs/2008.08186)

## Relation to Graphs

Any set of vectors can be viewed as a weighted graph by treating vectors as nodes and similarities/distances (f.e. cosine) as edge weights. Connecting every node to every other node gives a complete weighted graph. In practice one keeps only the K nearest neighbors, sometimes symmetrized, which preserves local neighborhood structure but loses some global metric information (and is directed unless symmetrized).

## Limits of Dense Embeddings

Most neural network architectures process data as dense vectors, making them hard to interpret for humans.

A more human interpretable embedding representation would be sparse (few dimensions are active) and spatially meaningful (position of dimensions encodes information).

This would make them easier to interpret for humans and potentially offer some other benefits: encode single-concept objects vs multi-concept objects, encode novelty (outliers), increase robustness and reduce storage / increase efficiency. But this is another [blog post](/posts/sparse-distributed-representations).

## Problems with Contrastive Embeddings

**Modality Gap**: Multi-modal training strategies like CLIP learn a shared comparison space, but text and image embeddings can still occupy separated regions of it while preserving cross-modal matching. This can be benign for standard image-text retrieval, but problematic for mixed-modality ranking, calibration and transfer (the gap can create intra-modal ranking bias and inter-modal fusion failures).

- <https://jina.ai/news/the-what-and-why-of-text-image-modality-gap-in-clip-models/>
- [Closing the CLIP Modality Gap](/posts/closing-clip-modality-gap)
- [Connect, Collapse, Corrupt: Learning Cross-Modal Tasks with Uni-Modal Data](https://arxiv.org/abs/2401.08567)
- [Closing the Modality Gap for Mixed Modality Search](https://arxiv.org/abs/2507.19054)

**Dimensional Collapse**: A phenomenon in self-supervised representation learning (known first in non-contrastive methods, and also occurring in contrastive setups) where the learned representations span a lower-dimensional subspace than the nominal embedding dimension, effectively "collapsing" along certain dimensions. This results in embeddings that don't fully use the available dimensions, leading to highly correlated dimensions rather than independent features.

- [Understanding Dimensional Collapse in Contrastive Self-supervised Learning](https://arxiv.org/abs/2110.09348)
- [Dimensional Collapse in Video Representation Learning](https://www.static.tu.berlin/fileadmin/www/10002219/132153_-_digitale_Abschlussarbeiten/Dimensional_Collapse_in_Video_Representation_Learning_Publication_Paul_Kapust.pdf)

## References

- [Hyperbolic embeddings](https://markkm.com/blog/embeddings-in-data-science/)

TODO:
- How do image (continuous input space) and text (discrete input space) embedding spaces differ (number of clusters, density, etc.)?
- How sparse are dense embeddings (how much information do they lose if sparsified -> compare different embeddings based on layer depth)?

#ML
