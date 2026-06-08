# AI Investment Forecast

[Generative AI](https://en.wikipedia.org/wiki/Generative_AI) is a powerful tool. But the fundamental architecture of [large language models](https://en.wikipedia.org/wiki/Large_language_model) imposes structural limits: no autonomy, no taste, no self-awareness. These are not gaps that more compute will close. That gap shapes where the real money should go.

## AI Infrastructure

The safest investment assuming centralized AI wins is energy. Giant data centers eat enormous amounts of power. [Microsoft](https://en.wikipedia.org/wiki/Microsoft) signed a long term power purchase agreement that helped justify the restart of Unit 1 at Three Mile Island (now the Crane Clean Energy Center), with the restart carried out by [Constellation Energy](https://en.wikipedia.org/wiki/Constellation_Energy). [Google](https://en.wikipedia.org/wiki/Google) and [Amazon](https://en.wikipedia.org/wiki/Amazon_(company)) followed with their own nuclear deals.

But decentralized AI cannot be ruled out. The mainframe vs personal computer story is worth remembering. Mainframes won for a while, then PCs took over. Local [large language model](https://en.wikipedia.org/wiki/Large_language_model) inference on consumer hardware is already real. If decentralized inference wins, the energy thesis weakens. But chip and hardware makers stay strong either way.

Solid bets on either path:

- [Rare earth element](https://en.wikipedia.org/wiki/Rare-earth_element) miners
- Chip producers (GPU, RAM, CPU)

Strong bet if centralized AI wins:

- Energy companies (nuclear, gas, renewables)

**Key risks:** Energy companies face regulated returns, grid bottlenecks, and permitting delays. Rare earth miners carry significant China concentration risk, political exposure, and boom-bust pricing cycles. Both are capital intensive with long lead times.

## Data Center Infrastructure

The GPU story is well known, but the physical infrastructure behind AI data centers is less discussed and potentially less crowded.

Every new data center requires transformers, switchgear, cooling systems, and grid equipment at scale. Power density per rack is rising sharply with newer GPU generations, and existing infrastructure was not built for it. Many investors believe these physical infrastructure suppliers are more direct beneficiaries of AI capex than generic energy plays.

Investment angle: electrical equipment makers, cooling technology companies, grid infrastructure suppliers.

**Key risks:** This is cyclical capital goods with long order backlogs. Lead times for transformers and switchgear can stretch years, meaning demand signals today reflect capex plans that could shift if AI investment slows.

## Networking

AI clusters are only as fast as the connections between chips. High speed interconnects inside data centers, optical networking between facilities, and data center switching infrastructure are all scaling rapidly to keep pace with compute demand.

This area has attracted less retail attention than GPUs while capturing a meaningful share of AI infrastructure spend.

Investment angle: high speed interconnect suppliers, optical networking companies, data center networking hardware.

**Key risks:** Networking equipment can be commoditized over time. If AI cluster architectures shift toward wafer-scale or near-memory compute, the interconnect bottleneck changes in character.

## NVIDIA Counter Thesis

Chip producers, especially [Nvidia](https://en.wikipedia.org/wiki/Nvidia), look like a safe bet from any angle. But Nvidia trades at roughly 20 times forward annual revenue. That price assumes years of near perfect execution. The risks are real.

Nvidia's moat rests on three things: [CUDA](https://en.wikipedia.org/wiki/CUDA) (its proprietary programming framework), its [Mellanox](https://en.wikipedia.org/wiki/Mellanox_Technologies) interconnect technology, and proven Linux drivers. These have supported extraordinary margins and pricing power in data center accelerators. [AMD](https://en.wikipedia.org/wiki/AMD) makes comparable chips on paper but has poor Linux drivers and weak interconnect, which keeps it out of serious data center contention.

**Hardware challengers** are finding ways around the interconnect problem entirely. [Cerebras](https://en.wikipedia.org/wiki/Cerebras_Systems) builds one giant wafer scale chip, eliminating the need for GPU to GPU communication. Its four largest customers are the Mohamed bin Zayed University of Artificial Intelligence (62% of 2025 revenues), G42 (24%), OpenAI, and Amazon Web Services, with the latter two signing in 2026. That OpenAI and AWS, two of Nvidia's largest customers, are buying from Cerebras illustrates how seriously the diversification pressure has become.
[Groq](https://en.wikipedia.org/wiki/Groq) uses a deterministic compute architecture that runs inference faster per dollar. [Taalas](https://taalas.com/the-path-to-ubiquitous-ai/) pours LLM directly into silicon, achieving +10K tok/sec. Neither needs to beat Nvidia on raw performance. They just need to be good enough at a fraction of the price. In December 2025, Nvidia and Groq announced an agreement reportedly valued at approximately 20 billion $ to license Groq's AI inference technology and transfer several senior Groq executives to Nvidia, suggesting Nvidia recognized the inference efficiency threat and moved to absorb it rather than outcompete it.

**Big tech is building its own chips.** Google has been on its sixth generation of in house TPUs since 2016. Amazon has Trainium and Inferentia chips powering clusters of over 400,000 chips for [Anthropic](https://en.wikipedia.org/wiki/Anthropic). Microsoft, [OpenAI](https://en.wikipedia.org/wiki/OpenAI), [Meta](https://en.wikipedia.org/wiki/Meta), and [Apple](https://en.wikipedia.org/wiki/Apple_Inc.) all have custom silicon projects underway. Every major Nvidia customer is working to reduce dependence on Nvidia. None of these chips needs to outperform Nvidia. They just need to break even rather than generate extraordinary margins for a rival.

**[DeepSeek](https://en.wikipedia.org/wiki/DeepSeek) suggested compute may have been massively over provisioned.** The Chinese AI lab's reported training efficiency indicated that frontier-adjacent models may require far less compute than many investors assumed. Even if the headline cost figures are incomplete, the broader point stands: algorithmic efficiency can pressure long-term compute demand assumptions. Every major AI lab is now studying how to apply those same techniques. If those efficiency gains prove durable, the compute demand assumptions baked into Nvidia's valuation face real pressure.

**Software is reducing CUDA lock in.** Frameworks like Triton (OpenAI), MLX (Apple), and JAX (Google) let developers write code once and compile it for multiple hardware targets. This mirrors how hand tuned assembly gave way to C in the 1980s. The CUDA expertise premium will erode as high level abstractions mature.

None of this means Nvidia falls tomorrow. If AI scales as fast as the optimists expect, demand growth could absorb all of these pressures. But at 20 times forward revenue, the market prices in no bad news at all. That is a difficult position to hold.

## Robotics: Embodied AI

The potential economic impact of mass scale deployment of robotics is enormous.

The global industrial robot market hit $16.5 billion in 2025. China installed 276,300 industrial robots in 2023, more than the rest of the world combined. By 2024, that number had risen to 295,000, representing 54% of global deployments, with China's operational robot stock exceeding 2 million. The trend is accelerating.

Three things drive it:

**Physical AI** trains robots in virtual simulations before they touch real environments. Chip makers and robot makers now build dedicated hardware for this. The goal is a "ChatGPT moment" for robots where they learn by experience rather than explicit programming.

**[Humanoid robots](https://en.wikipedia.org/wiki/Humanoid_robot)** get a lot of attention but remain early. Today they mostly do single purpose tasks in auto plants and warehouses. Whether they become general purpose tools is still open.

**Labor shortages** are the practical driver. Aging populations in the US, Japan, Germany, China, and South Korea are shrinking workforces fast. Robots fill the gaps. Oxford Economics forecast (published 2019, revisited 2024) projected 20 million manufacturing jobs displaced by robots by 2030. [McKinsey](https://en.wikipedia.org/wiki/McKinsey_%26_Company) estimates AI and robots could automate 40% of US jobs by 2030 while generating $2.9 trillion in economic value per year (via Robotics and Automation News, citing McKinsey research).

That is a massive disruption. The companies that build, program, and maintain these robots win.

Investment angle: robot makers, simulation software, [collaborative robot](https://en.wikipedia.org/wiki/Cobot) platforms, and industrial automation companies.

**Key risks:** Robotics hardware carries long sales cycles, high integration costs, and thin margins. Humanoid robots in particular are early; hype currently exceeds deployment. Industrial-grade reliability takes years and significant capital to prove out at scale.

## Medicine: Biggest Chance

[Drug discovery](https://en.wikipedia.org/wiki/Drug_discovery) is where AI might matter most.

Traditional drug development takes 10 to 15 years and costs over a billion dollars per drug. The failure rate is brutal. Most candidate drugs never reach patients.

[AlphaFold](https://en.wikipedia.org/wiki/AlphaFold), [Google DeepMind](https://en.wikipedia.org/wiki/Google_DeepMind)'s protein structure prediction model, changed that. It achieved a breakthrough in [protein structure prediction](https://en.wikipedia.org/wiki/Protein_folding), reaching accuracy levels that transformed the field after 50 years of limited progress. AlphaFold's creators [Demis Hassabis](https://en.wikipedia.org/wiki/Demis_Hassabis) and [John Jumper](https://en.wikipedia.org/wiki/John_M._Jumper) shared the 2024 Nobel Prize in Chemistry with [David Baker](https://en.wikipedia.org/wiki/David_Baker_(biochemist)), who received the other half for protein design work. AlphaFold can now predict the structure of proteins, RNA, and DNA. This gives drug designers a map of the molecules they want to target.

[Isomorphic Labs](https://en.wikipedia.org/wiki/Isomorphic_Labs), the company built on AlphaFold, secured $2.1 billion in funding in May 2026 to push drugs into clinical trials. It has active collaborations with [Johnson and Johnson](https://en.wikipedia.org/wiki/Johnson_%26_Johnson), [Novartis](https://en.wikipedia.org/wiki/Novartis), and [Eli Lilly](https://en.wikipedia.org/wiki/Eli_Lilly). The long term goal is to dramatically shorten the drug discovery process, potentially reducing timelines from many years to a fraction of today's norms.

On the clinical side, a Menlo Ventures survey found healthcare AI adoption jumped 7x in a single year among respondents, with health systems leading at 27% adoption. AI handles triage, scheduling, imaging analysis, and administrative work. The $4.9 trillion US healthcare sector now deploys AI at more than twice the rate of the broader economy.

Investment angle: biotech companies using AI for drug discovery, medical imaging AI, diagnostics, and clinical workflow tools. This sector has real data moats and hard to copy specialized models.

**Key risks:** Clinical failure rates remain high even if AI accelerates discovery. Finding a promising molecule is only the first step; the path through trials is long, expensive, and uncertain. Regulatory timelines and approval risks are unchanged by AI.

## Weapon Manufacturing

AI enabled weapon systems is a growing investment theme in the US and Europe. Autonomous systems, surveillance, logistics optimization, and military robotics are attracting significant government procurement and venture capital. Rising defense budgets in NATO countries and intensifying great power competition are structural tailwinds independent of the broader AI cycle.

Investment angle: defense contractors integrating AI into platforms, autonomous systems developers, drone and counter drone technology companies.

**Key risks:** Defense procurement cycles are long and unpredictable. Ethics and regulation around autonomous weapons remain unsettled. Government budget priorities can shift, and many defense AI programs remain classified, making competitive positioning difficult to assess from the outside.

## Application Layer Risk

Companies with no data moat and a thin app layer on top of general AI face increasing pressure.

When software gets cheap to build, value shifts to the data and relationships underneath. AI lowers software development costs and compresses feature advantages, increasing pressure on application layer businesses whose primary differentiation is functionality rather than proprietary data, distribution, or ecosystem effects. Companies with deep ecosystem lock in, enterprise relationships, compliance advantages, and marketplace network effects are more resilient than those competing purely on features.

Watch companies that:

- Sell software AI can replicate fast with no proprietary data or unique pipelines
- Lack meaningful switching costs, ecosystem effects, or distribution advantages
- Compete on feature parity rather than data or integration depth

Identifying a business as vulnerable is not the same as identifying a good short. Many mediocre businesses stay expensive to short for years. Valuation, balance sheet analysis, and timing matter as much as the competitive thesis.

## Ranking of AI Investment Themes

| Theme | Confidence | Main risk |
| --- | --- | --- |
| Grid and electrical infrastructure | High | Permitting delays, regulated returns |
| Data center networking | High | Commoditization over time |
| AI drug discovery | Medium-high | Clinical failure still dominates |
| Nvidia long-term dominance | Medium | Valuation, custom silicon, inference efficiency |
| Humanoid robotics | Low-medium | Hype exceeds current deployment |
| Weapon manufacturing | Low-medium | Procurement cycles, classified opacity |
| Thin AI apps without moats | High vulnerability | Misidentifying apps with genuine moats as vulnerable |

**Highest confidence:** Data center electrical infrastructure, grid equipment, networking, and custom silicon supply chains. These benefit from AI capex regardless of which AI applications win.

**Highest upside, highest uncertainty:** AI drug discovery, humanoid robotics, defense autonomy. Long timelines with transformative potential if execution follows.

**Most vulnerable:** Thin application wrappers without proprietary data or distribution. The addressable market for feature-only software is shrinking as AI commoditizes development.

**Most crowded:** Nvidia and obvious GPU beneficiaries. The thesis is right, but the price may already reflect it.

## References

1. [Top 5 Global Robotics Trends 2025](https://ifr.org/ifr-press-releases/news/top-5-global-robotics-trends-2025) (International Federation of Robotics)
2. [World Robotics 2025 – Industrial Robots](https://ifr.org/ifr-press-releases/news/global-robot-demand-in-factories-doubles-over-10-years) (IFR)
3. [AI Index 2025: Economy Chapter](https://hai.stanford.edu/ai-index/2025-ai-index-report/economy) (Stanford HAI)
4. [Isomorphic Labs secures $2.1 billion funding](https://www.prnewswire.com/news-releases/isomorphic-labs-secures-2-1-billion-funding-to-scale-its-ai-drug-design-engine-302769674.html) (PR Newswire)
5. [Constellation to Launch Crane Clean Energy Center](https://www.constellationenergy.com/news/2024/Constellation-to-Launch-Crane-Clean-Energy-Center-Restoring-Jobs-and-Carbon-Free-Power-to-The-Grid.html) (Constellation Energy)
6. [NVIDIA Fiscal 2026 Financial Results](https://nvidianews.nvidia.com/news/nvidia-announces-financial-results-for-fourth-quarter-and-fiscal-2026) (NVIDIA Newsroom)
7. [AI and robots in 2025: the robotics revolution](https://www.oxfordeconomics.com/resource/ai-and-robots-in-2025-the-robotics-revolution-we-predicted-has-arrived/) (Oxford Economics)
8. [McKinsey: AI and robots could automate 40% of US jobs by 2030](https://roboticsandautomationnews.com/2025/11/26/mckinsey-warns-ai-and-robots-could-automate-40-percent-of-us-jobs-by-2030/97003/) (Robotics and Automation News)
9. [2025: The State of AI in Healthcare](https://menlovc.com/perspective/2025-the-state-of-ai-in-healthcare/) (Menlo Ventures)
10. [The Short Case for Nvidia Stock](https://youtubetranscriptoptimizer.com/blog/05_the_short_case_for_nvda) (Jeffrey Emanuel)

#AI #idea
