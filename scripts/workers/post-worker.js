import { parentPort, workerData } from 'worker_threads';
import fs from 'fs';
import path from 'path';
import { getPostData, getRelatedPostsByTag } from '../../lib/posts.js';
import { generatePostHTML } from '../templates.js';

const { postId, postsDir } = workerData;

async function processPost() {
  try {
    const postData = await getPostData(postId);

    // Get related posts if there are tags
    let relatedPostCandidates = [];
    if (postData.tags && postData.tags.length > 0) {
      const firstTag = postData.tags[0];
      const result = getRelatedPostsByTag(postId, firstTag, 3, 10);
      relatedPostCandidates = result.posts;
    }

    // Randomly select 3 related posts (or fewer if not enough)
    let relatedPosts = [];
    if (relatedPostCandidates.length <= 3) {
      relatedPosts = relatedPostCandidates;
    } else {
      // Get the latest 2 posts
      const latestTwo = relatedPostCandidates.slice(0, 2);

      // Get one random from the rest
      const remainingPosts = relatedPostCandidates.slice(2);
      const randomIndex = Math.floor(Math.random() * remainingPosts.length);
      const randomPost = remainingPosts[randomIndex];

      relatedPosts = [...latestTwo, randomPost];
    }

    const postHTML = generatePostHTML(postData, relatedPosts);
    fs.writeFileSync(path.join(postsDir, `${postId}.html`), postHTML, 'utf8');

    parentPort.postMessage({ success: true, postId });
  } catch (error) {
    parentPort.postMessage({ success: false, postId, error: error.message });
  }
}

processPost();
