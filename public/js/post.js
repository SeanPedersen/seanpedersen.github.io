(function() {
  'use strict';

  const POST_DATA = window.__POST_DATA__ || {};

  // Decode HTML entities
  function decodeHTMLEntities(str) {
    if (!str || typeof str !== 'string') return str;
    const named = {
      amp: '&',
      lt: '<',
      gt: '>',
      quot: '"',
      apos: "'",
      nbsp: '\u00A0'
    };
    return str.replace(/&(#(?:x[0-9a-fA-F]+|\d+)|[a-zA-Z]+);/g, (match, code) => {
      if (code[0] === '#') {
        const isHex = code[1] === 'x' || code[1] === 'X';
        const num = parseInt(code.slice(isHex ? 2 : 1), isHex ? 16 : 10);
        if (!isNaN(num)) {
          try { return String.fromCodePoint(num); } catch { return String.fromCharCode(num); }
        }
        return match;
      }
      return Object.prototype.hasOwnProperty.call(named, code) ? named[code] : match;
    });
  }

  // Extract headings from content
  function getHeadings(content) {
    const regex = /<h([1-6]).*?>(.*?)<\/h[1-6]>/g;
    const headings = [];
    let match;

    while ((match = regex.exec(content)) !== null) {
      const level = Number(match[1]);
      const rawText = match[2].replace(/<[^>]*>/g, '');
      const decodedText = decodeHTMLEntities(rawText);
      const id = decodedText.toLowerCase().replace(/[^a-z0-9]+/g, '-');
      headings.push({ level, text: decodedText, id });
    }

    return headings;
  }

  // Generate Table of Contents
  function generateTableOfContents() {
    const tocContainer = document.getElementById('tocContainer');
    if (!tocContainer || !POST_DATA.hasTableOfContents) return;

    const markdownContent = document.querySelector('.markdown-content');
    if (!markdownContent) return;

    const headings = getHeadings(markdownContent.innerHTML);

    if (headings.length === 0) return;

    const tocHTML = `
      <nav class="toc">
        <div class="tocHeader">
          ${POST_DATA.title && POST_DATA.titleId ? `<h2><a href="#${POST_DATA.titleId}">${POST_DATA.title}</a></h2>` : '<h2>Contents</h2>'}
          <button
            class="toggleButton"
            id="tocToggle"
            aria-label="Toggle table of contents"
          >
            ▽
          </button>
        </div>
        <ul class="tocList collapsed" id="tocList">
          ${headings.map((heading, index) => `
            <li style="margin-left: ${Math.max(0, (heading.level - 1) * 12)}px;">
              <a href="#${heading.id}">${heading.text}</a>
            </li>
          `).join('')}
        </ul>
      </nav>
    `;

    tocContainer.innerHTML = tocHTML;

    // Setup toggle button
    const toggleButton = document.getElementById('tocToggle');
    const tocList = document.getElementById('tocList');
    let isExpanded = false;

    if (toggleButton && tocList) {
      toggleButton.addEventListener('click', function() {
        isExpanded = !isExpanded;
        if (isExpanded) {
          tocList.classList.remove('collapsed');
          tocList.classList.add('expanded');
          toggleButton.textContent = '△';
          toggleButton.setAttribute('aria-label', 'Collapse table of contents');
        } else {
          tocList.classList.remove('expanded');
          tocList.classList.add('collapsed');
          toggleButton.textContent = '▽';
          toggleButton.setAttribute('aria-label', 'Expand table of contents');
        }
      });
    }
  }

  // Make code blocks expandable
  function makeCodeBlocksExpandable() {
    const markdownContent = document.querySelector('.markdown-content');
    if (!markdownContent) return;

    const codeBlocks = markdownContent.querySelectorAll('pre');

    codeBlocks.forEach((pre, index) => {
      const code = pre.querySelector('code');
      if (!code) return;

      const textContent = code.textContent;
      const lineCount = textContent.split('\n').length;

      // Only make expandable if more than 20 lines
      if (lineCount <= 20) {
        // Just add copy button
        addCopyButton(pre, code, false, false);
        return;
      }

      // Wrap in expandable container
      const wrapper = document.createElement('div');
      wrapper.className = 'codeBlockWrapper';

      const container = document.createElement('div');
      container.className = 'codeBlockContainer';

      const parent = pre.parentNode;
      parent.insertBefore(wrapper, pre);
      container.appendChild(pre);
      wrapper.appendChild(container);

      // Add collapsed class
      const codeBlock = document.createElement('div');
      codeBlock.className = 'codeBlock collapsed';
      while (container.firstChild) {
        codeBlock.appendChild(container.firstChild);
      }
      container.appendChild(codeBlock);

      // Add fade overlay
      const overlay = document.createElement('div');
      overlay.className = 'fadeOverlay';
      container.appendChild(overlay);

      // Add copy button
      addCopyButton(codeBlock, code, true, false);

      // Add expand button
      const expandBtn = document.createElement('button');
      expandBtn.className = 'expandButton';
      expandBtn.textContent = `Show More (${lineCount} lines)`;
      wrapper.appendChild(expandBtn);

      let isExpanded = false;

      expandBtn.addEventListener('click', function() {
        isExpanded = !isExpanded;

        if (isExpanded) {
          codeBlock.classList.remove('collapsed');
          overlay.style.display = 'none';
          expandBtn.textContent = 'Show Less';

          // Update copy button position
          const copyBtn = codeBlock.querySelector('.copyButton, .copyButtonExpanded');
          if (copyBtn) {
            copyBtn.classList.remove('copyButton');
            copyBtn.classList.add('copyButtonExpanded');
          }
        } else {
          codeBlock.classList.add('collapsed');
          overlay.style.display = 'block';
          expandBtn.textContent = `Show More (${lineCount} lines)`;

          // Update copy button position
          const copyBtn = codeBlock.querySelector('.copyButton, .copyButtonExpanded');
          if (copyBtn) {
            copyBtn.classList.remove('copyButtonExpanded');
            copyBtn.classList.add('copyButton');
          }
        }
      });
    });
  }

  // Add copy button to code block
  function addCopyButton(parent, code, isExpandable, isExpanded) {
    const copyBtn = document.createElement('button');
    copyBtn.className = isExpandable ? (isExpanded ? 'copyButtonExpanded' : 'copyButton') : 'copyButtonNormal';
    copyBtn.title = 'Copy code';
    copyBtn.innerHTML = `
      <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <rect x="3" y="3" width="13" height="13" rx="2" stroke="currentColor" strokeWidth="1.5" fill="none"/>
        <rect x="7" y="7" width="13" height="13" rx="2" stroke="currentColor" strokeWidth="1.5" fill="none"/>
      </svg>
    `;

    let copied = false;

    copyBtn.addEventListener('click', async function() {
      try {
        await navigator.clipboard.writeText(code.textContent);
        copied = true;

        // Show checkmark
        copyBtn.innerHTML = `
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M20 6L9 17l-5-5" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round"/>
          </svg>
        `;

        setTimeout(() => {
          copied = false;
          copyBtn.innerHTML = `
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <rect x="3" y="3" width="13" height="13" rx="2" stroke="currentColor" strokeWidth="1.5" fill="none"/>
              <rect x="7" y="7" width="13" height="13" rx="2" stroke="currentColor" strokeWidth="1.5" fill="none"/>
            </svg>
          `;
        }, 2000);
      } catch (err) {
        console.error('Failed to copy code:', err);
      }
    });

    parent.style.position = 'relative';
    parent.appendChild(copyBtn);
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

  // Handle initial page load with hash
  function handleInitialHash() {
    const hash = window.location.hash;
    if (!hash) return;

    // Wait for all images to load before scrolling
    const imgs = Array.from(document.images || []);
    if (imgs.length === 0) {
      scrollToHash(hash);
      return;
    }

    const promises = imgs.map(img => {
      if (img.complete) return Promise.resolve();
      return new Promise(resolve => {
        img.addEventListener('load', resolve, { once: true });
        img.addEventListener('error', resolve, { once: true });
      });
    });

    Promise.all(promises).then(() => scrollToHash(hash));
  }

  // Scroll to hash
  function scrollToHash(hash) {
    const id = hash.replace('#', '');
    const element = document.getElementById(id);
    if (element) {
      element.scrollIntoView({ behavior: 'auto', block: 'start' });
    }
  }

  // Initialize everything on page load
  document.addEventListener('DOMContentLoaded', function() {
    generateTableOfContents();
    makeCodeBlocksExpandable();
    addBlockquoteMarkers();
    setupBackToTop();
    handleInitialHash();
  });
})();
