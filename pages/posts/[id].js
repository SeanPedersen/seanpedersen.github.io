import Layout from '../../components/layout'
import styles from '../../components/layout.module.css'
import { getAllPostIds, getPostData, getRelatedPostsByTag } from '../../lib/posts'
import Head from 'next/head'
import Date from '../../components/date'
import utilStyles from '../../styles/utils.module.css'
import codeStyles from '../../styles/code.module.css'
import TableOfContents from '../../components/TableOfContents'
import Link from 'next/link'

export async function getStaticProps({ params }) {
  const postData = await getPostData(params.id)

  // Get related posts if there are tags
  let relatedPosts = []
  if (postData.tags && postData.tags.length > 0) {
    // Use the first tag to find related posts
    const firstTag = postData.tags[0]
    relatedPosts = getRelatedPostsByTag(params.id, firstTag, 3)
  }

  return {
    props: {
      postData,
      relatedPosts
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

export default function Post({ postData, relatedPosts }) {
  const escapeHtml = (unsafe) => {
    return unsafe
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;")
      .replace(/'/g, "&#039;");
  }

  const addHeadingIds = (content) => {
    return content.replace(
      /<h([1-6])>(.*?)<\/h[1-6]>/g,
      (match, level, text) => {
        const id = text.toLowerCase().replace(/[^a-z0-9]+/g, '-');
        return `<h${level} id="${id}">${text}</h${level}>`;
      }
    );
  };

  const hasTableOfContents = postData.contentHtml.includes('<h2') || postData.contentHtml.includes('<h3');

  return (
    <Layout title={postData.title}>
      <Head>
        <title>{postData.title}</title>
      </Head>
      <div className={`${styles.main} ${styles.postPage} ${!hasTableOfContents ? styles.noToc : ''}`}>
        {hasTableOfContents && <TableOfContents content={postData.contentHtml} />}
        <article className={utilStyles.postContainer}>
          <h1 className={utilStyles.headingXl}>{postData.title}</h1>
          <div className={utilStyles.lightText}>
            <Date dateString={postData.date} />
          </div>
          <div
            className="markdown-content"
            dangerouslySetInnerHTML={{
              __html: addHeadingIds(postData.contentHtml)
                .replace(
                  /<pre><code>(.*?)<\/code><\/pre>/gs,
                  (_, code) => `
                    <div class="${codeStyles.codeBlock}">
                      <pre>${escapeHtml(code)}</pre>
                    </div>
                  `
                )
                .replace(
                  /<code>(.*?)<\/code>/g,
                  (_, code) => `<code class="${codeStyles.inlineCode}">${escapeHtml(code)}</code>`
                )
            }}
          />

          {/* Related Posts Footer */}
          {relatedPosts && relatedPosts.length > 0 && (
            <footer className={utilStyles.relatedPostsFooter}>
              <h3>Related Articles</h3>
              <ul className={utilStyles.relatedPostsList}>
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
        </article>
      </div>
    </Layout>
  )
}
