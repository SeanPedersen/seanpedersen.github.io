---
title: 'Analyze Your Telegram Chats'
date: '2025-04-17'
---
A small tutorial how you can easily explore your Telegram (<https://desktop.telegram.org/>) chat messages using [Digger Solo](https://solo.digger.lol).

Export the messages of a Telegram chat by clicking the three dots (top right corner) and select "Export chat history". Next select as export format JSON (not HTML).

Open a console and change directory into the chat export directory containing the result.json file. Next open up a Python 3 interpreter: `ipython3` and paste following code. Hit enter - et voila! All your messages have been converted into files in the directory messages which you can now import and explore with Digger Solo.

```python
import json
import os
import re

def merge_text(text):
    if isinstance(text, list):
        merged = ""
        for part in text:
            if isinstance(part, dict) and part.get("type") == "link":
                merged += part.get("text", "")
            elif isinstance(part, str):
                merged += part
        return merged
    elif isinstance(text, str):
        return text
    return ""

def safe_filename(s, maxlen=100):
    # Remove/replace unsafe characters and trim to maxlen
    s = re.sub(r'[\\/*?:"<>|]', '_', s)  # Replace filesystem-forbidden chars
    return s[:maxlen] or "empty_message"

def save_messages(messages):
    os.makedirs("messages", exist_ok=True)
    for msg in messages:
        content = merge_text(msg.get("text", ""))
        filename = safe_filename(content)
        filepath = os.path.join("messages", f"{filename}.txt")
        with open(filepath, "w", encoding="utf-8") as f:
            f.write(content)

if __name__ == "__main__":
    with open('result.json', 'r', encoding='utf-8') as f:
        data = json.load(f)
    save_messages(data["messages"])
```
