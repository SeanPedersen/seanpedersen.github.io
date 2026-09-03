# Deep Learning: Training Dynamics

**Double descent**: Making the model bigger can first improve, then hurt, and then surprisingly improve generalization again.

**Grokking**: Training longer can cause a model that has memorized the data (overfit to training data) to suddenly discover the underlying rule (generalize to test data).

The interesting open question is whether grokking is a special phenomenon caused by certain controlled datasets and training regimes, or whether it is a more general form of slow representation restructuring that also occurs in large real-world models but is harder to detect. Right now grokking seems to happen more reliably in language than in vision models.

Grokking is inherently connected to representation learning (how do we represent knowledge) and thus of utter importance to understand to arrive at learning algorithms that generalize faster and broader.

Ideally, we would like algorithms that can:
- detect when a representation is still becoming more abstract or compositional;
- distinguish memorization progress from representation progress;
- accelerate the transition toward useful abstractions;
- preserve useful structure while preventing shortcut learning;
- transfer representations across tasks before full supervised convergence;
- choose stopping criteria based on representation quality rather than only loss or validation accuracy.

Curriculum learning is also interesting in the context of grokking. Do not learn from all samples at once (randomly) but start with simple ones (low complexity) and iteratively increase complexity - hoping to learn compositional abstract representations faster.

## References
- [Nakkiran et al. (2019), “Deep Double Descent: Where Bigger Models and More Data Hurt”](https://arxiv.org/abs/1912.02292)
- [Hastie et al. (2019), “Surprises in High-Dimensional Ridgeless Least Squares Interpolation”](https://arxiv.org/abs/1903.08560)
- [Power et al. (2022), “Grokking: Generalization Beyond Overfitting on Small Algorithmic Datasets”](https://arxiv.org/abs/2201.02177)
- [Liu, Michaud & Tegmark (2022), “Omnigrok: Grokking Beyond Algorithmic Data”](https://arxiv.org/abs/2210.01117)
- [Unifying Grokking and Double Descent](https://arxiv.org/abs/2303.06173)
- [YouTube: The most complex model we actually understand](https://www.youtube.com/watch?v=D8GOeCFFby4)
- [YouTube: What the Books Get Wrong about AI - Double Descent](https://www.youtube.com/watch?v=z64a7USuGX0)

#ML
