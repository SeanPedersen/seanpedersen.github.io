# mRNA Vaccines: How They Work

mRNA vaccines work by mimicking a trick viruses have used for billions of years: injecting genetic instructions (mRNA) into your cells to make them produce a specific target protein.

## What is mRNA?

Messenger RNA (mRNA) is a short-lived molecule that carries a protein blueprint copied from DNA. Cells read mRNA through ribosomes, their protein-making machinery, and build the protein the mRNA describes. Once read, the mRNA is degraded. It never enters the cell nucleus and cannot alter your DNA.

## The Blueprint Trick

A virus survives by injecting its own genetic instructions into host cells, forcing them to manufacture viral proteins. mRNA vaccines borrow exactly this strategy. The vaccine delivers a synthetic mRNA strand encoding a target protein. Your ribosomes read it and build that protein. Your immune system sees it as foreign, mounts a defence, and stores memory cells for future protection. You are never exposed to the actual pathogen.

Your own cells are the factory. The mRNA is the blueprint.

## The Delivery Problem

Naked mRNA degrades within seconds in the bloodstream. It also cannot cross cell membranes on its own. The solution is lipid nanoparticles (LNPs): tiny fatty shells that protect the mRNA and fuse with cell membranes to deliver the payload inside.

LNPs have no precise targeting ability. They distribute largely by proximity. Most are absorbed by cells at the injection site, nearby immune cells, and to some extent the liver, which efficiently clears lipid particles from blood.

## Why the Deltoid?

The standard injection site is the deltoid muscle in the upper arm. This is not arbitrary. Muscle tissue in this region is rich in dendritic cells and macrophages, immune cells that pick up foreign proteins and present them to the adaptive immune system. The axillary lymph nodes sit nearby, which is where antibody production and memory cell formation are organised.

Injecting into a vein rather than muscle sends LNPs circulating systemically, reaching organs beyond the intended local site. This distinction between intramuscular and intravenous delivery has practical safety implications and is why injection technique matters. Accidental intravenous injection sends LNPs into blood circulation, potentially reaching the heart and causing myocarditis (inflammation of the heart muscle) like it happened with corona vaccines.

## What Happens Inside

1. LNPs are absorbed by local muscle cells and nearby immune cells
2. mRNA is released inside the cell
3. Ribosomes read the mRNA and produce the target protein (for COVID vaccines: the spike protein)
4. The cell displays fragments of this protein on its surface
5. The immune system recognises the protein as foreign
6. B cells produce antibodies; T cells learn to attack cells displaying this protein
7. Memory cells are stored for long-term protection
8. The mRNA degrades within days

## Beyond Infectious Disease

mRNA technology is not limited to vaccines against pathogens. Its core ability, instructing cells to produce any protein on demand, opens broader applications.

Cancer treatment is the most significant near-term area. Tumours carry mutations that produce abnormal proteins called neoantigens. These are unique to each patient's cancer. Researchers can sequence a patient's tumour, identify its neoantigens, synthesise a personalised mRNA vaccine encoding those targets, and inject it to train the immune system to attack cells displaying them. This approach, already in clinical trials for melanoma and other cancers in combination with checkpoint inhibitors, effectively turns the immune system into a targeted therapy customised per patient.

Other areas in active development include:

**Protein replacement therapies**: Delivering mRNA that encodes a missing or defective protein, relevant for conditions like cystic fibrosis or certain metabolic disorders.

**Autoimmune conditions**: Tolerogenic mRNA approaches, where instead of activating an immune response, the goal is to suppress one by instructing cells to express proteins that calm immune activity against self-tissue.

**Rare genetic diseases**: Periodic mRNA dosing could temporarily restore function of a protein the body cannot produce, without permanent gene editing.

## mRNA vs Protein Vaccine — Inflammatory Risk

**Activation timeline**
- mRNA: protein detectable after ~4 hours, peaks at 24 hours, gone by 72 hours
- Protein vaccine: antigen immediately available but fixed dose, degrades within hours
- Immune response timeline is similar for both (~14 days to meaningful antibodies) — the difference is mechanism, not speed of protection

**The amplification tradeoff**
mRNA continuously produces fresh antigen for 72 hours, generating a stronger immune signal than a fixed injected protein dose. This is why protein vaccines need adjuvants and often booster shots to match mRNA efficacy — but it also means mRNA keeps the inflammatory stimulus active longer.
Vein exposure risk

mRNA: if accidentally injected intravenously, LNPs distribute systemically and cells throughout the body — including cardiac tissue — produce spike protein for up to 72 hours, creating a sustained inflammatory stimulus

Protein vaccine: if accidentally IV injected, you get a fixed bolus that immediately begins degrading — no ongoing production, self-limiting exposure

**The double hit problem**
mRNA vaccines have two independent inflammatory sources: the LNP/mRNA delivery mechanism itself, plus the immune response to the expressed protein. Protein vaccines with adjuvants deliberately add back one inflammatory stimulus, but it's more controlled than systemic LNP distribution.

**What the data shows**
Mouse studies confirm IV mRNA injection causes myocarditis. Some countries (Denmark, Hong Kong) adopted aspiration before injection as a precaution. But the natural experiment — comparing myocarditis rates between countries that did and didn't adopt aspiration — has not been published, leaving your core hypothesis scientifically open.

## What Comes Next

The platform's key advantage is speed. Once a protein target is identified, the mRNA sequence can be designed computationally and synthesised in weeks. This compresses development timelines that previously took years. During COVID-19, the Moderna vaccine went from sequence design to first human dose in 63 days.

The bottleneck is now delivery. Current LNPs distribute broadly rather than to specific cell types, and tend to accumulate in the liver by default. For immune-based cancer vaccines, the goal is not to reach tumour cells directly — the immune system handles that — but to deliver mRNA efficiently to antigen-presenting cells like dendritic cells, which then orchestrate the immune response. For direct therapeutic approaches, such as gene editing or reprogramming cells within the tumour microenvironment, precise delivery to specific cell types does matter. More broadly, targeting mRNA to tissues like lung epithelium or particular immune cell populations would dramatically expand what is possible and reduce off-target effects. This is an active area of research.

mRNA vaccines are not a narrow tool for infectious disease. They are a general platform for programming cellular protein production. The COVID vaccines were the proof of concept at scale. Personalised cancer treatment may be the first demonstration of their full potential.

## References

- https://www.nature.com/articles/s41541-021-00292-w
- https://www.nejm.org/doi/full/10.1056/NEJMoa2034577
- https://www.nature.com/articles/s41586-023-06063-y

#idea
