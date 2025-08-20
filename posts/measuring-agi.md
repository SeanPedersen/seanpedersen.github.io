---
title: 'Metrics for Artificial General Intelligence'
date: '2025-07-16'
---

We need a new benchmark score that expresses the robustness of models within a coherent problem set (punish brittleness -> making stupid mistakes while otherwise achieving good scores). Humans possess very robust (generalizable) intelligence, preventing hallucinations and adversarial examples (small changes in input space causing large changes in prediction space).

----------------

**Thinking about the I in AI**

A framework allowing to compare intelligence metrics for (AI) agents
across problems. Focus lies on general intelligence and making published
results easier to compare.

## Introduction

Many recent publications in the field of AI may seem to the uninitiated
like great scientific leaps pushing the so called "state of the (dark)
art". But are we really building more **intelligent** systems though?

To answer this question a definition of intelligence including a metric
that quantifies intelligence of an agent for a given problem (set) is
needed. This paper tries to do exactly this: develop a family of
universally applicable intelligence metrics that are problem (set)
contextual.

Most of current AI research (mainly Machine/Deep Learning) is dedicated
to increasing either some accuracy metric (self-/ supervised problems)
or more generally some reward score (Reinforcement Learning). This is
often achieved by carefully human (AI researchers) designed
"intelligent" systems that improve incrementally on prior published work
scores on common benchmarks.

These SOTA achieving papers all answer the same (wrong) question: How
did we hand design and tune system X that achieved a better accuracy /
reward than most other previously published works solving a narrow class
of problems.

Sadly many papers that indeed achieve new impressive results, then
continue to publish it with too often post rationalized explanations
cluttered with fancy but unrelated mathematical proofs to make things
look sophisticated enough for "prestigious" conferences or journals.

This paper tries to explain why the current trend of publishing
incrementally fine tuned specialized "AI" solutions, is the wrong
question to ask as an AI researcher with a **focus on Intelligence** and
**not on Artificial** (human designed) systems that solely solve
specialized (narrow) problems. A novel and simple family of formalized
metrics for (artificial) intelligence agents are introduced in the hope
of sparking more interest into the design of generally intelligent
agents.

Meta-learning or AI-Generating Algorithms as a research field is slowly
gaining the traction it deserves, on the premise of exponential growth
of compute capacity. Which will likely continue with new manufacturing
processes and completely different computation paradigms such as
quantum, photonic and biological computers on the horizon.

## Definitions

The following definitions are not presented as objectively correct
definitions of the following concepts but merely serve as helpful
building blocks to understand the ideas this paper tries to convey.

**Data** (Information) is raw and meaningless. Exists as measurable/observable quantities.
Its simplest form is a bit: 0 / 1 or stated differently the smallest
observation is a variable with two states.

**Observation** (Perception) is data perceived by an agent from the environment (through sensors).

**Problem** (Goal) is defined as anything that can be formulated as a
loss/objective/reward/fitness function with at least one solution (>0
global maxi-/minima exist/s). E.g. classifying the MNIST test set
accurately or winning a game of chess.

A conditional probability distribution over all possible observation
sequences assigning each observation sequence a reward. This problem
definition is more general than f.e. a function that merely maps single
observations to a reward.

Sequential Decision Problems

Urproblem: Evolution - Stay alive aka stay around to observe the next
moment. In a universe that can destroy an agent it logically follows for
an agent to adapt to its environment in order to stay alive. For this to
happen pure chance aka randomness and some time is enough - no form of
consciousness is needed to act intelligently - to produce combinations
of matter that actively manipulate their environment to stay alive.

Problem(Environment)

**Knowledge** is just data (executed as programs) predicting accurately (ideally lossless) unknown / future data.

**Intelligence** is the rate at which an agent generates knowledge solving a given problem
from data w.r.t. observations or time needed to do so. This definition
is context (problem) specific. The intelligence of two or more agents
can thus only be compared for the same problem set.

**General intelligence** capabilities of an agent are defined for a set of
problems, measuring the agent's intelligence for each problem, then
normalizing all subproblem scores (0,1), finally computing the average
as the general intelligence score for the given set of problems for the
agent.

**Problem complexity** is defined for a given problem by the minimal number of observations needed
by the most possibly (general? -> which problem set) intelligent agent
to solve the problem (=extract all and not a bit more of knowledge that
is needed to solve the given problem contained in the data of perceived
observations).

**TODO:**

