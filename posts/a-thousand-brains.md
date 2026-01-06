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

ref frame = map

for abstract concepts like words in a language f.e. democracy
we create abstract maps that encode how democracy is structured and how it behaves
how to navigate democracy: how to create a new party? etc.

Properties of maps:
- indexable (knowing a few bits of a location, allows us to fetch the correct map & location)
- structured by locality (abstract: similarity)
- find routes from location / concept A to B

## Road to AGI

Hawkins is sure that deep learning (including LLM) will not lead to AGI. Instead he identified the following properties for AGI.

- continuous learning
- learning via movement
- compositional world model of objects
- knowledge stored in reference frames

## References

- [YouTube - Jeff Hawkins NAISys: How the Brain Uses Reference Frames](https://youtu.be/mGSG7I9VKDU)

#book #AI
