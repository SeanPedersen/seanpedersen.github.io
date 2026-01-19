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

  // Initialize everything on page load
  document.addEventListener('DOMContentLoaded', function () {
    setupTocToggle();
  });
})();
