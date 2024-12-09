import Head from 'next/head'
import styles from './layout.module.css'
import utilStyles from '../styles/utils.module.css'
import Link from 'next/link'

const name = 'Sean Pedersen'
export const siteTitle = "Sean's Blog"

export default function Layout({ children, home, title }) {
  return (
    <div className={styles.container}>
      <Head>
        <link rel="icon" href="/favicon.ico" />
        <meta
          name="description"
          content="Another place for thought infusion"
        />
        <meta
          property="og:image"
          content="/images/site-image.svg"
        />
        <meta name="og:title" content={title || siteTitle} />
        <meta name="twitter:card" content="summary" />
      </Head>
      <header className={styles.header}>
        {home ? (
          <div style={{
            display: "flex",
            alignItems: "center",
            gap: "20px"
          }}>
            <img
              src="/images/profile.jpg"
              className={`${styles.headerHomeImage} ${utilStyles.borderCircle}`}
              alt={name}
            />
            <h1 className={utilStyles.heading2Xl}>{name}</h1>
          </div>
        ) : (
          <div style={{ display: "flex", alignItems: "center", gap: "20px" }}>
            <Link href="/">
              <img
                src="/images/profile.jpg"
                className={`${styles.headerImage} ${utilStyles.borderCircle}`}
                alt={name}
              />
            </Link>
            <h2 className={utilStyles.headingLg}>
              <Link href="/">
                <p className={utilStyles.colorInherit}>{siteTitle}</p>
              </Link>
            </h2>
          </div>
        )
        }
      </header >
      <main className={styles.main}>{children}</main>
      {
        !home && (
          <div className={styles.backToHome}>
            <Link href="/">
              <p>← Back to home</p>
            </Link>
          </div>
        )
      }
    </div >
  )
}
