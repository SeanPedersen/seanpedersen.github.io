# ResNet - Skip Connections

Adding more layers to deep neural networks increases their representational capacity but also destabilizes learning. Skip connections unlocked training of very deep neural networks by adding identity inputs to layers (called skip connections).

The residual stream: instead of learning a completely new transformation for every layer, just learn the residual (updates to the existing input). So the representation is not repeatedly overwritten but instead progressively updated - stabilizing training of deep neural networks.

So instead of normal (where F is MLP / convolution + activation):
$$x_{l+1} = F(x_l)$$

the residual version is:
$$x_{l+1} = x_l + F(x_l)$$


**Do skip connections fix vanishing / exploding gradient problem?**
Skip connections change the optimization problem so that very deep networks have a much healthier gradient pathway. They don't make vanishing/exploding gradients mathematically impossible.

**Prior art:** Very deep networks were trained before ResNets. He et al. demonstrated extremely deep plain rectifier networks using careful initialization designed to preserve signal variance through the network. Schmidhuber et al. then introduced Highway Networks, which used learned gates to provide an identity-like information pathway through hundreds of layers (inspired by LSTM).

## References

- [Deep Residual Learning for Image Recognition](https://arxiv.org/abs/1512.03385)
- [Identity Mappings in Deep Residual Networks](https://arxiv.org/abs/1603.05027)
- [YouTube: The most cited paper of the century is a brilliant hack](https://www.youtube.com/watch?v=QgH9sr7G13Q)
- [Delving Deep into Rectifiers: Surpassing Human-Level Performance on ImageNet Classification](https://arxiv.org/abs/1502.01852)
- [Highway Networks](https://arxiv.org/abs/1505.00387)

#ML
