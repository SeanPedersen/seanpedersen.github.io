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
          content="/favicon.ico"
        />
        <meta name="og:title" content={title || siteTitle} />
        <meta name="twitter:card" content="summary" />
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
                <a href="/rss.xml" className={styles.rssLink} title="RSS Feed">
                  RSS
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
                    <svg className={styles.icon} viewBox="0 0 24 24" aria-hidden="true" focusable="false">
                      <path d="M12 .297C5.373.297 0 5.67 0 12.297c0 5.302 3.438 9.8 8.205 11.387.6.11.82-.26.82-.577 0-.285-.011-1.04-.017-2.042-3.338.726-4.042-1.61-4.042-1.61-.546-1.386-1.334-1.757-1.334-1.757-1.09-.745.083-.73.083-.73 1.205.085 1.84 1.238 1.84 1.238 1.07 1.834 2.807 1.304 3.492.997.108-.775.418-1.305.762-1.606-2.665-.304-5.466-1.332-5.466-5.93 0-1.31.469-2.382 1.236-3.222-.124-.303-.536-1.524.117-3.176 0 0 1.008-.322 3.3 1.23a11.5 11.5 0 0 1 6.004 0c2.29-1.552 3.297-1.23 3.297-1.23.655 1.652.243 2.873.119 3.176.77.84 1.235 1.912 1.235 3.222 0 4.61-2.806 5.624-5.48 5.922.43.372.814 1.102.814 2.222 0 1.604-.015 2.896-.015 3.29 0 .32.218.694.827.576C20.565 22.095 24 17.598 24 12.297 24 5.67 18.627.297 12 .297z" />
                    </svg>
                  </a>
                  <a
                    href="https://x.com/SeanPedersen96"
                    className={styles.iconLink}
                    aria-label="X (Twitter)"
                    title="X (Twitter)"
                    target="_blank"
                    rel="noreferrer noopener"
                  >
                    <svg className={styles.icon} viewBox="0 0 24 24" aria-hidden="true" focusable="false">
                      <path d="M18.244 2H21l-7.97 9.133L22 22h-7.68l-5.14-6.07L2.87 22H0l8.65-9.91L0 2h7.75l4.94 5.84L18.244 2zm-2.69 18h1.64L8.52 4H6.88l8.674 16z" />
                    </svg>
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
