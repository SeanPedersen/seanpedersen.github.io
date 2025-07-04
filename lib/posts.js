import fs from 'fs'
import path from 'path'
import matter from 'gray-matter'
import remark from 'remark'
import html from 'remark-html'
import remarkPrism from 'remark-prism';

const postsDirectory = path.join(process.cwd(), 'posts')

// Function to extract hashtag-style tags from the last line of a markdown file
function extractTags(content) {
  const lines = content.split('\n').filter(line => line.trim() !== '');
  if (lines.length === 0) return [];

  const lastLine = lines[lines.length - 1].trim();
  // Check if the last line contains hashtag-style tags (#tag-1 #tag-2)
  if (lastLine.includes('#')) {
    const tagMatches = lastLine.match(/#[a-zA-Z0-9_-]+/g);
    if (tagMatches) {
      return tagMatches.map(tag => tag.substring(1)); // Remove the # prefix
    }
  }
  return [];
}

// runs on the server-side
export function getSortedPostsData() {
  // Get file names under /posts
  const fileNames = fs.readdirSync(postsDirectory)
  const allPostsData = fileNames.map(fileName => {
    // Remove ".md" from file name to get id
    const id = fileName.replace(/\.md$/, '')

    // Read markdown file as string
    const fullPath = path.join(postsDirectory, fileName)
    const fileContents = fs.readFileSync(fullPath, 'utf8')

    // Use gray-matter to parse the post metadata section
    const matterResult = matter(fileContents)

    // Extract tags from the last line
    const tags = extractTags(fileContents)

    // Combine the data with the id and tags
    return {
      id,
      tags,
      ...matterResult.data
    }
  })
  // Sort posts by date
  return allPostsData.sort((a, b) => {
    if (a.date < b.date) {
      return 1
    } else {
      return -1
    }
  })
}

// Get all unique tags from all posts
export function getAllTags() {
  const allPosts = getSortedPostsData();
  const allTags = new Set();

  allPosts.forEach(post => {
    if (post.tags && Array.isArray(post.tags)) {
      post.tags.forEach(tag => allTags.add(tag));
    }
  });

  return Array.from(allTags).sort();
}

export function getAllPostIds() {
  const fileNames = fs.readdirSync(postsDirectory)

  // Returns an array that looks like this:
  // [
  //   {
  //     params: {
  //       id: 'ssg-ssr'
  //     }
  //   },
  //   {
  //     params: {
  //       id: 'pre-rendering'
  //     }
  //   }
  // ]
  return fileNames.map(fileName => {
    return {
      params: {
        id: fileName.replace(/\.md$/, '')
      }
    }
  })
}

// Get related posts by tag (excluding the current post)
export function getRelatedPostsByTag(currentPostId, tag, limit = 3, maxCandidates = 10) {
  const allPosts = getSortedPostsData();

  // Filter posts that have the specified tag and exclude the current post
  const taggedPosts = allPosts.filter(post =>
    post.id !== currentPostId &&
    post.tags &&
    post.tags.includes(tag)
  );

  // If we don't have more posts than the limit, return them all
  if (taggedPosts.length <= limit) {
    return {
      posts: taggedPosts,
      hasMorePosts: false
    };
  }

  // Get a limited number of candidates (sorted by date)
  const candidates = taggedPosts.slice(0, Math.min(taggedPosts.length, maxCandidates));

  return {
    posts: candidates,
    hasMorePosts: candidates.length > limit
  };
}

export async function getPostData(id) {
  const fullPath = path.join(postsDirectory, `${id}.md`)
  const fileContents = fs.readFileSync(fullPath, 'utf8')

  // Use gray-matter to parse the post metadata section
  const matterResult = matter(fileContents)

  // Use remark to convert markdown into HTML string
  const processedContent = await remark()
    .use(remarkPrism) // Apply remark-prism before converting to HTML
    .use(html, { sanitize: false }) // Convert to HTML after prism, disable sanitization
    .process(matterResult.content)
  let contentHtml = processedContent.toString()

  // Decode HTML entities that get incorrectly encoded in code blocks
  contentHtml = contentHtml
    // Handle triple and double-encoded entities first
    .replace(/&amp;#x26;gt;/g, '>')
    .replace(/&amp;#x26;lt;/g, '<')
    .replace(/&#x26;gt;/g, '>')
    .replace(/&#x26;lt;/g, '<')
    // Handle standard HTML entities
    .replace(/&lt;/g, '<')
    .replace(/&gt;/g, '>')
    // Handle other double-encoded entities
    .replace(/&#x26;amp;/g, '&')
    .replace(/&#x26;quot;/g, '"')
    .replace(/&#x26;#39;/g, "'")
    .replace(/&#x26;#x27;/g, "'")
    .replace(/&#x26;#x2F;/g, '/')
    .replace(/&#x26;#x3D;/g, '=')

  // Re-encode only standalone angle brackets that could be interpreted as HTML tags
  // while preserving Prism.js syntax highlighting spans
  contentHtml = contentHtml.replace(/<code[^>]*>([\s\S]*?)<\/code>/g, (match, codeContent) => {
    // Encode angle brackets in generic type syntax (e.g., <Store>, <T>, etc.)
    const encodedContent = codeContent
      .replace(/(\w+)<(\w+)>/g, '$1&lt;$2&gt;')  // Match word<word> patterns
      .replace(/(<)(\w+)(>)/g, '&lt;$2&gt;');    // Match standalone <word> patterns
    return match.replace(codeContent, encodedContent);
  });

  // Also handle pre blocks with code  
  contentHtml = contentHtml.replace(/<pre[^>]*><code[^>]*>([\s\S]*?)<\/code><\/pre>/g, (match, codeContent) => {
    // Encode angle brackets in generic type syntax (e.g., <Store>, <T>, etc.)
    const encodedContent = codeContent
      .replace(/(\w+)<(\w+)>/g, '$1&lt;$2&gt;')  // Match word<word> patterns
      .replace(/(<)(\w+)(>)/g, '&lt;$2&gt;');    // Match standalone <word> patterns
    return match.replace(codeContent, encodedContent);
  });

  // Extract tags from the last line
  const tags = extractTags(fileContents)

  // Combine the data with the id, contentHtml, and tags
  return {
    id,
    contentHtml,
    tags,
    ...matterResult.data
  }
}

