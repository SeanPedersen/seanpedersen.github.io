import { useState, useEffect, useRef } from 'react';
import styles from './Search.module.css';

const Search = ({ isExpanded = false, onToggle = null, onSearch = null }) => {
  const [query, setQuery] = useState('');
  const [isFocused, setIsFocused] = useState(false);
  const [wrapperWidth, setWrapperWidth] = useState(0);
  const inputRef = useRef(null);
  const containerRef = useRef(null);
  const inputWrapperRef = useRef(null);

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

      // Auto-focus input immediately - don't wait for animation
      if (inputRef.current) {
        inputRef.current.focus();
      }
    }
  }, [isExpanded]);

  // Handle input change
  const handleInputChange = (e) => {
    const value = e.target.value;
    setQuery(value);

    // Pass query to parent component
    if (onSearch) {
      onSearch(value);
    }
  };

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
        if (onSearch) {
          onSearch('');
        }
        if (onToggle && isExpanded) {
          onToggle(); // Collapse search
        } else {
          inputRef.current?.blur();
        }
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [isFocused, isExpanded, onToggle, onSearch]);

  // Click outside to close search
  useEffect(() => {
    if (!isExpanded) return;

    const handleClickOutside = (e) => {
      // Check if click is outside the input wrapper
      const clickedOutsideInput = inputWrapperRef.current && !inputWrapperRef.current.contains(e.target);

      // Only close if query is empty
      if (clickedOutsideInput && !query) {
        // Collapse search if toggle function provided
        if (onToggle) {
          onToggle();
        }
      }
    };

    // Add slight delay to prevent immediate closing when opening
    const timeoutId = setTimeout(() => {
      document.addEventListener('mousedown', handleClickOutside);
    }, 100);

    return () => {
      clearTimeout(timeoutId);
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [isExpanded, onToggle, query]);

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
                if (onSearch) {
                  onSearch('');
                }
                inputRef.current?.focus();
              }}
              aria-label="Clear search"
            >
              ×
            </button>
          )}
        </div>
      )}
    </div>
  );
};

export default Search;
