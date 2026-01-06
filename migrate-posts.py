#!/usr/bin/env python3
import os
import re

posts_dir = '/Users/Studio-MacMini/projects/seanpedersen.github.io/posts'

# First, clean up any .tmp files from failed migration
for filename in os.listdir(posts_dir):
    if filename.endswith('.tmp'):
        tmp_path = os.path.join(posts_dir, filename)
        os.remove(tmp_path)
        print(f"Removed {filename}")

for filename in os.listdir(posts_dir):
    if not filename.endswith('.md'):
        continue

    filepath = os.path.join(posts_dir, filename)

    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()

    # Extract title from frontmatter using regex
    title_match = re.search(r"^title:\s*['\"](.+?)['\"]", content, re.MULTILINE)

    if not title_match:
        print(f"Skipping {filename}: no title found")
        continue

    title = title_match.group(1)

    # Remove the title line from frontmatter
    new_content = re.sub(r"^title:\s*['\"].+?['\"]$\n?", '', content, flags=re.MULTILINE)

    # Add the title as H1 after the closing --- of frontmatter
    # Find the second occurrence of ---
    parts = new_content.split('---', 2)
    if len(parts) >= 3:
        # parts[0] is empty, parts[1] is frontmatter, parts[2] is content
        new_content = f"---{parts[1]}---\n# {title}\n{parts[2]}"

    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(new_content)

    print(f"Migrated {filename}")

print("Migration complete!")
