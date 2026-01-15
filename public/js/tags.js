(function() {
  'use strict';

  let selectedTag = null;

  // Initialize tag filtering
  function initializeTagFiltering() {
    // Set initial tag from hash
    const hash = window.location.hash.replace(/^#/, '');
    if (hash) {
      selectedTag = hash;
      selectTagByName(hash);
    }

    // Listen for hash changes (back/forward navigation)
    window.addEventListener('hashchange', handleHashChange);

    // Setup tag click listeners
    setupTagListeners();

    // Setup tag clicks in post list
    setupPostTagListeners();
  }

  // Handle hash change
  function handleHashChange() {
    const hash = window.location.hash.replace(/^#/, '');
    selectedTag = hash || null;
    selectTagByName(hash);
    filterPosts();
  }

  // Setup tag filter listeners
  function setupTagListeners() {
    const tags = document.querySelectorAll('.tagsContainer .tag');

    tags.forEach(tag => {
      tag.addEventListener('click', function() {
        const tagName = this.getAttribute('data-tag');
        handleTagSelect(tagName);
      });
    });
  }

  // Setup post tag listeners (tags in post list items)
  function setupPostTagListeners() {
    const postTagLinks = document.querySelectorAll('.postTags a');

    postTagLinks.forEach(link => {
      link.addEventListener('click', function(e) {
        e.preventDefault();
        e.stopPropagation();

        const href = this.getAttribute('href');
        const tagName = href.replace(/^#/, '');
        handleTagSelect(tagName);
      });
    });
  }

  // Handle tag selection
  function handleTagSelect(tagName) {
    selectedTag = tagName || null;

    if (tagName) {
      window.location.hash = tagName;
    } else {
      // Clear hash without creating history entry
      if (window.history.replaceState) {
        window.history.replaceState(null, '', window.location.pathname + window.location.search);
      } else {
        window.location.hash = '';
      }
    }

    selectTagByName(tagName);
    filterPosts();
  }

  // Select tag by name
  function selectTagByName(tagName) {
    const tags = document.querySelectorAll('.tagsContainer .tag');

    tags.forEach(tag => {
      const currentTagName = tag.getAttribute('data-tag');

      if (currentTagName === tagName || (!tagName && currentTagName === '')) {
        tag.classList.add('tagSelected');
      } else {
        tag.classList.remove('tagSelected');
      }
    });
  }

  // Filter posts by selected tag
  function filterPosts() {
    const postList = document.getElementById('postList');
    if (!postList) return;

    const posts = Array.from(postList.children);

    posts.forEach(post => {
      if (!selectedTag) {
        // Show all posts
        post.style.display = '';
        showAllTagsInPost(post);
      } else {
        const tagsAttr = post.getAttribute('data-tags');
        const tags = tagsAttr ? JSON.parse(tagsAttr) : [];

        if (tags.includes(selectedTag)) {
          post.style.display = '';
          hideSelectedTagInPost(post, selectedTag);
        } else {
          post.style.display = 'none';
        }
      }
    });
  }

  // Hide selected tag in post (remove it from display)
  function hideSelectedTagInPost(post, selectedTag) {
    const tagsContainer = post.querySelector('.post-tags-container');
    if (!tagsContainer) return;

    const tagsAttr = post.getAttribute('data-tags');
    const tags = tagsAttr ? JSON.parse(tagsAttr) : [];

    // Filter out selected tag
    const displayTags = tags.filter(tag => tag !== selectedTag);

    if (displayTags.length === 0) {
      tagsContainer.style.display = 'none';
    } else {
      tagsContainer.style.display = '';
      const postTagsSpan = tagsContainer.querySelector('.postTags');
      if (postTagsSpan) {
        postTagsSpan.innerHTML = displayTags.map((tag, idx) =>
          `<span><a href="#${escapeHtml(tag)}" onclick="return false;">${escapeHtml(tag)}</a>${idx < displayTags.length - 1 ? ', ' : ''}</span>`
        ).join('');

        // Re-setup listeners for new tags
        setupPostTagListeners();
      }
    }
  }

  // Show all tags in post
  function showAllTagsInPost(post) {
    const tagsContainer = post.querySelector('.post-tags-container');
    if (!tagsContainer) return;

    const tagsAttr = post.getAttribute('data-tags');
    const tags = tagsAttr ? JSON.parse(tagsAttr) : [];

    if (tags.length === 0) {
      tagsContainer.style.display = 'none';
    } else {
      tagsContainer.style.display = '';
      const postTagsSpan = tagsContainer.querySelector('.postTags');
      if (postTagsSpan) {
        postTagsSpan.innerHTML = tags.map((tag, idx) =>
          `<span><a href="#${escapeHtml(tag)}" onclick="return false;">${escapeHtml(tag)}</a>${idx < tags.length - 1 ? ', ' : ''}</span>`
        ).join('');

        // Re-setup listeners for new tags
        setupPostTagListeners();
      }
    }
  }

  // Escape HTML
  function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
  }

  // Initialize on page load
  document.addEventListener('DOMContentLoaded', function() {
    initializeTagFiltering();
  });
})();
