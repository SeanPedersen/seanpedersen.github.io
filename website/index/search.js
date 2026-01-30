(function () {
  'use strict';

  let searchData = null;
  let isLoadingSearch = false;
  let searchQuery = '';
  let searchExpanded = false;
  let originalPostOrder = []; // Store original order of posts

  // Create search UI
  function createSearchUI() {
    const container = document.getElementById('searchContainer');
    if (!container) return;

    // Initially just show the search icon button (collapsed state)
    container.innerHTML = `
      <div class="searchContainer collapsed" id="searchContainerEl">
        <button class="searchIconButton" id="searchIconBtn" aria-label="Open search">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="11" cy="11" r="6" />
            <line x1="19.59" y1="19.59" x2="15.24" y2="15.24" />
          </svg>
        </button>
      </div>
    `;

    // Add event listener to icon button - use mousedown for instant response
    document.getElementById('searchIconBtn').addEventListener('mousedown', (e) => {
      e.preventDefault(); // Prevent default to avoid focus issues
      expandSearch();
    });
  }

  // Expand search bar
  function expandSearch() {
    const container = document.getElementById('searchContainerEl');
    const wrapper = document.getElementById('searchTagsWrapper');

    if (!container || !wrapper) return;

    searchExpanded = true;
    const wrapperWidth = wrapper.offsetWidth;

    // Replace with expanded search bar
    container.classList.remove('collapsed');
    container.classList.add('expanded');
    container.innerHTML = `
      <div class="searchInputWrapper" style="--wrapper-width: ${wrapperWidth}px">
        <svg class="searchIcon" width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
          <circle cx="11" cy="11" r="6" />
          <line x1="19.59" y1="19.59" x2="15.24" y2="15.24" />
        </svg>
        <input
          id="searchInput"
          type="text"
          class="searchInput"
          placeholder="Search posts..."
          value="${searchQuery}"
        />
        <button class="clearButton" id="clearSearchBtn" aria-label="Clear search" style="display: none;">×</button>
      </div>
    `;

    // Add search expansion class to wrapper
    wrapper.classList.add('searchExpanded');

    // Hide tags
    const tagsContainer = document.getElementById('tagsContainer');
    if (tagsContainer) {
      tagsContainer.classList.add('tagsHidden');
    }

    // Focus input
    const input = document.getElementById('searchInput');
    if (input) {
      input.focus();

      // Backup focus attempt after render (helps on some mobile browsers)
      requestAnimationFrame(() => {
        input.focus();
      });

      input.addEventListener('input', handleSearchInput);
      input.addEventListener('blur', handleSearchBlur);

      // Show/hide clear button based on input value
      if (searchQuery) {
        document.getElementById('clearSearchBtn').style.display = 'flex';
      }
    }

    // Setup clear button
    const clearBtn = document.getElementById('clearSearchBtn');
    if (clearBtn) {
      // Use mousedown instead of click to prevent blur event from firing first
      clearBtn.addEventListener('mousedown', clearSearch);
    }

    // Setup click outside handler
    setupClickOutside();

    // Load RSS data asynchronously in background AFTER UI has expanded
    // Use setTimeout to ensure this happens after the current call stack
    setTimeout(() => {
      if (!searchData && !isLoadingSearch) {
        loadSearchData();
      }
    }, 0);
  }

  // Collapse search bar
  function collapseSearch() {
    const container = document.getElementById('searchContainerEl');
    const wrapper = document.getElementById('searchTagsWrapper');

    if (!container || !wrapper) return;

    searchExpanded = false;
    searchQuery = '';

    // Replace with collapsed icon button
    container.classList.remove('expanded');
    container.classList.add('collapsed');
    container.innerHTML = `
      <button class="searchIconButton" id="searchIconBtn" aria-label="Open search">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="11" cy="11" r="6" />
          <line x1="19.59" y1="19.59" x2="15.24" y2="15.24" />
        </svg>
      </button>
    `;

    // Remove search expansion class from wrapper
    wrapper.classList.remove('searchExpanded');

    // Show tags again
    const tagsContainer = document.getElementById('tagsContainer');
    if (tagsContainer) {
      tagsContainer.classList.remove('tagsHidden');
    }

    // Re-add icon button event listener - use mousedown for instant response
    document.getElementById('searchIconBtn').addEventListener('mousedown', (e) => {
      e.preventDefault(); // Prevent default to avoid focus issues
      expandSearch();
    });

    // Perform search with empty query
    performSearch('');
  }

  // Handle search input
  function handleSearchInput(e) {
    searchQuery = e.target.value;

    // Show/hide clear button
    const clearBtn = document.getElementById('clearSearchBtn');
    if (clearBtn) {
      clearBtn.style.display = searchQuery ? 'flex' : 'none';
    }

    // Check for matrix easter egg
    if (searchQuery.toLowerCase() === 'matrix') {
      activateMatrixEasterEgg();
      return;
    }

    // Perform search
    performSearch(searchQuery);
  }

  // Handle search blur
  function handleSearchBlur() {
    setTimeout(() => {
      if (!searchQuery && searchExpanded) {
        collapseSearch();
      }
    }, 200);
  }

  // Clear search
  function clearSearch(event) {
    // Prevent blur handler from collapsing the search
    if (event) {
      event.preventDefault();
      event.stopPropagation();
    }

    searchQuery = '';
    const input = document.getElementById('searchInput');
    if (input) {
      input.value = '';
      // Use setTimeout to refocus after the click event completes
      setTimeout(() => {
        input.focus();
      }, 0);
    }

    const clearBtn = document.getElementById('clearSearchBtn');
    if (clearBtn) {
      clearBtn.style.display = 'none';
    }

    performSearch('');
  }

  // Setup click outside handler
  function setupClickOutside() {
    setTimeout(() => {
      document.addEventListener('mousedown', handleClickOutside);
    }, 100);
  }

  // Handle click outside
  function handleClickOutside(e) {
    const container = document.getElementById('searchContainerEl');
    if (container && !container.contains(e.target) && !searchQuery) {
      document.removeEventListener('mousedown', handleClickOutside);
      collapseSearch();
    }
  }

  // Load search data from RSS
  function loadSearchData() {
    if (isLoadingSearch || searchData) return;

    isLoadingSearch = true;

    fetch('/rss.xml')
      .then(response => response.text())
      .then(xmlText => {
        const parser = new DOMParser();
        const xmlDoc = parser.parseFromString(xmlText, 'text/xml');
        const items = xmlDoc.querySelectorAll('item');

        searchData = Array.from(items).map(item => {
          const title = item.querySelector('title')?.textContent || '';
          const link = item.querySelector('link')?.textContent || '';
          const contentEncoded = item.querySelector('encoded')?.textContent || '';
          const categories = Array.from(item.querySelectorAll('category')).map(
            cat => cat.textContent
          );

          // Strip HTML tags from content for searching
          // Remove img tags first to prevent browser from loading images
          const contentWithoutImages = contentEncoded.replace(/<img[^>]*>/gi, '');
          const tempDiv = document.createElement('div');
          tempDiv.innerHTML = contentWithoutImages;
          const textContent = tempDiv.textContent || tempDiv.innerText || '';

          // Extract post ID from link
          const url = new URL(link);
          const id = url.pathname.split('/').filter(Boolean).pop();

          return {
            id,
            title,
            content: textContent,
            categories,
            searchText: `${title} ${textContent} ${categories.join(' ')}`.toLowerCase()
          };
        });

        isLoadingSearch = false;

        // Re-run search now that data is loaded
        if (searchQuery) {
          performSearch(searchQuery);
        }
      })
      .catch(error => {
        console.error('Failed to load RSS feed:', error);
        isLoadingSearch = false;
      });
  }

  // Perform search
  function performSearch(query) {
    const postList = document.getElementById('postList');
    if (!postList) return;

    const posts = Array.from(postList.children);

    // Capture original order on first search
    if (originalPostOrder.length === 0) {
      originalPostOrder = posts.slice();
    }

    if (!query.trim()) {
      // Restore original order and remove highlights
      originalPostOrder.forEach(post => {
        post.style.display = '';
        postList.appendChild(post);
        const link = post.querySelector('a');
        if (link && searchData) {
          const href = link.getAttribute('href') || '';
          const postId = href.match(/\/posts\/([^/]+)\/?$/)?.[1] || '';
          const postSearchData = postId && searchData.find(p => p.id === postId);
          if (postSearchData) {
            const titleEl = link.querySelector('.postTitleText');
            if (titleEl) {
              titleEl.innerHTML = escapeHtml(postSearchData.title);
            } else {
              link.innerHTML = escapeHtml(postSearchData.title);
            }
          }
        }
      });
      return;
    }

    // Only perform RSS-based search if data is loaded
    if (!searchData) {
      // RSS not loaded yet, inline search (in templates.js) is handling it
      return;
    }

    const lowerQuery = query.toLowerCase();

    // RSS full-text search with scoring and sorting (combines title/tag/content matches)
    const scoredPosts = posts.map(post => {
      const link = post.querySelector('a');
      const href = link?.getAttribute('href') || '';
      const postId = href.match(/\/posts\/([^/]+)\/?$/)?.[1] || '';
      const postSearchData = postId && searchData.find(p => p.id === postId);

      let score = 0;
      let titleMatch = false;
      let tagMatch = false;

      // Get DOM tags for tag matching (from class names like tag-AI, tag-ML)
      const tags = Array.from(post.classList).filter(c => c.startsWith('tag-')).map(c => c.slice(4));

      if (!postSearchData) {
        // No RSS data, fall back to title/tag only
        const title = link?.textContent || '';

        if (title.toLowerCase().includes(lowerQuery)) {
          score += 10;
          titleMatch = true;
        }

        if (tags.some(tag => tag.toLowerCase().includes(lowerQuery))) {
          score += 5;
          tagMatch = true;
        }

        return { post, score, titleMatch, tagMatch };
      }

      const lowerTitle = postSearchData.title.toLowerCase();
      const lowerContent = postSearchData.content.toLowerCase();

      // Check if title matches
      if (lowerTitle.includes(lowerQuery)) {
        const regex = new RegExp(lowerQuery.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'g');
        const titleMatches = (lowerTitle.match(regex) || []).length;
        score += titleMatches * 10;
        titleMatch = true;
      }

      // Check if tags match (DOM tags, not RSS categories)
      if (tags.some(tag => tag.toLowerCase().includes(lowerQuery))) {
        score += 5;
        tagMatch = true;
      }

      // Check if content matches
      if (lowerContent.includes(lowerQuery)) {
        const regex = new RegExp(lowerQuery.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'g');
        const contentMatches = (lowerContent.match(regex) || []).length;
        score += contentMatches;
      }

      // Check if categories match (RSS categories as additional signal)
      postSearchData.categories.forEach(cat => {
        if (cat.toLowerCase().includes(lowerQuery)) {
          score += 3;
        }
      });

      return { post, score, titleMatch, tagMatch };
    });

    // Sort by title match first, then tag match, then by score
    scoredPosts.sort((a, b) => {
      if (a.titleMatch && !b.titleMatch) return -1;
      if (!a.titleMatch && b.titleMatch) return 1;
      if (a.tagMatch && !b.tagMatch) return -1;
      if (!a.tagMatch && b.tagMatch) return 1;
      return b.score - a.score;
    });

    // Reorder DOM elements by appending in sorted order
    scoredPosts.forEach(({ post, score, titleMatch }) => {
      if (score > 0) {
        post.style.display = '';

        // Highlight title if it matches
        const link = post.querySelector('a');
        if (link) {
          const href = link.getAttribute('href') || '';
          const postId = href.match(/\/posts\/([^/]+)\/?$/)?.[1] || '';
          const postSearchData = postId && searchData.find(p => p.id === postId);
          if (postSearchData) {
            const titleEl = link.querySelector('.postTitleText');
            const targetEl = titleEl || link;
            if (titleMatch) {
              targetEl.innerHTML = highlightTitle(postSearchData.title, query);
            } else {
              // No title match, just display plain title without highlighting
              targetEl.innerHTML = escapeHtml(postSearchData.title);
            }
          }
        }

        // Reorder by appending to end (in sorted order)
        postList.appendChild(post);
      } else {
        post.style.display = 'none';
      }
    });
  }

  // Highlight matching text in title
  function highlightTitle(title, query) {
    if (!query.trim()) return escapeHtml(title);

    const lowerTitle = title.toLowerCase();
    const lowerQuery = query.toLowerCase();
    const index = lowerTitle.indexOf(lowerQuery);

    if (index === -1) return escapeHtml(title);

    const before = title.slice(0, index);
    const match = title.slice(index, index + query.length);
    const after = title.slice(index + query.length);

    return `${escapeHtml(before)}<mark style="background-color: var(--primary-color); color: var(--color-background); padding: 2px 4px; border-radius: 3px;">${escapeHtml(match)}</mark>${escapeHtml(after)}`;
  }

  // Escape HTML
  function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
  }

  // Matrix easter egg
  let matrixActive = false;
  let matrixScriptLoaded = false;

  function activateMatrixEasterEgg() {
    // Don't toggle off if already active - only ESC can deactivate
    if (matrixActive) {
      return;
    }

    // Load matrix.js if not already loaded
    if (!matrixScriptLoaded) {
      const script = document.createElement('script');
      script.src = '/js/matrix.js';
      script.onload = function() {
        matrixScriptLoaded = true;
        startMatrix();
      };
      document.head.appendChild(script);
    } else {
      startMatrix();
    }
  }

  function startMatrix() {
    if (window.Matrix && !matrixActive) {
      const width = window.innerWidth;
      const height = window.innerHeight;
      window.Matrix.init(width, height);
      matrixActive = true;

      // Make body background transparent so matrix shows through
      document.body.style.backgroundColor = 'transparent';
      document.documentElement.style.backgroundColor = '#000'; // Keep html black for contrast

      // Handle window resize
      window.addEventListener('resize', handleMatrixResize);

      // Add escape handler to deactivate
      document.addEventListener('keydown', handleMatrixEscape);
    }
  }

  function handleMatrixResize() {
    if (matrixActive && window.Matrix) {
      window.Matrix.destroy();
      const width = window.innerWidth;
      const height = window.innerHeight;
      window.Matrix.init(width, height);
    }
  }

  function handleMatrixEscape(e) {
    if (e.key === 'Escape' && matrixActive) {
      if (window.Matrix) {
        window.Matrix.destroy();
        matrixActive = false;
      }

      // Restore body background
      document.body.style.backgroundColor = '';
      document.documentElement.style.backgroundColor = '';

      // Also clear the search input
      const input = document.getElementById('searchInput');
      if (input) {
        input.value = '';
        searchQuery = '';
        const clearBtn = document.getElementById('clearSearchBtn');
        if (clearBtn) {
          clearBtn.style.display = 'none';
        }
      }
      document.removeEventListener('keydown', handleMatrixEscape);
      window.removeEventListener('resize', handleMatrixResize);
    }
  }

  // Keyboard shortcuts
  function setupKeyboardShortcuts() {
    document.addEventListener('keydown', (e) => {
      // Ctrl/Cmd + K to focus search
      if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
        e.preventDefault();
        if (!searchExpanded) {
          expandSearch();
        } else {
          const input = document.getElementById('searchInput');
          if (input) input.focus();
        }
      }

      // Escape to clear and close
      if (e.key === 'Escape') {
        const input = document.getElementById('searchInput');
        if (input && document.activeElement === input) {
          if (searchQuery) {
            clearSearch();
          } else if (searchExpanded) {
            collapseSearch();
          }
        }
      }
    });
  }

  // Initialize search on page load
  function initSearch() {
    const existingInput = document.getElementById('searchInput');

    if (existingInput) {
      // Inline search already initialized, enhance it
      enhanceInlineSearch();
    } else {
      // Normal initialization (keyboard shortcut case)
      createSearchUI();
    }

    setupKeyboardShortcuts();
  }

  // Enhance existing inline search with full functionality
  function enhanceInlineSearch() {
    const input = document.getElementById('searchInput');
    if (!input) return;

    // Store current value and focus state
    searchQuery = window.__searchInputValue || input.value || '';
    const hadFocus = document.activeElement === input;

    // Clone input to remove inline event listeners
    const newInput = input.cloneNode(true);
    input.parentNode.replaceChild(newInput, input);

    // Setup full search handlers
    newInput.addEventListener('input', handleSearchInput);
    newInput.addEventListener('blur', handleSearchBlur);

    // Restore focus if input had it before cloning
    if (hadFocus) {
      newInput.focus();
    }

    // Clone clear button to remove inline event listeners
    const clearBtn = document.getElementById('clearSearchBtn');
    if (clearBtn) {
      const newClearBtn = clearBtn.cloneNode(true);
      clearBtn.parentNode.replaceChild(newClearBtn, clearBtn);
      newClearBtn.addEventListener('mousedown', clearSearch);
    }

    setupClickOutside();

    // Load RSS in background
    setTimeout(() => {
      if (!searchData && !isLoadingSearch) {
        loadSearchData();
      }
    }, 0);

    // Trigger search with current value for highlighting
    if (searchQuery) {
      performSearch(searchQuery);
    }
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initSearch);
  } else {
    // DOM already loaded (lazy loaded case)
    initSearch();
  }
})();
