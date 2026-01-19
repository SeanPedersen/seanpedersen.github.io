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
    setupBackToTop();
  });
})();
