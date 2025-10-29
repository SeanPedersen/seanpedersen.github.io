import Layout from '../../components/layout'
import styles from '../../components/layout.module.css'
import { getAllPostIds, getPostData, getRelatedPostsByTag } from '../../lib/posts'
import Head from 'next/head'
import Date from '../../components/date'
import utilStyles from '../../styles/utils.module.css'
import TableOfContents from '../../components/TableOfContents'
import Link from 'next/link'
import { useState, useEffect } from 'react'
import ExpandableCodeBlock from '../../components/ExpandableCodeBlock'

export async function getStaticProps({ params }) {
  const postData = await getPostData(params.id)

  // Get related posts if there are tags
  let relatedPostCandidates = [];
  let hasMorePosts = false;

  if (postData.tags && postData.tags.length > 0) {
    // Use the first tag to find related posts
    const firstTag = postData.tags[0];
    const result = getRelatedPostsByTag(params.id, firstTag, 3, 10);
    relatedPostCandidates = result.posts;
    hasMorePosts = result.hasMorePosts;
  }

  return {
    props: {
      postData,
      relatedPostCandidates,
      hasMorePosts
    }
  }
}

export async function getStaticPaths() {
  const paths = getAllPostIds()
  return {
    paths,
    fallback: false
  }
}

export default function Post({ postData, relatedPostCandidates, hasMorePosts }) {
  const [relatedPosts, setRelatedPosts] = useState([]);
  const [showBackToTop, setShowBackToTop] = useState(false);

  // Randomize related posts on component mount and when navigating between posts
  useEffect(() => {
    if (relatedPostCandidates.length <= 3) {
      setRelatedPosts(relatedPostCandidates);
    } else {
      // Get the latest 2 posts
      const latestTwo = relatedPostCandidates.slice(0, 2);

      // Get one random from the rest
      const remainingPosts = relatedPostCandidates.slice(2);
      const randomIndex = Math.floor(Math.random() * remainingPosts.length);
      const randomPost = remainingPosts[randomIndex];

      setRelatedPosts([...latestTwo, randomPost]);
    }
  }, [relatedPostCandidates]);

  useEffect(() => {
    const checkScrollbar = () => {
      const el = document.scrollingElement || document.documentElement;
      setShowBackToTop(el.scrollHeight > el.clientHeight + 1);
    };

    checkScrollbar();

    const onResize = () => checkScrollbar();
    window.addEventListener('resize', onResize);

    // Re-check when images load (post content can change height)
    const imgs = Array.from(document.images || []);
    const onImgLoad = () => checkScrollbar();
    imgs.forEach(img => {
      if (!img.complete) img.addEventListener('load', onImgLoad, { once: true });
    });

    return () => {
      window.removeEventListener('resize', onResize);
      imgs.forEach(img => img.removeEventListener?.('load', onImgLoad));
    };
  }, [postData.id, relatedPostCandidates]);

  // Handle anchor links - scroll to element after page loads and images are ready
  useEffect(() => {
    const scrollToHash = () => {
      const hash = window.location.hash;
      if (hash) {
        const id = hash.replace('#', '');
        const element = document.getElementById(id);
        if (element) {
          // Use requestAnimationFrame to ensure DOM is ready
          requestAnimationFrame(() => {
            element.scrollIntoView({ behavior: 'smooth', block: 'start' });
          });
        }
      }
    };

    // Scroll after a short delay to ensure content is rendered
    const timeoutId = setTimeout(scrollToHash, 100);

    // Also scroll when images finish loading
    const imgs = Array.from(document.images || []);
    let loadedCount = 0;
    const onImgLoad = () => {
      loadedCount++;
      if (loadedCount === imgs.length) {
        scrollToHash();
      }
    };

    imgs.forEach(img => {
      if (img.complete) {
        loadedCount++;
      } else {
        img.addEventListener('load', onImgLoad, { once: true });
      }
    });

    // If all images were already loaded, scroll immediately
    if (loadedCount === imgs.length) {
      scrollToHash();
    }

    // Handle hash changes (when clicking TOC links)
    window.addEventListener('hashchange', scrollToHash);

    return () => {
      clearTimeout(timeoutId);
      window.removeEventListener('hashchange', scrollToHash);
      imgs.forEach(img => img.removeEventListener?.('load', onImgLoad));
    };
  }, [postData.id]);

  const processMarkdown = (content) => {
    // Add IDs to headings
    let processedContent = content.replace(
      /<h([1-6])>(.*?)<\/h[1-6]>/g,
      (match, level, text) => {
        // Strip HTML tags from text before creating ID to match TableOfContents behavior
        const rawText = text.replace(/<[^>]*>/g, '');
        const id = rawText.toLowerCase().replace(/[^a-z0-9]+/g, '-');
        return `<h${level} id="${id}">${text}</h${level}>`;
      }
    );

    // Wrap tables in a scrollable container div
    processedContent = processedContent.replace(
      /<table>/g,
      '<div class="table-wrapper"><table>'
    );
    processedContent = processedContent.replace(
      /<\/table>/g,
      '</table></div>'
    );

    // Convert hashtags to clickable links only on the last 2 lines
    const lines = processedContent.split('\n');
    if (lines.length > 0) {
      const startIndex = Math.max(0, lines.length - 2);
      for (let i = startIndex; i < lines.length; i++) {
        lines[i] = lines[i].replace(
          /#([a-zA-Z0-9_-]+)(?=\s|$|<|[^\w])/g,
          '<a href="/#$1" style="color: inherit;">#$1</a>'
        );
      }
      processedContent = lines.join('\n');
    }

    return processedContent;
  };

  const renderContentWithExpandableCodeBlocks = (content) => {
    const processedContent = processMarkdown(content);

    // Split content by Prism code blocks (which use <pre class="language-*"><code>)
    const parts = processedContent.split(/(<pre[^>]*class="[^"]*language-[^"]*"[^>]*>[\s\S]*?<\/pre>)/g);

    return parts.map((part, index) => {
      // Check if this part is a Prism code block
      const codeBlockMatch = part.match(/<pre[^>]*class="[^"]*language-[^"]*"[^>]*>([\s\S]*?)<\/pre>/);

      if (codeBlockMatch) {
        // Extract the inner content and count lines
        const innerContent = codeBlockMatch[1];
        // Remove HTML tags to count actual code lines
        const textContent = innerContent.replace(/<[^>]*>/g, '');
        const lineCount = textContent.split('\n').filter(line => line.trim()).length;

        return (
          <ExpandableCodeBlock key={index} lineCount={lineCount}>
            {part}
          </ExpandableCodeBlock>
        );
      }

      // Regular content
      return (
        <div key={index} dangerouslySetInnerHTML={{ __html: part }} />
      );
    });
  };

  const hasTableOfContents = postData.contentHtml.includes('<h2') || postData.contentHtml.includes('<h3');
  const titleId = postData.title.toLowerCase().replace(/[^a-z0-9]+/g, '-');

  return (
    <Layout title={postData.title}>
      <Head>
        <title>{postData.title}</title>
        <link rel="canonical" href={`https://seanpedersen.github.io/posts/${postData.id}`} />
        <meta name="description" content={postData.contentHtml.replace(/<[^>]+>/g, ' ').replace(/\s+/g, ' ').trim().slice(0, 160)} />
        <meta name="keywords" content={(postData.tags || []).join(', ')} />

        <meta property="og:type" content="article" />
        <meta property="og:title" content={postData.title} />
        <meta property="og:description" content={postData.contentHtml.replace(/<[^>]+>/g, ' ').replace(/\s+/g, ' ').trim().slice(0, 160)} />
        <meta property="og:url" content={`https://seanpedersen.github.io/posts/${postData.id}`} />
        <meta property="og:image" content="https://seanpedersen.github.io/images/sierpinski-twitter-square.png" />

        <meta name="twitter:card" content="summary" />
        <meta name="twitter:title" content={postData.title} />
        <meta name="twitter:description" content={postData.contentHtml.replace(/<[^>]+>/g, ' ').replace(/\s+/g, ' ').trim().slice(0, 160)} />
        <meta name="twitter:image" content="https://seanpedersen.github.io/images/sierpinski-twitter-square.png" />

        <script
          type="application/ld+json"
          dangerouslySetInnerHTML={{
            __html: JSON.stringify({
              '@context': 'https://schema.org',
              '@type': 'Article',
              headline: postData.title,
              datePublished: postData.date,
              dateModified: postData.date,
              author: [{ '@type': 'Person', name: 'Sean Pedersen' }],
              mainEntityOfPage: `https://seanpedersen.github.io/posts/${postData.id}`,
              image: ['https://seanpedersen.github.io/images/sierpinski-twitter-square.png'],
              description: postData.contentHtml.replace(/<[^>]+>/g, ' ').replace(/\s+/g, ' ').trim().slice(0, 160)
            })
          }}
        />
      </Head>
      <div className={`${styles.main} ${styles.postPage} ${!hasTableOfContents ? styles.noToc : ''}`}>
        {hasTableOfContents && (
          <TableOfContents
            content={postData.contentHtml}
            title={postData.title}
            titleId={titleId}
          />
        )}
        <article className={utilStyles.postContainer}>
          <h1 id={titleId} className={utilStyles.headingXl}>{postData.title}</h1>
          <div className={utilStyles.postMeta}>
            <div className={utilStyles.lightText}>
              <Date dateString={postData.date} />
            </div>
            <a
              href={`https://github.com/SeanPedersen/seanpedersen.github.io/edit/main/posts/${postData.id}.md`}
              target="_blank"
              rel="noopener noreferrer"
              className={utilStyles.editOnGithubLink}
            >
              Edit on GitHub
            </a>
          </div>
          <div
            className="markdown-content"
            style={{ paddingBottom: '0.25rem', marginBottom: 0 }}
          >
            {renderContentWithExpandableCodeBlocks(postData.contentHtml)}
          </div>

          {/* Related Posts Footer */}
          {relatedPosts && relatedPosts.length > 0 && (
            <footer className={utilStyles.relatedPostsFooter}>
              <h3>Related Articles</h3>
              <ul className={utilStyles.relatedPostsList} style={{ paddingLeft: 0 }}>
                {relatedPosts.map(({ id, title, date }) => (
                  <li key={id} className={utilStyles.relatedPostItem}>
                    <Link href={`/posts/${id}`}>
                      {title}
                    </Link>
                    <small className={utilStyles.lightText}>
                      <Date dateString={date} />
                    </small>
                  </li>
                ))}
              </ul>
            </footer>
          )}

          {/* Back to top area */}
          <footer className={utilStyles.backToTopFooter}>
            {/* Separator */}
            <hr
              aria-hidden="true"
              style={{
                width: '100%',
                border: 0,
                borderTop: '1px solid rgba(127,127,127,0.35)',
                margin: '0.25rem 0'
              }}
            />
            {showBackToTop && (
              <>

                <a href={`#${titleId}`}>↑ Back to top</a>
              </>
            )}
            <p>omnia mirari, gaudium explorandi .:.</p>
          </footer>
        </article>
      </div>
    </Layout>
  )
}
