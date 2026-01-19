// Only used on post pages containing code blocks
(function () {
  'use strict';

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

      expandBtn.addEventListener('click', function () {
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
        <rect x="3" y="3" width="13" height="13" rx="2" stroke="currentColor" stroke-width="1.5" fill="none"/>
        <rect x="7" y="7" width="13" height="13" rx="2" stroke="currentColor" stroke-width="1.5" fill="none"/>
      </svg>
    `;

    let copied = false;

    copyBtn.addEventListener('click', async function () {
      try {
        await navigator.clipboard.writeText(code.textContent);
        copied = true;

        // Show checkmark
        copyBtn.innerHTML = `
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M20 6L9 17l-5-5" stroke="currentColor" stroke-width="2.5" strokeLinecap="round" strokeLinejoin="round"/>
          </svg>
        `;

        setTimeout(() => {
          copied = false;
          copyBtn.innerHTML = `
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <rect x="3" y="3" width="13" height="13" rx="2" stroke="currentColor" stroke-width="1.5" fill="none"/>
              <rect x="7" y="7" width="13" height="13" rx="2" stroke="currentColor" stroke-width="1.5" fill="none"/>
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

  // Initialize on page load
  document.addEventListener('DOMContentLoaded', makeCodeBlocksExpandable);
})();
