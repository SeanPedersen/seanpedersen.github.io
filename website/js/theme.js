(function() {
  'use strict';

  // Get initial theme from cookie or system preference
  function getInitialTheme() {
    const storedTheme = document.cookie
      .split('; ')
      .find(row => row.startsWith('theme='));

    if (storedTheme) {
      return storedTheme.split('=')[1];
    }

    if (window.matchMedia && window.matchMedia('(prefers-color-scheme: light)').matches) {
      return 'light';
    }

    return 'dark'; // Default
  }

  // Apply theme to document
  function applyTheme(theme) {
    // Remove ALL inline style tags that look like anti-flicker styles
    // These have body{background-color with !important
    const inlineStyles = Array.from(document.head.querySelectorAll('style'));
    inlineStyles.forEach(style => {
      const content = style.textContent || style.innerHTML || '';
      // Check if it's a short inline style with !important and body background
      if (content.includes('!important') &&
          content.includes('body{') &&
          content.includes('background-color') &&
          content.length < 300) {
        style.remove();
      }
    });

    if (theme === 'light') {
      document.body.classList.add('light-theme');
      document.body.removeAttribute('data-theme');
    } else {
      document.body.classList.remove('light-theme');
      document.body.setAttribute('data-theme', 'dark');
    }
  }

  // Toggle theme
  function toggleTheme() {
    const currentTheme = document.body.classList.contains('light-theme') ? 'light' : 'dark';
    const newTheme = currentTheme === 'dark' ? 'light' : 'dark';

    applyTheme(newTheme);

    // Save theme to cookie
    document.cookie = `theme=${newTheme}; path=/; max-age=31536000; SameSite=Lax`;

    // Update button icon
    updateThemeIcon(newTheme);
  }

  // Update theme toggle icon
  function updateThemeIcon(theme) {
    const themeIcon = document.getElementById('themeIcon');
    if (!themeIcon) return;

    if (theme === 'dark') {
      // Show sun icon (switch to light)
      themeIcon.innerHTML = `<circle cx="12" cy="12" r="4" /><path d="M12 1v4M12 19v4M4.22 4.22l2.83 2.83M16.95 16.95l2.83 2.83M1 12h4M19 12h4M4.22 19.78l2.83-2.83M16.95 7.05l2.83-2.83" />`;
    } else {
      // Show moon icon (switch to dark)
      themeIcon.innerHTML = `<path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" />`;
    }

    themeIcon.setAttribute('title', `Switch to ${theme === 'dark' ? 'light' : 'dark'} mode`);
  }

  // Handle scroll to show/hide theme toggle on home page
  function setupScrollHandler() {
    const themeToggle = document.getElementById('themeToggle');
    if (!themeToggle) return;

    // Only apply scroll hiding on index page
    const isIndexPage = window.location.pathname === '/' || window.location.pathname === '/index.html';
    if (!isIndexPage) return;

    function handleScroll() {
      const currentScrollY = window.scrollY;

      if (currentScrollY <= 10) {
        // At the very top (within 10px)
        themeToggle.classList.remove('isHidden');
      } else {
        // Anywhere else, hide the toggle
        themeToggle.classList.add('isHidden');
      }
    }

    window.addEventListener('scroll', handleScroll);
    handleScroll(); // Initial check
  }

  // Initialize theme on page load
  document.addEventListener('DOMContentLoaded', function() {
    const theme = getInitialTheme();
    applyTheme(theme);
    updateThemeIcon(theme);

    // Setup theme toggle button
    const themeToggle = document.getElementById('themeToggle');
    if (themeToggle) {
      themeToggle.addEventListener('click', toggleTheme);
    }

    // Setup scroll handler for hiding/showing toggle
    setupScrollHandler();
  });
})();
