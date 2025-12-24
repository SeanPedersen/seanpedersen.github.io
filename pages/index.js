import Head from 'next/head'
import Link from 'next/link'
import { useState, useEffect } from 'react' // Import useEffect
import Layout, { siteTitle } from '../components/layout'
import utilStyles from '../styles/utils.module.css'
import { getSortedPostsData, getAllTags } from '../lib/posts'
import { useTheme } from '../contexts/ThemeContext'; // Import useTheme
import Search from '../components/Search'

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
  const [searchExpanded, setSearchExpanded] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');
  const [searchData, setSearchData] = useState(null);
  const [isLoadingSearch, setIsLoadingSearch] = useState(false);

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

  // Load search data from RSS when search is first used
  useEffect(() => {
    if (searchQuery && !searchData && !isLoadingSearch) {
      setIsLoadingSearch(true);

      fetch('/rss.xml')
        .then(response => response.text())
        .then(xmlText => {
          const parser = new DOMParser();
          const xmlDoc = parser.parseFromString(xmlText, 'text/xml');
          const items = xmlDoc.querySelectorAll('item');

          const posts = Array.from(items).map(item => {
            const title = item.querySelector('title')?.textContent || '';
            const link = item.querySelector('link')?.textContent || '';
            const contentEncoded = item.querySelector('encoded')?.textContent || '';
            const categories = Array.from(item.querySelectorAll('category')).map(
              cat => cat.textContent
            );

            // Strip HTML tags from content for searching
            const tempDiv = document.createElement('div');
            tempDiv.innerHTML = contentEncoded;
            const textContent = tempDiv.textContent || tempDiv.innerText || '';

            // Extract post ID from link
            const urlObj = new URL(link);
            const id = urlObj.pathname.split('/').filter(Boolean).pop();

            return {
              id,
              title,
              content: textContent,
              categories,
              searchText: `${title} ${textContent} ${categories.join(' ')}`.toLowerCase()
            };
          });

          setSearchData(posts);
          setIsLoadingSearch(false);
        })
        .catch(error => {
          console.error('Failed to load RSS feed:', error);
          setIsLoadingSearch(false);
        });
    }
  }, [searchQuery, searchData, isLoadingSearch]);

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

  // Highlight matching text in title
  const highlightTitle = (title, query) => {
    if (!query.trim()) return title;

    const lowerTitle = title.toLowerCase();
    const lowerQuery = query.toLowerCase();
    const index = lowerTitle.indexOf(lowerQuery);

    if (index === -1) return title;

    const before = title.slice(0, index);
    const match = title.slice(index, index + query.length);
    const after = title.slice(index + query.length);

    return (
      <>
        {before}
        <mark style={{
          backgroundColor: 'var(--primary-color)',
          color: 'var(--color-background)',
          padding: '2px 4px',
          borderRadius: '3px'
        }}>
          {match}
        </mark>
        {after}
      </>
    );
  };

  // Filter posts by selected tag and search query
  const filteredPosts = (() => {
    // If no search query, filter by tag
    if (!searchQuery.trim()) {
      return allPostsData.filter(post => {
        return !selectedTag || (post.tags && post.tags.includes(selectedTag));
      });
    }

    // When searching, search ALL posts regardless of selected tag
    const query = searchQuery.toLowerCase();
    const posts = allPostsData;

    // If search data is loaded, use full-text search with scoring
    if (searchData) {
      const scoredPosts = posts
        .map(post => {
          const postSearchData = searchData.find(p => p.id === post.id);
          if (!postSearchData) return null;

          let score = 0;
          let titleMatch = false;

          const lowerTitle = postSearchData.title.toLowerCase();
          const lowerContent = postSearchData.content.toLowerCase();

          // Check if title matches
          if (lowerTitle.includes(query)) {
            // Count occurrences in title (worth 10 points each)
            const titleMatches = (lowerTitle.match(new RegExp(query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'g')) || []).length;
            score += titleMatches * 10;
            titleMatch = true;
          }

          // Check if content matches
          if (lowerContent.includes(query)) {
            // Count occurrences in content (worth 1 point each)
            const contentMatches = (lowerContent.match(new RegExp(query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'g')) || []).length;
            score += contentMatches;
          }

          // Check if categories match
          postSearchData.categories.forEach(cat => {
            if (cat.toLowerCase().includes(query)) {
              score += 5;
            }
          });

          // Only include posts with matches
          if (score === 0) return null;

          return { ...post, score, titleMatch };
        })
        .filter(Boolean)
        .sort((a, b) => b.score - a.score); // Sort by score (highest first)

      return scoredPosts;
    }

    // Fallback to title/tag search if RSS not loaded yet
    const scoredPosts = posts
      .map(post => {
        let score = 0;
        let titleMatch = false;

        if (post.title.toLowerCase().includes(query)) {
          score += 10;
          titleMatch = true;
        }

        if (post.tags && post.tags.some(tag => tag.toLowerCase().includes(query))) {
          score += 5;
        }

        if (score === 0) return null;

        return { ...post, score, titleMatch };
      })
      .filter(Boolean)
      .sort((a, b) => b.score - a.score);

    return scoredPosts;
  })()

  return (
    <div className={utilStyles.flexer}>
      <Layout home>
        <Head>
          <title>{siteTitle}</title>
        </Head>
        <section className={utilStyles.headingMd}>
          <p>Machine Learning / Data Privacy / Cybernetics / Memetics</p>
        </section>

        <section className={`${utilStyles.headingMd} ${utilStyles.padding1px}`}>
          {/* Search and Tags Filter Container */}
          <div className={`${utilStyles.searchTagsWrapper} ${searchExpanded ? utilStyles.searchExpanded : ''}`}>
            {/* Search */}
            <Search
              isExpanded={searchExpanded}
              onToggle={() => setSearchExpanded(!searchExpanded)}
              onSearch={(query) => setSearchQuery(query)}
            />

            {/* Tags Filter */}
            <div className={`${utilStyles.tagsContainer} ${searchExpanded ? utilStyles.tagsHidden : ''}`}>
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
          </div>

          <ul className={utilStyles.list}>
            {filteredPosts.map(({ id, date, title, tags, titleMatch }) => (
              <li className={utilStyles.listItem} key={id}>
                <Link href={`/posts/${id}`}>
                  {titleMatch && searchQuery ? highlightTitle(title, searchQuery) : title}
                </Link>
                {tags && tags.length > 0 && (
                  <small className={utilStyles.lightText}>
                    {' •'}
                    <span className={utilStyles.postTags}>
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
                  </small>
                )}
              </li>
            ))}
          </ul>
        </section>
      </Layout>
      <footer className={utilStyles.footer}>
        <p>Copy©at ᓚᘏᗢ {new Date().getFullYear()} | All lights served .:.</p>
      </footer>
      {/* Theme Toggle Button (always mounted; fades via CSS) */}
      <button
        onClick={toggleTheme}
        className={`themeToggleButton ${showToggle ? '' : 'isHidden'}`}
        aria-label="Toggle theme"
        aria-hidden={showToggle ? 'false' : 'true'}
        tabIndex={showToggle ? 0 : -1}
        title={`Switch to ${theme === 'dark' ? 'light' : 'dark'} mode`}
      >
        {hasMounted ? (
          theme === 'dark' ? (
            // Sun icon for dark theme (switch to light)
            <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <circle cx="12" cy="12" r="4" />
              <path d="M12 1v4M12 19v4M4.22 4.22l2.83 2.83M16.95 16.95l2.83 2.83M1 12h4M19 12h4M4.22 19.78l2.83-2.83M16.95 7.05l2.83-2.83" />
            </svg>
          ) : (
            // Moon icon for light theme (switch to dark)
            <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" />
            </svg>
          )
        ) : null}
      </button>
    </div>
  )
}
