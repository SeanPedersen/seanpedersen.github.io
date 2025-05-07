---
title: 'Closing the CLIP modality gap'
date: '2025-04-29'
---
## What is the Modality Gap?

The modality gap refers to a phenomenon observed in multi-modal embedding models such as CLIP (Contrastive Language-Image Pre-training). These models learn joint representations of images and text, but often struggle with a systematic separation between the embedding spaces of different modalities. In simple terms, even when an image and text are perfectly matched in meaning, their vector representations may be distant in the joint embedding space.

This gap can significantly reduce model performance when using embeddings for cross-modal retrieval tasks, like finding images that match a text description or captioning images accurately.

## Why Does the Modality Gap Exist?

One reason for the gap is the creation of false negative samples during training.

Let's explore why this happens:

When training multi-modal models like CLIP, we need both positive examples (matching image-text pairs) and negative examples (unrelated image-text pairs). As researchers from Jina AI explain:

"We would ideally want to train with image-text pairs that we knew with certainty were related and unrelated, but there is no obvious way to get known unrelated pairs. It’s possible to ask people “Does this sentence describe this picture?” and expect consistent answers. It’s much harder to get consistent answers from asking “Does this sentence have nothing to do with this picture?”

Instead, we get unrelated image-text pairs by randomly selecting pictures and texts from our training data, expecting they will practically always be bad matches." - <https://jina.ai/news/the-what-and-why-of-text-image-modality-gap-in-clip-models>

Instead of manually labeling unrelated pairs, most approaches simply assume that randomly selected image-text pairs from the training data will be unrelated. This batch-based contrastive learning treats all non-matching pairs as equally dissimilar - a convenient but flawed assumption.

## The Problem with Traditional Contrastive Learning

In traditional contrastive learning for multi-modal models, given two related image-text pairs (I₁,T₁) and (I₂,T₂), we train the model with these distance objectives:

```
D(I₁,T₁) = 0       // Related pairs should have minimal distance
D(I₂,T₂) = 0
D(I₁,T₂) = ∞       // Unrelated pairs are assumed maximally distant
D(I₂,T₁) = ∞
```

However, this approach ignores the reality that some "unrelated" pairs may have semantic similarities. For example, an image of a cat (I₁) and text about dogs (T₂) are more related than an image of a cat and text about astrophysics.

## Proposed Solution: Leveraging Uni-Modal Similarities

Rather than treating all negative pairs as equally distant, we can use pre-trained uni-modal encoders to estimate the actual semantic similarity between items across modalities. Here's how this approach works:

Start with a dataset of paired images and texts, where we know (I₁,T₁) and (I₂,T₂) are related pairs.
Use pre-trained uni-modal image encoder m_i and text encoder m_t to calculate meaningful distances between unpaired items of the same modality.
Adjust the contrastive learning objective to reflect these more nuanced relationships.

This transforms our distance objectives to:
```
D(I₁,T₁) = 0       // Related pairs still have minimal distance
D(I₂,T₂) = 0
D(I₁,T₂) = D(I₂,T₁) = (CosSim(m_i(I₁),m_i(I₂)) + CosSim(m_t(T₁),m_t(T₂)))/2
```

Where CosSim represents cosine similarity between the encoded representations.
By incorporating these more accurate similarity metrics, we can avoid treating semantically related items as completely unrelated during training, which helps reduce the modality gap.

## References

- Closing the Gap (Connect, Collapse, Corrupt): <https://yuhui-zh15.github.io/C3-Website/>

#machine-learning
