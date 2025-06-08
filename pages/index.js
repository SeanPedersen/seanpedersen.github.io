import Head from 'next/head'
import Link from 'next/link'
import { useState, useEffect } from 'react' // Import useEffect
import Date from '../components/date'
import Layout, { siteTitle } from '../components/layout'
import utilStyles from '../styles/utils.module.css'
import { getSortedPostsData, getAllTags } from '../lib/posts'
import { useTheme } from '../contexts/ThemeContext'; // Import useTheme

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
  const { theme, toggleTheme } = useTheme(); // Use theme from context
  const [hasMounted, setHasMounted] = useState(false);

  useEffect(() => {
    setHasMounted(true);

    // Function to update selectedTag based on URL hash
    const updateTagFromHash = () => {
      const hash = window.location.hash.replace(/^#/, '');
      if (hash && allTags.includes(hash)) {
        setSelectedTag(hash);
      } else if (!hash) {
        setSelectedTag(null); // If hash is empty, select "All"
      }
    };

    updateTagFromHash(); // Set initial tag based on hash

    window.addEventListener('hashchange', updateTagFromHash);
    return () => {
      window.removeEventListener('hashchange', updateTagFromHash);
    };
  }, [allTags]); // Add allTags as a dependency

  // Update URL hash when selectedTag changes
  useEffect(() => {
    if (hasMounted) { // Ensure this runs only client-side after initial mount
      if (selectedTag) {
        window.location.hash = selectedTag;
      } else {
        // Remove hash if "All" is selected
        // Check if history.pushState is available (modern browsers)
        if (window.history.pushState) {
          window.history.pushState("", document.title, window.location.pathname + window.location.search);
        } else {
          // Fallback for older browsers
          window.location.hash = '';
        }
      }
    }
  }, [selectedTag, hasMounted]);

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
                          e.stopPropagation();
                          setSelectedTag(tag);
                        }}>
                          {/* Use a button or styled span for better accessibility if not a real link */}
                          <a href={`#${tag}`} onClick={(e) => { // Update href for semantic correctness
                            e.preventDefault();
                            setSelectedTag(tag);
                          }}>
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
      {/* Theme Toggle Button */}
      <button
        onClick={toggleTheme}
        className="themeToggleButton" // This class will be styled in global.css
        aria-label="Toggle theme"
        title={`Switch to ${theme === 'dark' ? 'light' : 'dark'} mode`}
      >
        {hasMounted ? (theme === 'dark' ? '☀️' : '🌙') : null}
      </button>
    </div>
  )
}
