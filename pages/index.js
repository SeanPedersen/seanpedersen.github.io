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
  const [showToggle, setShowToggle] = useState(true);

  useEffect(() => {
    setHasMounted(true);

    // Set initial tag from hash without creating history entry
    const hash = window.location.hash.replace(/^#/, '');
    if (hash && allTags.includes(hash)) {
      setSelectedTag(hash);
    }

    // Listen for hash changes (back/forward navigation)
    const handleHashChange = () => {
      const hash = window.location.hash.replace(/^#/, '');
      if (hash && allTags.includes(hash)) {
        setSelectedTag(hash);
      } else {
        setSelectedTag(null);
      }
    };

    // Handle scroll to hide/show theme toggle
    const handleScroll = () => {
      const currentScrollY = window.scrollY;
      
      if (currentScrollY <= 10) {
        // At the very top (within 10px)
        setShowToggle(true);
      } else {
        // Anywhere else, hide the toggle
        setShowToggle(false);
      }
    };

    window.addEventListener('hashchange', handleHashChange);
    window.addEventListener('scroll', handleScroll);
    
    return () => {
      window.removeEventListener('hashchange', handleHashChange);
      window.removeEventListener('scroll', handleScroll);
    };
  }, [allTags]);

  // Handle tag selection
  const handleTagSelect = (tag) => {
    setSelectedTag(tag);
    if (tag) {
      window.location.hash = tag;
    } else {
      // Use replaceState only when clearing the hash to avoid extra history entries
      if (window.history.replaceState) {
        window.history.replaceState(null, '', window.location.pathname + window.location.search);
      } else {
        window.location.hash = '';
      }
    }
  };

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
              onClick={() => handleTagSelect(null)}
            >
              All
            </span>
            {allTags.map(tag => (
              <span
                key={tag}
                className={`${utilStyles.tag} ${selectedTag === tag ? utilStyles.tagSelected : ''}`}
                onClick={() => handleTagSelect(tag)}
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
                        <span key={tag}>
                          <Link
                            href="#"
                            onClick={(e) => {
                              e.preventDefault();
                              e.stopPropagation();
                              handleTagSelect(tag);
                            }}
                          >
                            {tag}
                          </Link>
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
      {showToggle && (
        <button
          onClick={toggleTheme}
          className="themeToggleButton" // This class will be styled in global.css
          aria-label="Toggle theme"
          title={`Switch to ${theme === 'dark' ? 'light' : 'dark'} mode`}
        >
          {hasMounted ? (theme === 'dark' ? '☀️' : '🌙') : null}
        </button>
      )}
    </div>
  )
}
