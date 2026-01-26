(function() {
  'use strict';

  // Track which URLs have already been prefetched
  const prefetchedUrls = new Set();

  // Track the current active prefetch controller
  let currentPrefetchController = null;

  // Prefetch a URL using fetch API
  function prefetchUrl(url) {
    // Avoid prefetching the same URL multiple times
    if (prefetchedUrls.has(url)) {
      return;
    }

    // Cancel any previous pending prefetch
    if (currentPrefetchController) {
      currentPrefetchController.abort();
    }

    // Mark as prefetched
    prefetchedUrls.add(url);

    // Create new AbortController for this prefetch
    currentPrefetchController = new AbortController();

    // Use fetch with low priority for prefetching
    if (window.fetch) {
      try {
        fetch(url, {
          priority: 'low',
          credentials: 'same-origin',
          signal: currentPrefetchController.signal
        }).then(function() {
          // Prefetch completed successfully
          currentPrefetchController = null;
        }).catch(function(err) {
          // Silently fail - this is just a prefetch hint
          // Don't clear controller if it was an abort (we want to keep the new one)
          if (err.name !== 'AbortError') {
            currentPrefetchController = null;
          }
        });
      } catch (e) {
        // Silently fail
        currentPrefetchController = null;
      }
    }
  }

  // Check if URL should be prefetched (only internal links)
  function shouldPrefetch(url) {
    // Skip if not a valid URL
    if (!url) return false;

    // Skip external links, anchors, and special protocols
    if (url.startsWith('http://') ||
        url.startsWith('https://') ||
        url.startsWith('#') ||
        url.startsWith('mailto:') ||
        url.startsWith('tel:')) {
      // Only allow if it's same origin
      if (url.startsWith('http')) {
        try {
          const linkUrl = new URL(url);
          const currentUrl = new URL(window.location.href);
          if (linkUrl.origin !== currentUrl.origin) {
            return false;
          }
        } catch (e) {
          return false;
        }
      } else {
        return false;
      }
    }

    // Skip if already on the current page
    if (url === window.location.pathname ||
        url === window.location.pathname + window.location.search) {
      return false;
    }

    return true;
  }

  // Setup prefetch on hover for all links
  function setupPrefetchListeners() {
    // Get all links on the page
    const links = document.querySelectorAll('a[href]');

    links.forEach(link => {
      const href = link.getAttribute('href');

      if (!shouldPrefetch(href)) {
        return;
      }

      // Prefetch on mouseenter (hover)
      link.addEventListener('mouseenter', function() {
        prefetchUrl(href);
      }, { once: true, passive: true });

      // Also prefetch on touchstart for mobile devices
      link.addEventListener('touchstart', function() {
        prefetchUrl(href);
      }, { once: true, passive: true });
    });
  }

  // Initialize prefetch functionality on page load
  document.addEventListener('DOMContentLoaded', function() {
    // Small delay to avoid interfering with critical page load
    setTimeout(setupPrefetchListeners, 100);
  });
})();
