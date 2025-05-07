import Head from 'next/head'
import Link from 'next/link'
import { useState } from 'react'
import Date from '../components/date'
import Layout, { siteTitle } from '../components/layout'
import utilStyles from '../styles/utils.module.css'
import { getSortedPostsData, getAllTags } from '../lib/posts'

export async function getStaticProps() {
  const allPostsData = getSortedPostsData()
  const allTags = getAllTags()
  return {
    props: {
      allPostsData,
      allTags
    }
  }
}

export default function Home({ allPostsData, allTags }) {
  const [selectedTag, setSelectedTag] = useState(null)

  // Filter posts by selected tag
  const filteredPosts = selectedTag
    ? allPostsData.filter(post => post.tags && post.tags.includes(selectedTag))
    : allPostsData

  return (
    <div className={utilStyles.flexer}>
      <Layout home>
        <Head>
          <title>{siteTitle}</title>
        </Head>
        <section className={utilStyles.headingMd}>
          <p>Exploring AI and more. Stay in touch: <a href="https://twitter.com/SeanPedersen96" rel="noreferrer noopener" target="_blank">@SeanPedersen96</a></p>
          <div style={{ display: 'flex', alignItems: 'center', gap: '16px', margin: '12px 0' }}>
            <p style={{ margin: 0 }}>
              Building <a href="https://solo.digger.lol/">Digger Solo</a>: Digital Cartography for Your Files
            </p>
          </div>
        </section>

        <section className={`${utilStyles.headingMd} ${utilStyles.padding1px}`}>
          <h2 className={utilStyles.headingLg}>Blog</h2>

          {/* Tags Filter */}
          <div className={utilStyles.tagsContainer}>
            <span
              className={`${utilStyles.tag} ${selectedTag === null ? utilStyles.tagSelected : ''}`}
              onClick={() => setSelectedTag(null)}
            >
              All
            </span>
            {allTags.map(tag => (
              <span
                key={tag}
                className={`${utilStyles.tag} ${selectedTag === tag ? utilStyles.tagSelected : ''}`}
                onClick={() => setSelectedTag(tag)}
              >
                {tag}
              </span>
            ))}
          </div>

          <ul className={utilStyles.list}>
            {filteredPosts.map(({ id, date, title, tags }) => (
              <li className={utilStyles.listItem} key={id}>
                <Link href={`/posts/${id}`}>
                  {title}
                </Link>
                <br />
                <small className={utilStyles.lightText}>
                  <Date dateString={date} />
                  {tags && tags.length > 0 && (
                    <span className={utilStyles.postTags}>
                      {' • '}
                      {tags.map((tag, index) => (
                        <span key={tag} onClick={(e) => {
                          e.preventDefault();
                          setSelectedTag(tag);
                        }}>
                          <a href="#" onClick={(e) => e.preventDefault()}>
                            {tag}
                          </a>
                          {index < tags.length - 1 ? ', ' : ''}
                        </span>
                      ))}
                    </span>
                  )}
                </small>
              </li>
            ))}
          </ul>
        </section>
      </Layout>
      <footer className={utilStyles.footer}>
        <p>Check out my projects on <a href="https://github.com/SeanPedersen/" rel="noreferrer noopener" target="_blank">Github</a></p>
      </footer>
    </div>
  )
}
