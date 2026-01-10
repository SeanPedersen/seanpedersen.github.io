---
date: '2025-12-27'
---
# A Thousand Brains - Jeff Hawkins

## Reference Frames

Identifies so called reference frames (grids / maps) as key concept unifying all neural network processing for robust / invariant sequence prediction (allowing sequence prediction in changing environments by using relative positions).

Key points:
- Grid cells: Each cortical column uses grid cell-like mechanisms (from the entorhinal cortex) to create reference frames anchored to objects, not just spatial locations.
- Object-centered coordinates: When you touch a coffee cup, each column builds a model in the cup's own reference frame - tracking features relative to the object itself, not your body or the room.
- Voting mechanism: Thousands of columns process different inputs (different fingers touching, different viewpoints) in parallel. They "vote" to reach consensus on what object is present and its pose.
- Location + feature: Each column stores what features exist at specific locations within an object's reference frame. Moving your sensor updates the location signal.
- Compositional models: Reference frames allow hierarchical object models - a wheel has its own frame nested within a car's frame.

for abstract concepts like words in a language f.e. democracy
we create abstract maps (ref. frame) that encode how democracy is structured and how it behaves (model)
how to navigate democracy: how to create a new party? etc.

Properties of reference frames:
- indexable (knowing a few bits of a location, allows us to fetch the correct map & location)
- structured by locality (abstract: similarity)
- find routes from location / concept A to B

Useful knowledge representation uses object based models: allowing to make inferences by predicting interactions / behavior. Storing hard facts (as text) is necessary but not sufficient for a useful knowledge base. Storing knowledge in gigantic, fractured weight matrices (like used in LLMs) is useful and interesting but does not produce human like adaptive intelligence.

## Road to AGI

Hawkins is sure that deep learning (including LLM) will not lead to AGI - as they lack model based knowledge representation. Instead he identified the following properties for realizing AGI.

- Continuous learning
- Learning via movement
- Compositional world model: predictive (behavior) models of objects
- Knowledge stored in reference frames

## References

- [YouTube - Jeff Hawkins NAISys: How the Brain Uses Reference Frames](https://youtu.be/mGSG7I9VKDU)

#book #AI
