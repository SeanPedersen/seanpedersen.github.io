import Layout from '../../components/layout'
import styles from '../../components/layout.module.css'
import { getAllPostIds, getPostData, getRelatedPostsByTag } from '../../lib/posts'
import Head from 'next/head'
import Date from '../../components/date'
import utilStyles from '../../styles/utils.module.css'
import TableOfContents from '../../components/TableOfContents'
import Link from 'next/link'
import { useState, useEffect } from 'react'

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
