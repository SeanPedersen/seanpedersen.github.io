(function() {
  'use strict';

  // Track which URLs have already been prefetched
  const prefetchedUrls = new Set();

  // Stack to manage prefetch requests (LIFO)
  const prefetchStack = [];

  // Track URLs currently being prefetched
  const inProgressUrls = new Set();

  // Prefetch a URL using fetch API
  function prefetchUrl(url) {
    // Avoid prefetching if already done, in progress, or queued
    if (prefetchedUrls.has(url) || inProgressUrls.has(url) || prefetchStack.includes(url)) {
      return;
    }

    // Add to stack and process
    prefetchStack.push(url);
    processStack();
  }

  // Process the prefetch stack
  function processStack() {
    while (prefetchStack.length > 0) {
      const url = prefetchStack.pop();
      inProgressUrls.add(url);

      // Use fetch with low priority for prefetching
      if (window.fetch) {
        fetch(url, {
          priority: 'low',
          credentials: 'same-origin'
        }).then(function() {
          // Prefetch completed successfully
          prefetchedUrls.add(url);
          inProgressUrls.delete(url);
        }).catch(function(err) {
          // Silently fail - this is just a prefetch hint
          inProgressUrls.delete(url);
        });
      } else {
        inProgressUrls.delete(url);
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
