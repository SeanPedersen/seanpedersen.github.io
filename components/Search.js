import { useState, useEffect, useRef, useCallback, useMemo } from 'react';
import Link from 'next/link';
import styles from './Search.module.css';

const Search = ({ isExpanded = false, onToggle = null }) => {
  const [query, setQuery] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [searchData, setSearchData] = useState(null);
  const [results, setResults] = useState([]);
  const [isFocused, setIsFocused] = useState(false);
  const [wrapperWidth, setWrapperWidth] = useState(0);
  const [resultsTop, setResultsTop] = useState(120);
  const inputRef = useRef(null);
  const containerRef = useRef(null);
  const inputWrapperRef = useRef(null);
  const debounceTimeoutRef = useRef(null);

  // Lazy load and parse RSS feed
  const loadSearchData = useCallback(async () => {
    if (searchData) return; // Already loaded

    setIsLoading(true);
    try {
      const response = await fetch('/rss.xml');
      const xmlText = await response.text();
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

        return {
          title,
          link,
          content: textContent,
          categories,
          searchText: `${title} ${textContent} ${categories.join(' ')}`.toLowerCase()
        };
      });

      setSearchData(posts);
    } catch (error) {
      console.error('Failed to load RSS feed:', error);
    } finally {
      setIsLoading(false);
    }
  }, [searchData]);

  // Perform search
  const performSearch = useCallback((searchQuery) => {
    if (!searchQuery.trim() || !searchData) {
      setResults([]);
      return;
    }

    const queryTerms = searchQuery.toLowerCase().split(/\s+/).filter(Boolean);

    const searchResults = searchData
      .map(post => {
        let score = 0;

        // Calculate relevance score
        queryTerms.forEach(term => {
          // Title matches are worth more
          if (post.title.toLowerCase().includes(term)) {
            score += 10;
          }
          // Content matches
          if (post.content.toLowerCase().includes(term)) {
            score += 1;
          }
          // Category matches
          if (post.categories.some(cat => cat.toLowerCase().includes(term))) {
            score += 5;
          }
        });

        return { ...post, score };
      })
      .filter(post => post.score > 0)
      .sort((a, b) => b.score - a.score)
      .slice(0, 10); // Limit to top 10 results

    setResults(searchResults);
  }, [searchData]);

  // Handle input change with debouncing
  const handleInputChange = useCallback((e) => {
    const value = e.target.value;
    setQuery(value);

    // Clear existing timeout
    if (debounceTimeoutRef.current) {
      clearTimeout(debounceTimeoutRef.current);
    }

    // Load data on first keystroke
    if (value && !searchData && !isLoading) {
      loadSearchData();
    }

    // Debounce search
    debounceTimeoutRef.current = setTimeout(() => {
      performSearch(value);
    }, 300);
  }, [searchData, isLoading, loadSearchData, performSearch]);

  // Cleanup timeout on unmount
  useEffect(() => {
    return () => {
      if (debounceTimeoutRef.current) {
        clearTimeout(debounceTimeoutRef.current);
      }
    };
  }, []);

  // Measure wrapper width and auto-focus input when expanded
  useEffect(() => {
    if (isExpanded) {
      // Measure parent wrapper width
      if (containerRef.current) {
        const wrapper = containerRef.current.parentElement;
        if (wrapper) {
          setWrapperWidth(wrapper.offsetWidth);
        }
      }

      // Calculate position for search results
      const updatePosition = () => {
        if (inputWrapperRef.current) {
          const rect = inputWrapperRef.current.getBoundingClientRect();
          setResultsTop(rect.bottom + 16);
        }
      };

      // Update position after animation
      setTimeout(updatePosition, 500);

      // Update on window resize
      window.addEventListener('resize', updatePosition);
      window.addEventListener('scroll', updatePosition);

      // Auto-focus input immediately - don't wait for animation
      if (inputRef.current) {
        inputRef.current.focus();
      }

      return () => {
        window.removeEventListener('resize', updatePosition);
        window.removeEventListener('scroll', updatePosition);
      };
    }
  }, [isExpanded]);

  // Keyboard shortcut: Ctrl/Cmd + K to focus search
  useEffect(() => {
    const handleKeyDown = (e) => {
      if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
        e.preventDefault();
        if (!isExpanded && onToggle) {
          onToggle();
        } else {
          inputRef.current?.focus();
        }
      }

      // Escape to clear and blur
      if (e.key === 'Escape' && isFocused) {
        setQuery('');
        setResults([]);
        if (onToggle && isExpanded) {
          onToggle(); // Collapse search
        } else {
          inputRef.current?.blur();
        }
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [isFocused, isExpanded, onToggle]);

  // Extract URL path from full URL
  const getPostPath = (url) => {
    try {
      const urlObj = new URL(url);
      return urlObj.pathname;
    } catch {
      return url;
    }
  };

  // Highlight matching terms in text
  const highlightText = (text, maxLength = 150) => {
    if (!query.trim()) return <span>{text.slice(0, maxLength)}</span>;

    const terms = query.toLowerCase().split(/\s+/).filter(Boolean);
    let excerpt = text;

    // Find first match to show context
    const lowerText = text.toLowerCase();
    let firstMatchIndex = -1;
    terms.forEach(term => {
      const index = lowerText.indexOf(term);
      if (index !== -1 && (firstMatchIndex === -1 || index < firstMatchIndex)) {
        firstMatchIndex = index;
      }
    });

    // Extract excerpt around first match
    if (firstMatchIndex !== -1) {
      const start = Math.max(0, firstMatchIndex - 50);
      const end = Math.min(text.length, firstMatchIndex + maxLength);
      excerpt = (start > 0 ? '...' : '') + text.slice(start, end) + (end < text.length ? '...' : '');
    } else {
      excerpt = text.slice(0, maxLength) + (text.length > maxLength ? '...' : '');
    }

    // Highlight matched terms
    const parts = [];
    let lastIndex = 0;
    const lowerExcerpt = excerpt.toLowerCase();

    // Create array of matches with positions
    const matches = [];
    terms.forEach(term => {
      let pos = 0;
      while ((pos = lowerExcerpt.indexOf(term, pos)) !== -1) {
        matches.push({ start: pos, end: pos + term.length });
        pos += term.length;
      }
    });

    // Sort and merge overlapping matches
    matches.sort((a, b) => a.start - b.start);
    const merged = [];
    matches.forEach(match => {
      if (merged.length === 0 || match.start > merged[merged.length - 1].end) {
        merged.push(match);
      } else {
        merged[merged.length - 1].end = Math.max(merged[merged.length - 1].end, match.end);
      }
    });

    // Build highlighted text
    merged.forEach((match, index) => {
      parts.push(excerpt.slice(lastIndex, match.start));
      parts.push(
        <mark key={index} className={styles.highlight}>
          {excerpt.slice(match.start, match.end)}
        </mark>
      );
      lastIndex = match.end;
    });
    parts.push(excerpt.slice(lastIndex));

    return <span>{parts}</span>;
  };

  // Handle search icon click when collapsed
  const handleIconClick = () => {
    if (onToggle) {
      onToggle();
    }
  };

  return (
    <div
      ref={containerRef}
      className={`${styles.searchContainer} ${isExpanded ? styles.expanded : styles.collapsed}`}
    >
      {!isExpanded ? (
        // Collapsed state: just show the icon
        <button
          className={styles.searchIconButton}
          onClick={handleIconClick}
          aria-label="Open search"
        >
          <svg
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
          >
            <circle cx="11" cy="11" r="6" />
            <line x1="19.59" y1="19.59" x2="15.24" y2="15.24" />
          </svg>
        </button>
      ) : (
        // Expanded state: show the full search bar
        <>
          <div
            ref={inputWrapperRef}
            className={styles.searchInputWrapper}
            style={{ '--wrapper-width': `${wrapperWidth}px` }}
          >
            <svg
              className={styles.searchIcon}
              width="1em"
              height="1em"
              viewBox="0 0 24 24"
              aria-hidden="true"
              focusable="false"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
              style={{ pointerEvents: 'none' }}
            >
              <circle cx="11" cy="11" r="6" />
              <line x1="19.59" y1="19.59" x2="15.24" y2="15.24" />
            </svg>
            <input
              ref={inputRef}
              type="text"
              className={styles.searchInput}
              placeholder="Search posts..."
              value={query}
              onChange={handleInputChange}
              onFocus={() => setIsFocused(true)}
              onBlur={() => {
                setTimeout(() => {
                  setIsFocused(false);
                  // Collapse search if query is empty
                  if (!query && onToggle && isExpanded) {
                    onToggle();
                  }
                }, 200);
              }}
            />
            {query && (
              <button
                className={styles.clearButton}
                onClick={() => {
                  setQuery('');
                  setResults([]);
                  inputRef.current?.focus();
                }}
                aria-label="Clear search"
              >
                ×
              </button>
            )}
          </div>
        </>
      )}

      {isExpanded && (
        <>
          {isLoading && query && (
            <div className={styles.searchResults} style={{ '--results-top': `${resultsTop}px` }}>
              <div className={styles.loading}>Loading search data...</div>
            </div>
          )}

          {!isLoading && query && results.length > 0 && (
            <div className={styles.searchResults} style={{ '--results-top': `${resultsTop}px` }}>
              {results.map((result, index) => (
                <Link
                  key={index}
                  href={getPostPath(result.link)}
                  className={styles.resultItem}
                >
                  <div className={styles.resultTitle}>{result.title}</div>
                  {result.categories.length > 0 && (
                    <div className={styles.resultCategories}>
                      {result.categories.map((cat, i) => (
                        <span key={i} className={styles.category}>{cat}</span>
                      ))}
                    </div>
                  )}
                  <div className={styles.resultExcerpt}>
                    {highlightText(result.content)}
                  </div>
                </Link>
              ))}
            </div>
          )}

          {!isLoading && query && searchData && results.length === 0 && (
            <div className={styles.searchResults} style={{ '--results-top': `${resultsTop}px` }}>
              <div className={styles.noResults}>No posts found for "{query}"</div>
            </div>
          )}
        </>
      )}
    </div>
  );
};

export default Search;
