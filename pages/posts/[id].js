import Layout from '../../components/layout'
import styles from '../../components/layout.module.css'
import { getAllPostIds, getPostData } from '../../lib/posts'
import Head from 'next/head'
import Date from '../../components/date'
import utilStyles from '../../styles/utils.module.css'
import { Prism } from 'prism-react-renderer'
import codeStyles from '../../styles/code.module.css'
import TableOfContents from '../../components/TableOfContents'


export async function getStaticProps({ params }) {
  const postData = await getPostData(params.id)
  return {
    props: {
      postData
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

export default function Post({ postData }) {
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
    <Layout>
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
        </article>
      </div>
    </Layout>
  )
}
