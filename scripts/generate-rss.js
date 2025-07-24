const fs = require('fs')
const path = require('path')
const matter = require('gray-matter')

const SITE_URL = 'https://seanpedersen.github.io'
const SITE_TITLE = "Sean's Blog"
const SITE_DESCRIPTION = 'Another place for thought infusion'
const AUTHOR = 'Sean Pedersen'

function escapeXml(unsafe) {
  return unsafe
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#39;")
}

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

function extractContentFromHtml(htmlFile) {
  try {
    const htmlContent = fs.readFileSync(htmlFile, 'utf8')

    // Extract JSON from __NEXT_DATA__ script tag
    const scriptMatch = htmlContent.match(/<script id="__NEXT_DATA__" type="application\/json">(.*?)<\/script>/)

    if (scriptMatch && scriptMatch[1]) {
      const jsonData = JSON.parse(scriptMatch[1])
      return jsonData.props.pageProps.postData.contentHtml || ''
    }

    return ''
  } catch (error) {
    console.warn(`Could not extract content from ${htmlFile}:`, error.message)
    return ''
  }
}

function getSortedPostsData() {
  const postsDirectory = path.join(process.cwd(), 'posts')
  const outDirectory = path.join(process.cwd(), 'out', 'posts')

  // Get file names under /posts
  const fileNames = fs.readdirSync(postsDirectory)
  const allPostsData = fileNames.map(fileName => {
    // Remove ".md" from file name to get id
    const id = fileName.replace(/\.md$/, '')

    // Read markdown file as string for metadata
    const fullPath = path.join(postsDirectory, fileName)
    const fileContents = fs.readFileSync(fullPath, 'utf8')

    // Use gray-matter to parse the post metadata section
    const matterResult = matter(fileContents)

    // Extract tags from the last line
    const tags = extractTags(fileContents)

    // Get HTML content from built file
    const htmlFile = path.join(outDirectory, `${id}.html`)
    const htmlContent = fs.existsSync(htmlFile) ? extractContentFromHtml(htmlFile) : ''

    // Combine the data with the id, tags, and HTML content
    return {
      id,
      tags,
      contentHtml: htmlContent,
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

function generateRSS() {
  const posts = getSortedPostsData()
  const now = new Date()
  const rssDate = now.toUTCString()

  let rss = `<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom" xmlns:content="http://purl.org/rss/1.0/modules/content/">
  <channel>
    <title>${escapeXml(SITE_TITLE)}</title>
    <description>${escapeXml(SITE_DESCRIPTION)}</description>
    <link>${SITE_URL}</link>
    <atom:link href="${SITE_URL}/rss.xml" rel="self" type="application/rss+xml" />
    <language>en</language>
    <pubDate>${rssDate}</pubDate>
    <lastBuildDate>${rssDate}</lastBuildDate>
    <generator>Custom RSS Generator</generator>
    <managingEditor>${escapeXml(AUTHOR)}</managingEditor>
    <webMaster>${escapeXml(AUTHOR)}</webMaster>
    <ttl>60</ttl>
`

  posts.forEach(post => {
    const postUrl = `${SITE_URL}/posts/${post.id}`
    const postDate = new Date(post.date).toUTCString()

    // Use the HTML content from the built file
    const htmlContent = post.contentHtml || ''

    rss += `    <item>
      <title>${escapeXml(post.title)}</title>
      <link>${postUrl}</link>
      <guid isPermaLink="true">${postUrl}</guid>
      <pubDate>${postDate}</pubDate>
      <author>${escapeXml(AUTHOR)}</author>
`

    if (post.tags && post.tags.length > 0) {
      post.tags.forEach(tag => {
        rss += `      <category>${escapeXml(tag)}</category>
`
      })
    }

    rss += `      <content:encoded><![CDATA[${htmlContent}]]></content:encoded>
    </item>
`
  })

  rss += `  </channel>
</rss>`

  return rss
}

async function main() {
  try {
    const rss = generateRSS()
    const outputPath = path.join(process.cwd(), 'out', 'rss.xml')
    // Assumes the output directory exists (npm run build should create it)
    fs.writeFileSync(outputPath, rss)
    console.log('RSS feed generated successfully at out/rss.xml')
  } catch (error) {
    console.error('Error generating RSS feed:', error)
    process.exit(1)
  }
}

main()