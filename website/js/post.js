(function () {
  'use strict';

  const POST_DATA = window.__POST_DATA__ || {};

  // Setup TOC toggle button (mobile only)
  function setupTocToggle() {
    const mobileToggleButton = document.getElementById('tocMobileToggle');
    if (!mobileToggleButton) return;

    const tocNav = document.querySelector('.toc');
    const tocList = document.getElementById('tocList');
    if (!tocNav || !tocList) return;

    // Only setup mobile toggle (desktop TOC is always visible via CSS)
    const isDesktop = window.innerWidth >= 1400;
    if (isDesktop) return;

    // Show mobile toggle button
    mobileToggleButton.style.display = 'block';

    let isExpanded = false;

    mobileToggleButton.addEventListener('click', function () {
      isExpanded = !isExpanded;
      if (isExpanded) {
        tocNav.classList.add('toc-visible');
        tocList.classList.remove('collapsed');
        tocList.classList.add('expanded');
        mobileToggleButton.classList.add('expanded');
        mobileToggleButton.setAttribute('aria-label', 'Hide table of contents');
      } else {
        tocNav.classList.remove('toc-visible');
        tocList.classList.remove('expanded');
        tocList.classList.add('collapsed');
        mobileToggleButton.classList.remove('expanded');
        mobileToggleButton.setAttribute('aria-label', 'Show table of contents');
      }
    });
  }

  // Add blockquote markers
  function addBlockquoteMarkers() {
    setTimeout(() => {
      const blockquotes = document.querySelectorAll('.markdown-content blockquote');

      blockquotes.forEach(blockquote => {
        // Skip if already processed
        if (blockquote.dataset.quoteProcessed) return;
        blockquote.dataset.quoteProcessed = 'true';

        // Set up blockquote positioning
        blockquote.style.position = 'relative';
        blockquote.style.paddingLeft = '1.5rem';

        // Get all text elements
        const elements = blockquote.querySelectorAll('p, li');
        if (elements.length === 0) return;

        const blockquoteRect = blockquote.getBoundingClientRect();

        // Process each element that contains text
        elements.forEach(element => {
          // Skip empty elements
          if (!element.textContent.trim()) return;

          const computedStyle = getComputedStyle(element);
          const lineHeight = parseFloat(computedStyle.lineHeight);

          // Get element position relative to blockquote
          const elementRect = element.getBoundingClientRect();
          const elementTop = elementRect.top - blockquoteRect.top;

          // Calculate number of lines this element spans
          const elementHeight = element.scrollHeight;
          const numLines = Math.max(1, Math.round(elementHeight / lineHeight));

          // Add markers for each line of this element
          for (let i = 0; i < numLines; i++) {
            const marker = document.createElement('span');
            marker.textContent = '>';
            marker.className = 'quote-marker';
            marker.style.cssText = `
              position: absolute;
              left: 0;
              top: ${elementTop + (i * lineHeight)}px;
              opacity: 0.5;
              user-select: none;
              pointer-events: none;
              line-height: ${lineHeight}px;
              height: ${lineHeight}px;
            `;
            blockquote.appendChild(marker);
          }
        });
      });
    }, 100);

    // Re-run on window resize
    let resizeTimer;
    window.addEventListener('resize', () => {
      clearTimeout(resizeTimer);
      resizeTimer = setTimeout(() => {
        // Reset all blockquotes
        document.querySelectorAll('.markdown-content blockquote').forEach(bq => {
          delete bq.dataset.quoteProcessed;
          bq.style.paddingLeft = '';
          bq.querySelectorAll('.quote-marker').forEach(m => m.remove());
        });
        addBlockquoteMarkers();
      }, 250);
    });
  }

  // Handle back to top button
  function setupBackToTop() {
    const container = document.getElementById('backToTopContainer');
    if (!container) return;

    const checkScrollbar = () => {
      const el = document.scrollingElement || document.documentElement;
      const hasScrollbar = el.scrollHeight > el.clientHeight + 1;

      if (hasScrollbar && POST_DATA.titleId) {
        container.innerHTML = `<a href="#${POST_DATA.titleId}">↑ Back to top</a>`;
      } else {
        container.innerHTML = '';
      }
    };

    checkScrollbar();

    window.addEventListener('resize', checkScrollbar);

    // Re-check when images load
    const imgs = Array.from(document.images || []);
    imgs.forEach(img => {
      if (!img.complete) img.addEventListener('load', checkScrollbar, { once: true });
    });
  }

  // Initialize everything on page load
  document.addEventListener('DOMContentLoaded', function () {
    setupTocToggle();
    addBlockquoteMarkers();
    setupBackToTop();
  });
})();
