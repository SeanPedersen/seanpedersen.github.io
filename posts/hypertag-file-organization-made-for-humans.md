---
title: 'HyperTag: File Organization made for Humans'
date: '2021-01-10'
---
<a href="https://github.com/SeanPedersen/HyperTag" rel="noreferrer noopener" target="_blank">HyperTag</a> let's humans intuitively express how they think about their files using tags and the power of modern machine learning. Instead of introducing proprietary file formats like other existing file organization solutions, HyperTag just layers on top of your existing files without any fuss.

HyperTag is built around one simple goal: **Minimize the time between a thought and access to all relevant files.**

## Feature Overview
HyperTag offers many unique features that help it achieve its overall goal. It comes with a slick CLI but more importantly it creates a directory called ```HyperTagFS``` which is a file system based representation of your files and tags using symbolic links and directories. ```HyperTagFS``` also allows you to create queries and searches by creating directories, more about that later.

### Directory Import
A very useful feature, allowing to import your existing directory hierarchies using ```$ hypertag import path/to/directory```. HyperTag converts it automatically into a tag hierarchy using metatagging.

**Example Directory Hierarchy:**

- Documents
  - Research
    - Banana Farming
        - Files: BananaBible.pdf, BananaFarmingForDummies.epub
    - Machine Learning
        - Files: DeepLearning.pdf
  - eBooks
    - Chess
        - Files: ChessForDummies.pdf

Import: ```$ hypertag import path/to/Documents```

**Resulting MetaTag hierarchy:**<br>
BananaBible.pdf and BananaFarmingForDummies.epub are tagged with ```Research > "Banana Farming"```. DeepLearning.pdf is tagged with ```Research > "Machine Learning"```. And ChessForDummies.pdf with ```eBooks > Chess```.

### Fuzzy Matching Tag Queries
HyperTag uses fuzzy matching to minimize friction in the unlikely case of a typo. You can find your freshly imported files using queries based on set theory:
- ```$ hypertag query Reserch and "Mchine Learning"``` prints: DeepLearning.pdf
- ```$ hypertag query Ches or "Bana Farmin"``` prints: ChessForDummies.pdf, BananaBible.pdf, BananaFarmingForDummies.epub
- ```$ hypertag query Rsearch minus "Banna Farmng"``` prints: DeepLearning.pdf

### Semantic Text & Image Search
HyperTag uses the latest research advances from Machine Learning for vision and text to provide easy access to all your files: Search for **images** (jpg, png) and **text documents** (yes, even PDF's) content with a simple text query. Text search is powered by the awesome [Sentence Transformers](https://github.com/UKPLab/sentence-transformers) library. Text to image search is powered by OpenAI's [CLIP model](https://openai.com/blog/clip/). Currently only English queries are supported.

**Semantic search for text files**<br>
Print text file names sorted by matching score.
Performance benefits greatly from running the HyperTag daemon.
<br>Shortcut: `$ hypertag s`

```$ hypertag search "your important text query" --path --score --top_k=10```

**Semantic search for image files**<br>
Print image file names sorted by matching score.
Performance benefits greatly from running the HyperTag daemon.
<br>Shortcut: `$ hypertag si`

Text to image:
```$ hypertag search_image "your image content description" --path --score --top_k=10```

Image to image:
```$ hypertag search_image "path/to/image.jpg" --path --score --top_k=10```

### HyperTag Daemon
The HyperTag Daemon is a process you may want to add as an auto starting service as it makes your HyperTag experience at least 3x better. How? It provides not one but three awesome features you do not want to miss out:
- Watches `HyperTagFS` directory for user changes:
  - Maps file (symlink) and directory deletions into tag / metatag removal/s
  - On directory creation: Interprets name as set theory tag query and automatically populates it with results
  - On directory creation in `Search Images` or `Search Texts`: Interprets name as semantic search query (add top_k=42 to limit result size) and automatically populates it with results
- Watches directories on the auto import list for user changes:
  - Maps file changes (moves & renames) to DB
  - On file creation: Adds new file/s with inferred tag/s
- Spawns DaemonService to load and expose models used for semantic search, speeding it up significantly

### File Type Groups
A small but convenient feature: HyperTag automatically creates folders containing common files (e.g. Images: jpg, png, etc., Documents: txt, pdf, etc., Source Code: py, js, etc.), which can be found in ```HyperTagFS```.

### HyperTag Graph
Another small but very useful feature: Quickly get an overview of your HyperTag Graph! HyperTag visualizes the metatag graph on every change and saves it at `HyperTagFS/hypertag-graph.pdf`.

![HyperTag Graph Example](https://raw.githubusercontent.com/SeanPedersen/HyperTag/master/images/hypertag-graph.jpg)

## Architecture
- Python and it's vibrant open-source community power HyperTag
- Many other awesome open-source projects make HyperTag possible (listed in `pyproject.toml`)
- SQLite3 serves as the meta data storage engine (located at `~/.config/hypertag/hypertag.db`)
- Symbolic links and directories are used to represent your files and tags in the `HyperTagFS` directory
- Semantic text search is powered by the awesome [DistilBERT](https://arxiv.org/abs/1910.01108)
- Text to image search is powered by OpenAI's impressive [CLIP model](https://openai.com/blog/clip/)

## Inspiration

I have wanted to use and also build a tagging based file organizer for a long time. In the beginning of 2021 I finally came around to do so. After evaluating existing options, I concluded none of them really satisfied my requirements. So I built HyperTag. Using Python the first prototype was working after only 4 hours of coding. 7 days later and HyperTag provides a rich set of features that help it achieve the goal I set out for it: Minimize the time between a thought and access to all relevant files. I hope others will find use for it as well and maybe even contribute back to it. I am certainly excited for the future of HyperTag and hope you are by now too.

#programming
