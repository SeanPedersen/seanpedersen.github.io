import fs from 'fs';
import path from 'path';
import { format } from 'date-fns';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Read CSS files at build time
const globalCSS = fs.readFileSync(path.join(__dirname, '../styles/global.css'), 'utf-8');
const utilsCSS = fs.readFileSync(path.join(__dirname, '../styles/utils.module.css'), 'utf-8');
let layoutCSS = fs.readFileSync(path.join(__dirname, '../styles/layout.module.css'), 'utf-8');
// Remove :global() wrappers from layout CSS
layoutCSS = layoutCSS.replace(/:global\((.*?)\)/g, '$1');
const prismCSS = fs.readFileSync(path.join(__dirname, '../node_modules/prismjs/themes/prism-tomorrow.css'), 'utf-8');

// Escape HTML to prevent XSS
function escapeHtml(unsafe) {
  return unsafe
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#039;");
}

// Format date consistently
function formatDate(dateString) {
  return format(new Date(dateString), 'MMMM d, yyyy');
}

// Generate inline theme initialization script (prevent FOUC)
function getThemeInitScript() {
  return `(function() {
  try {
    var storedTheme = document.cookie
      .split('; ')
      .find(function(row) { return row.startsWith('theme='); });

    var theme = storedTheme ? storedTheme.split('=')[1] :
      (window.matchMedia && window.matchMedia('(prefers-color-scheme: light)').matches ? 'light' : 'dark');

    // Inject style to set background immediately
    var style = document.createElement('style');
    style.innerHTML = theme === 'light'
      ? 'body{background-color:#f0f0f0!important;color:#111!important}'
      : 'body{background-color:#000!important;color:#fff!important}';
    document.head.appendChild(style);

    // Also set class for CSS variables
    if (theme === 'light') {
      document.documentElement.classList.add('light-theme-pending');
    }
  } catch (e) {}
})();`;
}

// Generate body theme class application script
function getThemeBodyScript() {
  return `(function() {
  try {
    // Move class from html to body now that body exists
    if (document.documentElement.classList.contains('light-theme-pending')) {
      document.documentElement.classList.remove('light-theme-pending');
      document.body.classList.add('light-theme');
    }
  } catch (e) {}
})();`;
}

// Generate PostHog initialization script (using lightweight posthog-lite.js)
function getPostHogScript() {
  return `(function() {
  if (typeof window === 'undefined') return;

  // Load PostHog Lite
  var script = document.createElement('script');
  script.src = '/js/posthog-lite.js';
  script.onload = function() {
    if (window.PostHogLite) {
      window.posthog = new window.PostHogLite('phc_9XPlyPALuefIMAMSfsvBk4jVuSelJyjl7xwhXigkHAP', {
        host: 'https://us.i.posthog.com',
        person_profiles: 'identified_only',
        capturePageview: true
      });
    }
  };
  document.head.appendChild(script);
})();`;
}