-   Is there a practical way to compute / estimate the problem
    complexity for a problem? Theoretical measures like Kolmogorov
    complexity tend to be incomputable in praxis.

-   Think about complete sets of similar problems, which would prevent
    the agent from being able to have had some similar exposure to a
    problem preventing any form of transfer learning / solving by
    analogy.

**Environment**: A state machine or any other process that generates a new state based or
not based on its prior state and current actions executed by agent/s in
the environment.

Environment(T) = Actions(Environment(T-1))

**Agent**: A system capable of perceiving data for a problem domain environment
that produces solution/s for given problems by executing sequences of
actions, changing the state of the environment (solving the problem).

E.g. uniform distribution over all possible solutions (random agent), NN
trained with gradient descent or RL or simply an octopus, a human etc.

The agent perceives a subset of the current (t) environment state and
produces actions that shall solve the problem, leading to a change in
the environment's state (t+1). Rinse and repeat:

Agent(Environment~T~, Problem) -> Actions -> A(E~T+1,~ P) -> ...

**Agent Size**: Amount of all bits of information used by the agent to solve the given
problem.

Composed of:

-   prior knowledge (meta knowledge helping to acquire problem specific
    knowledge)

-   perceived data (experience in problem domain)

-   knowledge (learned skills to solve the problem)

**Energy Consumption**: Instead of using a physical measure of energy like Joule one might want
to use a more theoretical measure like e.g. FLOP to truly capture the
algorithmic intelligence with regards to computational efficiency
advances. This would move the field also away from current hardware and
time resource competitions (dominated by big tech companies) creating
superficial (big hardware enables big data crunching, creating "SOTA"
results...) progress.

Example: compare AIME for best human player vs AIME of alpha go to
demonstrate importance of it.

## Intelligence Metrics Family

A measure of intelligence an agent exhibits for a given problem can be
quantified as the accuracy of knowledge (solution) generated by the
agent divided by the number of observations or time frames needed to do
so:

**IM** = (knowledge accuracy) / (used observations)

To prevent researchers from circumventing the goal of building more
intelligent systems another coefficient can be added:

The size of a given agent e.g. number of weights as another coefficient
to prevent AI designers from just looking once at the training data and
copying it to minimize number of used observations.

E.g. : Image classification data set contains 1000 images, one could use
a conventional CNN architecture and train it for many epochs thus using
many observations repeatedly. The submitters could try to minimize their
used observations by copying all 1000 images in only one pass into their
agent, executing hidden away from the AIM metric system many training
epochs and thus cheating by hiding the use of many more observations.

