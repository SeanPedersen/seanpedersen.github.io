import Head from 'next/head'
import Link from 'next/link'
import Date from '../components/date'
import Layout, { siteTitle } from '../components/layout'
import utilStyles from '../styles/utils.module.css'
import { getSortedPostsData } from '../lib/posts'

export async function getStaticProps() {
  const allPostsData = getSortedPostsData()
  return {
    props: {
      allPostsData
    }
  }
}

export default function Home({ allPostsData }) {
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
              Building <a href="https://solo.digger.lol/">Digger Solo</a>: AI powered File Explorer
            </p>
          </div>
        </section>
        <section className={`${utilStyles.headingMd} ${utilStyles.padding1px}`}>
          <h2 className={utilStyles.headingLg}>Blog</h2>
          <ul className={utilStyles.list}>
            {allPostsData.map(({ id, date, title }) => (
              <li className={utilStyles.listItem} key={id}>
                <Link href={`/posts/${id}`}>
                  {title}
                </Link>
                <br />
                <small className={utilStyles.lightText}>
                  <Date dateString={date} />
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
