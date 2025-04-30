---
title: 'Analyze Your Twitter (X) Likes'
date: '2025-04-07'
---
A small tutorial how you can easily explore your past Twitter (X) likes using [Digger Solo](https://solo.digger.lol).

Export and download your personal Twitter data: Navigate to More -> Settings -> "Your account" -> "Download an archive of your data" and click "Request an archive" button. This will allow you to download an archive of all your Twitter interactions including all posts you ever liked.

## Converting the data

Create the file `export-likes.py` in your twitter archive directory (which is named sth. like twitter-random-date). And place following code into it, then run it using `python3 export-likes.py` (which creates the directory named twitter-likes containing all your likes as text files):

```python
import json
import os
import re


def preprocess_file(file_path):
    with open(file_path, "r", encoding="utf-8") as f:
        lines = f.readlines()
    # Remove the first line containing "window.YTD.like.part0 = ["
    cleaned_data = "[" + "".join(lines[1:])  # Skip the first line
    return cleaned_data


# Function to sanitize filenames
def sanitize_filename(filename):
    # Remove invalid characters for filenames
    sanitized = re.sub(r'[\\/*?:"<>|]', "", filename)
    # Replace newlines and tabs with spaces
    sanitized = re.sub(r"[\n\t\r]", " ", sanitized)
    # Limit length and trim whitespace
    return sanitized.strip()


# Function to parse JSON and write tweets to text files
def save_tweets_to_files(file_path):
    # Ensure the file exists
    if not os.path.exists(file_path):
        print(f"File not found: {file_path}")
        return

    # Create directory if it doesn't exist
    output_dir = "twitter-likes"
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)

    # Preprocess the file to remove invalid JSON prefix
    cleaned_data = preprocess_file(file_path)

    # Parse the cleaned JSON data
    try:
        data = json.loads(cleaned_data)
    except json.JSONDecodeError as e:
        print(f"Error decoding JSON: {e}")
        return

    # Iterate through the parsed data and save tweets to text files
    for index, item in enumerate(data):
        like = item.get("like", {})
        full_text = like.get("fullText", "")
        expanded_url = like.get("expandedUrl", "")

        # Use first chars as filename, fallback to index if empty
        if full_text:
            base_filename = full_text[:120]
            filename = sanitize_filename(base_filename) + ".txt"
        else:
            filename = f"tweet_{index + 1}.txt"

        # Create a text file for each tweet in the twitter-likes directory
        file_path = os.path.join(output_dir, filename)
        with open(file_path, "w", encoding="utf-8") as file:
            file.write(full_text + "\n")  # Write full text
            file.write(expanded_url)  # Write expanded URL

    print(f"Tweets have been saved to text files in the '{output_dir}' directory.")


# Modify path to your file
file_path = "./data/like.js"
save_tweets_to_files(file_path)
```

## Importing the data

Just import the created twitter-likes directory containing your likes as text files using [Digger Solo](https://solo.digger.lol) and dig in! Use the powerful search to find specific tweets or use the semantic data map to freely explore all of your likes clustered by content similarity.

#tutorial
