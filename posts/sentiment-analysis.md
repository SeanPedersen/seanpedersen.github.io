---
title: 'Natural Language Processing: Sentiment Analysis'
date: '2018-10-13'
---
In this article we will take a closer look at sentiment analysis. Given an input text predict the sentiment (emotional state) as either negative (0) or positive (1). Sentiment analysis is useful to understand human text interactions at big scales. (Updated: 13th January, 2021)

## Dataset: IMDB Movie Reviews

A widely used dataset for sentiment classification is provided by the Internet Movie DataBase (IMDB). The training data set consists of 50,000 movie review texts, each associated with the target prediction label 0 or 1.

## Models

We compare four increasingly complex approaches to tackle the IMDB sentiment classification problem.

### Logistic Regression with TF-IDF & Bi-Grams

- **N-Grams**: Tuples of order N created from a sequence of symbols (words). Bi-gram(a,b,c) = {(a,b),(b,c)}
- **Term Frequency (TF)**: Number of occurrences for each word in a document (here a review is a document) of a collection of documents. Results for each document in a sparse (since most likely not all documents share the same words) row vector, where each dimension represents a unique word of the total collection of documents.
- **N**: Number of total documents (reviews).
- **N(w)**: Number of documents in which a word w appears at least once.
- **Inverse Document Frequency (IDF)**: log(N / N(w))
- **TF-IDF**: TF * IDF

TF-IDF preprocessing results in weighing words heavily that are rare across all documents. The opposite is true for words that are common across all documents. Thus TF-IDF can be interpreted as a heuristic that minimizes the significance of common stop words (the, you, etc.) without needing to explicitly provide a static list of stop words.

### Neural Embeddings

Embeddings represent relationships between entities in usually high-dimensional geometric spaces (f.e. euclidean or hyperbolic). One common and useful embedding space for words as entities encodes the semantic / contextual relationships between them. For example one would want words with similar meaning (e.g. synonyms) to be reflected in the embedding by a small distance. A common model to compute embeddings for natural language texts is Word2Vec. Word2Vec can be trained in two ways: either by predicting surrounding words (context) for a given word (Skip-gram) or predicting the missing word from a given context (CBOW).

Instead of training text embeddings unsupervised (without labels) using f.e. Word2Vec: We can train an embedding for our problem at hand as an input layer.
The input layer embedding will learn a problem specific representation (determined by the training data labels and loss function).
The embedding layer works by converting each word-token of an input sequence into a one-hot vector resulting in a big sparse matrix as input, this matrix is matrix-multiplied with an embedding matrix resulting in a dense often low dimensional embedding layer output. The embedding matrix is trained by back-propagated gradient steps from the last layers output computed loss function.

### LSTM

LSTM stands for “Long Short-Term Memory” and is a neural architecture designed to model sequential data. It is engineered to overcome the vanishing / exploding gradient problem encountered with classical Recurrent Neural Networks (RNN), enabling LSTM's to model long-term relationships between data points in long sequences. They tackle the fundamental problem of deciding between noise and signal by selectively “remembering” relevant data and “forgetting” irrelevant data with respect to the loss function.

Learn how LSTM's function: [http://colah.github.io/posts/2015-08-Understanding-LSTMs/](http://colah.github.io/posts/2015-08-Understanding-LSTMs/)

### Transformer

Transformers are all the rage for text and even vision tasks now a days. They perform better at modeling sequential interdependencies of their inputs using so called attention heads. Their input size is fixed so they are not recurrent neural networks. Architecturally they are closer related to convolutional neural networks which they are making obsolete for most vision tasks.

Learn how Transformers work:
- [https://peterbloem.nl/blog/transformers](https://peterbloem.nl/blog/transformers)
- [https://jalammar.github.io/illustrated-transformer/](https://jalammar.github.io/illustrated-transformer/)

#machine-learning