Accounting for the agent size can also be viewed as favoring simple
solutions (Occam's razor).

**AIM** = (knowledge accuracy) / ((used observations) * (agent size))

### Examples

**Supervised learning**: An agent observing 1,000,000 images in
training, achieving an accuracy of 90% on the test set, would be less
intelligent than an agent that needs to observe only 1,000 images to
achieve the same or better accuracy on the test set.

**Reinforcement Learning**: An agent spending 1,000,000 hours or frames
of exposure in an environment to achieve N reward points would be dumber
than one that only needs 1,000 to score the same or higher reward.

### Extensions

The introduced artificial intelligence metric does not consider the
amount of energy spent while training to compute the knowledge. A
natural extension of AIM could therefore be to use energy usage as the
divisor. This should naturally nullify any attempt to do "hidden"
computations (trying to minimize observations, gaming the AIM score)
since it could be measured on a hardware level. Also this is the most
ethical AIM score since it promotes energy efficient agents, helping our
planet's climate and thus humanity:

**AIM/E** = AIM / (spent energy)

or simply:

**IM/E** = IM / (spent energy)

**Find out:**

-   Maybe (knowledge accuracy) / (spent energy) is enough to capture
    "efficient" intelligence? OTOH agent size should favor short and
    simple solutions ...but spent energy as well -> what is the
    correlation or even causation between short (simple) solutions and
    spent energy?

-   Which of the two are easier to implement?

## Back to the Roots

How can AIM benefit from powerful concepts like Kolmogorov complexity,
computational complexity, Solomonoff induction, AIXI and Gödel machines?

### Comparison with other Intelligence Metrics

AIXI / Universal Intelligence (based on kolmogorov complexity) - Hutter

"the intelligence of a system is a measure of its skill-acquisition
efficiency over a scope of tasks, with respect to priors, experience,
and generalization difficulty" - F. Chollet

### Technical Implementation

To calculate any AIM metric we need:

-   Agent (Algorithm that produces solution/s)
-   Environment (Observations/time frames perceived by agent)
-   Problem / Loss Function (Evaluation of solution / feedback / reward)

## Looking at recent AI research

What can we do in praxis with the introduced family of AI metrics in our
toolbox?

We could take a closer look at recent (or all with a small
representative sample) research progress in a specific AI research
problem set for example Computer Vision and see how novel SOTA achieving
approaches compare to one another under the light of AIM. E.g. dense vs
convolutional vs residual vs depth map vs NEAT on small and big problem
sets of similar and dissimilar kind.

Another application could be to report AIM scores as well as accuracy.

**Q:** What is by above definitions the current evaluation metrics of
most AI papers? In supervised learning it is accuracy, so what is it?
**A:** How accurately the hand designed agent "learned" correct
knowledge solving the given problem, e.g. the percentage of correctly
predicted unknown/future information (labels/state rewards). This gives
rise to unstable hand tuned algorithms that easily break when hyper
parameters are changed.

**Q:** How does this thought framework apply to Reinforcement Learning?
**A:** It is easy to apply. RL achieves the same thing as classical
supervised learning: predicting the unknown. Only that in sup. learning
the unknown is static (timeless) and in RL it is dynamic (temporal:
dependent on time). Both optimize an objective SL by minimizing loss
function and RL agents maximizing reward.

## Existing Research on Intelligence Metrics

In the paper "Universal Intelligence:A Definition of Machine
Intelligence" by Shane Legg and Marcus Hutter, the authors propose
following properties for a useful intelligence metric:

- **Valid.** A test/measure of intelligence should be just that, it should capture intelligence and not some related quantity or only a part of intelligence.
- **Informative.** The result should be a scalar value, or perhaps a vector, depending on our view of intelligence. We would like an absolute measure of intelligence so that comparisons across many agents can easily be made.
- **Wide range.** A test/definition should cover very low levels of intelligence right up to super human intelligence.
- **General.** Ideally we would like to have a very general test/definition that could be applied to everything from a fly to a machine learning algorithm.
- **Dynamic.** A test/definition should directly take into account the ability to learn and adapt over time as this is an important aspect of intelligence.
- **Unbiased.** A test/definition should not be biased towards any particular culture, species, etc.
- **Fundamental.** We do not want a test/definition that needs to be changed from time to time due to changing technology and knowledge.
- **Formal.** The test/definition should be specified with the highest degree of precision possible, allowing no room for misinterpretation. Ideally, it should be described using formal mathematics.
- **Objective.** The test/definition should not appeal to subjective assessments such as the opinions of human judges.
- **Fully Defined.** Has the test/definition been fully defined, or are parts still unspecified?
- **Universal.** Is the test/definition universal, or is it anthropocentric?
- **Practical.** A test should be able to be performed quickly and automatically, while from a definition it should be possible to create an efficient test.

The AIM family ticks off all proposed properties, even the last one
which Legg & Hutter do not achieve with their proposed universal
intelligence score since it relies on the Kolmogorov complexity which is
not feasible to compute in praxis.

# Human designed overfitting

Most research around AI is narrow and specialized. A group of
researchers think together hard about this one problem, coming up with
novel, even impressive strategies / heuristics or whatever to solve the
problem better than anyone before them (achieving so-called SOTA).

We strongly need to move away from this specialized single problem
solver agent research competition (narrow AI research) and push for more
universally applicable AI systems. Projects like OpenAI's gym are a
great step into the right direction, since they offer an accessible and
diverse problem landscape. Curriculum learning / iterative
complexification of training environments is a promising field of
research as well that deserves more attention, since it could be a key
ingredient to AGI by offering a diverse set of problems.

Less specialized priors about the problem landscape aka environments
will lead to more general learning algorithms that perform well on
broader ranges of problems - the missing part of most of current AI
research endeavors.

# Future Research Directions

The tools developed here should enable us to directly optimize for more
intelligent agents by optimizing (maximize) directly for an AIM score!
e.g. with evolutionary algorithms and compressed genomes (indirect
encodings) for big problem sets trained on a big cluster of GPU's
refocusing on general / universal intelligence research instead of the
current hype of narrow and wasteful AI research.

Since AIM includes agent size training an agent to maximize general AIM
score for a set of related problems should give natural rise to transfer
learning in order to minimize the agent size resulting in more general
applicable intelligent agents.

**Notes:**

Notion of intelligence is heavily shaped by thinking about the
biological evolution of life. -> Evolution (Physics&Time) is the only
mother of intelligence.

Evolution favors organisms that are highly adaptable to the environments
they live in. Small species with short lifespans, achieve high
adaptability by rapid reproduction making many DNA mutations possible.
Larger species with longer life times need to adapt within their life,
that is why biological neural networks developed for them to stick
around, since they enable them to stay highly adaptable within their
lifetime.

Exploring, researching intelligence from a biological perspective seems
also a promising route to AGI. But only the bottom up approach seems
tractable. First understand the nervous system of simple life forms like
a worm (C. Elegans) before you try the same for a mouse or dare I say
the human brain (you know: just the most complex object in the universe
we know of).

# References

- ["Universal Intelligence: A Definition of Machine Intelligence"
    (Shane Legg, Marcus Hutter)](https://arxiv.org/pdf/0712.3329.pdf)
- **[The Measure of
    Intelligence](https://arxiv.org/pdf/1911.01547.pdf) (Francois
    Chollet)**
- The Measure of All Minds **(José Hernández-Orallo)**
- Jürgen Schmidhuber
    [**Gödelmaschine**](http://people.idsia.ch/~juergen/goedelmachine.html)
    - <http://people.idsia.ch/~juergen/unilearn.html>
- Marcus Hutter - **AIXI**
    - <http://www.hutter1.net/ai/introref.htm>
- Jeff Clune - **AI-GAs**
    - <https://arxiv.org/pdf/1905.10985.pdf>
- <https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication970.pdf>
- <https://www.emeraldinsight.com/doi/pdfplus/10.1108/IJCS-01-2018-0003>
- <http://dspace.cusat.ac.in/jspui/bitstream/123456789/3187/1/MEASURING%20UNIVERSAL%20INTELLIGENCE.pdf>
- <https://pdfs.semanticscholar.org/5204/a0c2464bb64971dfb045e833bb0ca4f118fd.pdf>
- <http://www.vetta.org/documents/AIQ_Talk.pdf>
- <https://www.lesswrong.com/posts/vPtMSvnF8B5hM5LdL/intelligence-metrics-and-decision-theories>
- <https://arxiv.org/pdf/0712.3329.pdf>
- <https://ws680.nist.gov/publication/get_pdf.cfm?pub_id=824478>
- <https://arxiv.org/abs/1805.07883>
- <http://www.incompleteideas.net/IncIdeas/DefinitionOfIntelligence.html>
- <https://breckyunits.com/intelligence.html>

# Author Notes on existing Literature (incomplete)

So far nothing really strongly related to AIM has shown up in
references.

BUT his one stood out: ["Universal Intelligence:A Definition of Machine
Intelligence" by Shane Legg and Marcus
Hutter,](https://arxiv.org/pdf/0712.3329.pdf)

They favor problems / environments with low Kolmogorov complexity in
their definition of "Universal Intelligence" by weighing inversely
proportional Kolmogorov complexity for each environment over all
computable ones.

Legg and Hutter define their Universal Intelligence across all
computable environments of all complexities. AIM family could be
extended to all possible environments as well but is also applicable to
subsets or even single environments as well, which Legg and Hutter do
not seem to cover since computing the Kolmogorov complexity for even
small problems quickly becomes infeasible.

Ok if we take LH def of I and use it for a subset of all environments or
problems, weird stuff happens: We can define the AIM for the problem set
A = {MNIST, CIFAR-10}

Applying LH-UI def. kolmogorov(CIFAR-10) > kolmogorov(MNIST) and thus
the ability to solve MNIST would be weighted more. AIM would just weigh
both equivally. MNIST being way simpler to solve than CIFAR-10 would
favor agents adapted solely to MNIST more than the other way around
which is not a good thing to do when evaluating the
universal intelligence of an agent.

"The solution then, is to require the environmental probability measures
to be computable." LH restricts their definition of intelligence to all
computable environments. -> No-Free-Lunch-Theorem kicks in! The space
of all computable environments is still so vast that most environments
would act seemingly random, resulting in the best agent acting randomly.

<http://incompleteideas.net/IncIdeas/DefinitionOfIntelligence.html>

Solomonoff Induction:

<https://www.lesswrong.com/posts/Kyc5dFDzBg4WccrbK/an-intuitive-explanation-of-solomonoff-induction>

#AI #machine-learning