// Generate the index page HTML
function generateIndexHTML(allPostsData, allTags) {
  const posts = allPostsData.map(post => {
    const tags = post.tags && Array.isArray(post.tags) ? post.tags : [];
    return `    <li class="listItem" data-id="${escapeHtml(post.id)}" data-tags="${escapeHtml(JSON.stringify(tags))}">
      <a href="/posts/${escapeHtml(post.id)}.html">
        ${escapeHtml(post.title)}
      </a>
      ${tags.length > 0 ? `<small class="lightText post-tags-container">
        •<span class="postTags">
          ${tags.map((tag, idx) =>
      `<span><a href="#${escapeHtml(tag)}" onclick="return false;">${escapeHtml(tag)}</a>${idx < tags.length - 1 ? ', ' : ''}</span>`
    ).join('')}
        </span>
      </small>` : ''}
    </li>`;
  }).join('\n');

  const tagButtons = allTags.map(tag =>
    `<span class="tag" data-tag="${escapeHtml(tag)}">${escapeHtml(tag)}</span>`
  ).join('\n          ');

  return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Sean's Blog</title>
  <meta name="description" content="Another place for thought infusion">
  <meta property="og:image" content="https://seanpedersen.github.io/images/sierpinski-twitter-square.png">
  <meta name="og:title" content="Sean's Blog">
  <meta name="twitter:card" content="summary">
  <meta name="twitter:image" content="https://seanpedersen.github.io/images/sierpinski-twitter-square.png">
  <link rel="icon" href="/favicon.ico">
  <link rel="alternate" type="application/rss+xml" href="/rss.xml" title="RSS Feed">
  <style>${globalCSS}${layoutCSS}${utilsCSS}</style>
  <script>${getThemeInitScript()}</script>
</head>
<body>
  <script>${getThemeBodyScript()}</script>
  <div class="flexer">
    <div class="container">
      <header class="header">
        <div class="headerContainer">
          <div class="speechBubble">
            Building <a href="https://solo.digger.lol/" target="_blank" rel="noopener noreferrer">Digger Solo</a>
          </div>
          <div class="profileContainer">
            <img
              src="/images/profile.webp"
              class="headerHomeImage borderCircle"
              alt="Sean Pedersen"
            />
            <div style="display: flex; flex-direction: column; align-items: flex-start;">
              <h1 class="heading2Xl nameBreak">
                <span>Sean</span>
                <span>Pedersen</span>
              </h1>
              <div class="linkRow">
                <a href="/rss.xml" class="iconLink" aria-label="RSS" title="RSS Feed">
                  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none"></path><path d="M5 19m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0"></path><path d="M4 4a16 16 0 0 1 16 16"></path><path d="M4 11a9 9 0 0 1 9 9"></path></svg>
                </a>
                <div class="socialLinks">
                  <a href="https://github.com/SeanPedersen" class="iconLink" aria-label="GitHub" title="GitHub" target="_blank" rel="noreferrer noopener">
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none"></path><path d="M9 19c-4.3 1.4 -4.3 -2.5 -6 -3m12 5v-3.5c0 -1 .1 -1.4 -.5 -2c2.8 -.3 5.5 -1.4 5.5 -6a4.6 4.6 0 0 0 -1.3 -3.2a4.2 4.2 0 0 0 -.1 -3.2s-1.1 -.3 -3.5 1.3a12.3 12.3 0 0 0 -6.2 0c-2.4 -1.6 -3.5 -1.3 -3.5 -1.3a4.2 4.2 0 0 0 -.1 3.2a4.6 4.6 0 0 0 -1.3 3.2c0 4.6 2.7 5.7 5.5 6c-.6 .6 -.6 1.2 -.5 2v3.5"></path></svg>
                  </a>
                  <a href="https://x.com/SeanPedersen96" class="iconLink" aria-label="X (Twitter)" title="X (Twitter)" target="_blank" rel="noreferrer noopener">
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none"></path><path d="M4 4l11.733 16h4.267l-11.733 -16z"></path><path d="M4 20l6.768 -6.768m2.46 -2.46l6.772 -6.772"></path></svg>
                  </a>
                </div>
              </div>
            </div>
          </div>
        </div>
      </header>
      <main class="main homePage">
        <section class="headingMd">
          <p>Machine Learning / Data Privacy / Cybernetics / Memetics</p>
        </section>
        <section class="headingMd padding1px">
          <!-- Search and Tags Filter Container -->
          <div class="searchTagsWrapper" id="searchTagsWrapper">
            <!-- Search -->
            <div id="searchContainer"></div>

            <!-- Tags Filter -->
            <div class="tagsContainer" id="tagsContainer">
              <span class="tag tagSelected" data-tag="">All</span>
          ${tagButtons}
            </div>
          </div>

          <ul class="list" id="postList">
${posts}
          </ul>
        </section>
      </main>
    </div>
    <footer class="footer">
      <p>Copy©at ᓚᘏᗢ ${new Date().getFullYear()} | All lights served .:.</p>
    </footer>
    <!-- Theme Toggle Button -->
    <button
      id="themeToggle"
      class="themeToggleButton"
      aria-label="Toggle theme"
      title="Toggle theme"
    >
      <svg id="themeIcon" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" strokeLinecap="round" strokeLinejoin="round">
        <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" />
      </svg>
    </button>
  </div>
  <script src="/js/theme.js" defer></script>
  <script>
    // Lazy load search.js on first interaction with search icon
    (function() {
      let searchLoaded = false;

      function loadSearchAndExpand() {
        if (searchLoaded) return;
        searchLoaded = true;

        // Mark that we should auto-expand after load
        window.__expandSearchOnLoad = true;

        const script = document.createElement('script');
        script.src = '/js/search.js';
        document.head.appendChild(script);
      }

      // Wait for DOM to be ready
      if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', function() {
          // Create initial search icon that triggers lazy load
          const container = document.getElementById('searchContainer');
          if (container) {
            container.innerHTML = \`
              <div class="searchContainer collapsed" id="searchContainerEl">
                <button class="searchIconButton" id="searchIconBtn" aria-label="Open search">
                  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <circle cx="11" cy="11" r="6" />
                    <line x1="19.59" y1="19.59" x2="15.24" y2="15.24" />
                  </svg>
                </button>
              </div>
            \`;

            document.getElementById('searchIconBtn').addEventListener('click', loadSearchAndExpand, { once: true });
          }
        });
      } else {
        // DOM already loaded
        const container = document.getElementById('searchContainer');
        if (container) {
          container.innerHTML = \`
            <div class="searchContainer collapsed" id="searchContainerEl">
              <button class="searchIconButton" id="searchIconBtn" aria-label="Open search">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <circle cx="11" cy="11" r="6" />
                  <line x1="19.59" y1="19.59" x2="15.24" y2="15.24" />
                </svg>
              </button>
            </div>
          \`;

          document.getElementById('searchIconBtn').addEventListener('click', loadSearchAndExpand, { once: true });
        }
      }
    })();
  </script>
  <script src="/js/tags.js" defer></script>
  <script src="/js/prefetch.js" defer></script>
  <script>${getPostHogScript()}</script>
</body>
</html>`;
}

// Generate a post page HTML
function generatePostHTML(postData, relatedPosts = []) {
  const title = escapeHtml(postData.title);
  const id = escapeHtml(postData.id);
  const titleId = postData.title.toLowerCase().replace(/[^a-z0-9]+/g, '-');

  // Create excerpt for meta description
  const excerpt = postData.contentHtml
    .replace(/<[^>]+>/g, ' ')
    .replace(/\s+/g, ' ')
    .trim()
    .slice(0, 160);

  // Check if post has table of contents (any h1-h3 headings)
  const hasTableOfContents = postData.contentHtml.includes('<h1') ||
    postData.contentHtml.includes('<h2') ||
    postData.contentHtml.includes('<h3');

  const relatedPostsHTML = relatedPosts.length > 0 ? `
          <!-- Related Posts Footer -->
          <footer class="relatedPostsFooter">
            <h3>Related Articles</h3>
            <ul class="relatedPostsList" style="padding-left: 0">
              ${relatedPosts.map(related => `
              <li class="relatedPostItem">
                <a href="/posts/${escapeHtml(related.id)}.html">
                  ${escapeHtml(related.title)}
                </a>
                <small class="lightText">
                  ${formatDate(related.date)}
                </small>
              </li>`).join('')}
            </ul>
          </footer>` : '';

  return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>${title} - Sean's Blog</title>
  <link rel="canonical" href="https://seanpedersen.github.io/posts/${id}">
  <meta name="description" content="${escapeHtml(excerpt)}">
  <meta name="keywords" content="${(postData.tags || []).map(t => escapeHtml(t)).join(', ')}">

  <meta property="og:type" content="article">
  <meta property="og:title" content="${title}">
  <meta property="og:description" content="${escapeHtml(excerpt)}">
  <meta property="og:url" content="https://seanpedersen.github.io/posts/${id}">
  <meta property="og:image" content="https://seanpedersen.github.io/images/sierpinski-twitter-square.png">

  <meta name="twitter:card" content="summary">
  <meta name="twitter:title" content="${title}">
  <meta name="twitter:description" content="${escapeHtml(excerpt)}">
  <meta name="twitter:image" content="https://seanpedersen.github.io/images/sierpinski-twitter-square.png">

  <link rel="icon" href="/favicon.ico">
  <link rel="alternate" type="application/rss+xml" href="/rss.xml" title="RSS Feed">
  <style>${globalCSS}${layoutCSS}${utilsCSS}${prismCSS}</style>
  <script>${getThemeInitScript()}</script>
  <script type="application/ld+json">
  ${JSON.stringify({
    '@context': 'https://schema.org',
    '@type': 'Article',
    headline: postData.title,
    datePublished: postData.date,
    dateModified: postData.date,
    author: [{ '@type': 'Person', name: 'Sean Pedersen' }],
    mainEntityOfPage: `https://seanpedersen.github.io/posts/${postData.id}`,
    image: ['https://seanpedersen.github.io/images/sierpinski-twitter-square.png'],
    description: excerpt
  })}
  </script>
</head>
<body>
  <script>${getThemeBodyScript()}</script>
  <div class="container">
    <header class="header">
      <div style="display: flex; align-items: center; gap: 1rem; padding: 0.5rem;">
        <h2 class="headingLg">
          <a href="/" class="colorInherit">Sean's Blog</a>
        </h2>
      </div>
    </header>
    <main class="main postPage ${hasTableOfContents ? '' : 'noToc'}">
      <article class="postContainer">
        ${hasTableOfContents ? '<div id="tocContainer"></div>' : ''}
        <span id="${titleId}" style="position: absolute; top: 0; visibility: hidden;" aria-hidden="true"></span>
        <h1 class="headingXl">${title}</h1>
        <div class="postMeta">
          <div class="lightText">
            ${formatDate(postData.date)}
          </div>
          <a
            href="https://github.com/SeanPedersen/seanpedersen.github.io/edit/main/posts/${id}.md"
            target="_blank"
            rel="noopener noreferrer"
            class="editOnGithubLink"
          >
            Edit on GitHub
          </a>
        </div>
        <div class="markdown-content" style="padding-bottom: 0.25rem; margin-bottom: 0;">
          ${postData.contentHtml}
        </div>
        ${relatedPostsHTML}

        <!-- Back to top area -->
        <footer class="backToTopFooter">
          <!-- Separator -->
          <hr
            aria-hidden="true"
            style="width: 100%; border: 0; border-top: 1px solid rgba(127,127,127,0.35); margin: 0.25rem 0;"
          />
          <div id="backToTopContainer"></div>
          <p>omnia mirari, gaudium explorandi .:.</p>
        </footer>
      </article>
    </main>
  </div>
  <script>
    window.__POST_DATA__ = {
      title: ${JSON.stringify(postData.title)},
      titleId: ${JSON.stringify(titleId)},
      hasTableOfContents: ${hasTableOfContents}
    };
  </script>
  <script src="/js/theme.js" defer></script>
  <script src="/js/post.js" defer></script>
  <script src="/js/prefetch.js" defer></script>
  <script>${getPostHogScript()}</script>
</body>
</html>`;
}

export {
  generateIndexHTML,
  generatePostHTML,
  escapeHtml,
  formatDate
};
