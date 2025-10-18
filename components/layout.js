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
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <meta
          name="description"
          content="Another place for thought infusion"
        />
        <meta
          property="og:image"
          content="https://seanpedersen.github.io/images/sierpinski-twitter-square.png"
        />
        <meta name="og:title" content={title || siteTitle} />
        <meta name="twitter:card" content="summary" />
        <meta name="twitter:image" content="https://seanpedersen.github.io/images/sierpinski-twitter-square.png" />
        <link rel="alternate" type="application/rss+xml" href="/rss.xml" title="RSS Feed" />
      </Head>
      <header className={styles.header}>
        {home ? (
          <div style={{
            display: "flex",
            flexDirection: "row",
            alignItems: "center",
            gap: "1rem",
            padding: "1rem",
            maxWidth: "100%"
          }}>
            <img
              src="/images/profile.webp"
              className={`${styles.headerHomeImage} ${utilStyles.borderCircle}`}
              alt={name}
            />
            <div style={{ display: "flex", flexDirection: "column", alignItems: "flex-start" }}>
              <h1 className={`${utilStyles.heading2Xl} ${utilStyles.nameBreak}`}>
                <span>Sean</span>
                <span>Pedersen</span>
              </h1>
              <div className={styles.linkRow}>
                <a href="/rss.xml" className={styles.iconLink} aria-label="RSS" title="RSS Feed">
                  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" class="scale-125 stroke-accent stroke-3 rtl:-rotate-90"><path stroke="none" d="M0 0h24v24H0z" fill="none"></path><path d="M5 19m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0"></path><path d="M4 4a16 16 0 0 1 16 16"></path><path d="M4 11a9 9 0 0 1 9 9"></path></svg>
                </a>
                <div className={styles.socialLinks}>
                  <a
                    href="https://github.com/SeanPedersen"
                    className={styles.iconLink}
                    aria-label="GitHub"
                    title="GitHub"
                    target="_blank"
                    rel="noreferrer noopener"
                  >
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="inline-block size-6 scale-125 fill-transparent stroke-current stroke-2 opacity-90 group-hover:fill-transparent sm:scale-110"><path stroke="none" d="M0 0h24v24H0z" fill="none"></path><path d="M9 19c-4.3 1.4 -4.3 -2.5 -6 -3m12 5v-3.5c0 -1 .1 -1.4 -.5 -2c2.8 -.3 5.5 -1.4 5.5 -6a4.6 4.6 0 0 0 -1.3 -3.2a4.2 4.2 0 0 0 -.1 -3.2s-1.1 -.3 -3.5 1.3a12.3 12.3 0 0 0 -6.2 0c-2.4 -1.6 -3.5 -1.3 -3.5 -1.3a4.2 4.2 0 0 0 -.1 3.2a4.6 4.6 0 0 0 -1.3 3.2c0 4.6 2.7 5.7 5.5 6c-.6 .6 -.6 1.2 -.5 2v3.5"></path></svg>
                  </a>
                  <a
                    href="https://x.com/SeanPedersen96"
                    className={styles.iconLink}
                    aria-label="X (Twitter)"
                    title="X (Twitter)"
                    target="_blank"
                    rel="noreferrer noopener"
                  >
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="inline-block size-6 scale-125 fill-transparent stroke-current stroke-2 opacity-90 group-hover:fill-transparent sm:scale-110"><path stroke="none" d="M0 0h24v24H0z" fill="none"></path><path d="M4 4l11.733 16h4.267l-11.733 -16z"></path><path d="M4 20l6.768 -6.768m2.46 -2.46l6.772 -6.772"></path></svg>
                  </a>
                </div>
              </div>
            </div>
          </div>
        ) : (
          <div style={{
            display: "flex",
            alignItems: "center",
            gap: "1rem",
            padding: "0.5rem"
          }}>
            <h2 className={utilStyles.headingLg}>
              <Link href="/" className={utilStyles.colorInherit}>
                {siteTitle}
              </Link>
            </h2>
          </div>
        )}
      </header>
      <main className={`${styles.main} ${home ? styles.homePage : ''}`}>{children}</main>
    </div >
  )
}
