import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import { Worker } from 'worker_threads';
import os from 'os';
import { getSortedPostsData, getAllTags, getAllPostIds } from '../lib/posts.js';
import { generateIndexHTML } from './templates.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const MAX_WORKERS = os.cpus().length * 2;

// Helper to copy directory recursively
function copyDir(src, dest) {
  fs.mkdirSync(dest, { recursive: true });
  const entries = fs.readdirSync(src, { withFileTypes: true });

  for (const entry of entries) {
    const srcPath = path.join(src, entry.name);
    const destPath = path.join(dest, entry.name);

    if (entry.isDirectory()) {
      copyDir(srcPath, destPath);
    } else {
      fs.copyFileSync(srcPath, destPath);
    }
  }
}

// Helper to copy file
function copyFile(src, dest) {
  const destDir = path.dirname(dest);
  fs.mkdirSync(destDir, { recursive: true });
  fs.copyFileSync(src, dest);
}

async function build() {
  console.log('Starting static site generation...\n');

  const outDir = path.join(process.cwd(), 'out');

  // Clean output directory
  if (fs.existsSync(outDir)) {
    console.log('Cleaning output directory...');
    fs.rmSync(outDir, { recursive: true });
  }

  // Create output directory
  fs.mkdirSync(outDir, { recursive: true });
  console.log('Created output directory: out/\n');

  // Get all posts data
  console.log('Reading posts...');
  const allPostsData = getSortedPostsData();
  const allTags = getAllTags();
  console.log(`Found ${allPostsData.length} posts and ${allTags.length} tags\n`);

  // Generate index page
  console.log('Generating index page...');
  const indexHTML = generateIndexHTML(allPostsData, allTags);
  fs.writeFileSync(path.join(outDir, 'index.html'), indexHTML, 'utf8');
  console.log('✓ Generated index.html\n');

  // Generate individual post pages
  console.log('Generating post pages...');
  const postsDir = path.join(outDir, 'posts');
  fs.mkdirSync(postsDir, { recursive: true });

  const allPostIds = getAllPostIds();
  const totalPosts = allPostIds.length;
  console.log(`  Processing ${totalPosts} posts using ${MAX_WORKERS} workers...`);

  // Process all posts using worker threads with concurrency limit
  const startTime = Date.now();
  let completed = 0;
  let activeWorkers = 0;
  const queue = [...allPostIds];
  const errors = [];

  await new Promise((resolve, reject) => {
    function startWorker() {
      if (queue.length === 0) {
        if (activeWorkers === 0) {
          resolve();
        }
        return;
      }

      const { params } = queue.shift();
      activeWorkers++;

      const worker = new Worker(path.join(__dirname, 'workers', 'post-worker.js'), {
        workerData: {
          postId: params.id,
          postsDir
        }
      });

      worker.on('message', (result) => {
        if (result.success) {
          completed++;
          if (completed % 10 === 0 || completed === totalPosts) {
            console.log(`  [${completed}/${totalPosts}] posts completed...`);
          }
        } else {
          errors.push(`Failed to process ${result.postId}: ${result.error}`);
        }
        activeWorkers--;
        startWorker();
      });

      worker.on('error', (error) => {
        errors.push(`Worker error: ${error.message}`);
        activeWorkers--;
        startWorker();
      });

      worker.on('exit', (code) => {
        if (code !== 0) {
          errors.push(`Worker stopped with exit code ${code}`);
        }
      });
    }

    // Start initial batch of workers
    for (let i = 0; i < Math.min(MAX_WORKERS, totalPosts); i++) {
      startWorker();
    }
  });

  if (errors.length > 0) {
    console.error('\n⚠️  Some posts failed to generate:');
    errors.forEach(err => console.error(`  ${err}`));
  }

  const duration = ((Date.now() - startTime) / 1000).toFixed(2);
  console.log(`✓ Generated ${totalPosts} post pages in ${duration}s using ${MAX_WORKERS} workers\n`);

  // Copy static assets
  console.log('Copying static assets...');

  // Copy images
  const imagesDir = path.join(process.cwd(), 'public', 'images');
  if (fs.existsSync(imagesDir)) {
    copyDir(imagesDir, path.join(outDir, 'images'));
    console.log('  ✓ Copied images/');
  }

  // Copy favicon
  const faviconPath = path.join(process.cwd(), 'public', 'favicon.ico');
  if (fs.existsSync(faviconPath)) {
    copyFile(faviconPath, path.join(outDir, 'favicon.ico'));
    console.log('  ✓ Copied favicon.ico');
  }

  // Create styles directory and copy CSS files
  const stylesOutDir = path.join(outDir, 'styles');
  fs.mkdirSync(stylesOutDir, { recursive: true });

  // Copy global.css
  copyFile(
    path.join(process.cwd(), 'styles', 'global.css'),
    path.join(stylesOutDir, 'global.css')
  );
  console.log('  ✓ Copied global.css');

  // Copy prism theme
  copyFile(
    path.join(process.cwd(), 'node_modules', 'prismjs', 'themes', 'prism-tomorrow.css'),
    path.join(stylesOutDir, 'prism-tomorrow.css')
  );
  console.log('  ✓ Copied prism-tomorrow.css');

  // Copy utils.css (converted from module)
  const utilsModulePath = path.join(process.cwd(), 'styles', 'utils.module.css');
  const utilsCssPath = path.join(stylesOutDir, 'utils.css');

  // Read and convert module CSS to global CSS (remove .className notation)
  let utilsCss = fs.readFileSync(utilsModulePath, 'utf8');
  // The CSS classes in utils.module.css are already using simple class names, just copy it
  fs.writeFileSync(utilsCssPath, utilsCss, 'utf8');
  console.log('  ✓ Copied utils.css');

  // Copy layout.module.css as layout.css
  const layoutModulePath = path.join(process.cwd(), 'styles', 'layout.module.css');
  const layoutCssPath = path.join(stylesOutDir, 'layout.css');

  if (fs.existsSync(layoutModulePath)) {
    let layoutCss = fs.readFileSync(layoutModulePath, 'utf8');
    // Remove :global() wrappers if any
    layoutCss = layoutCss.replace(/:global\((.*?)\)/g, '$1');
    fs.writeFileSync(layoutCssPath, layoutCss, 'utf8');
    console.log('  ✓ Copied layout.css');
  }

  // Append layout CSS to global.css to ensure all styles are loaded
  const globalCssPath = path.join(stylesOutDir, 'global.css');
  let globalCss = fs.readFileSync(globalCssPath, 'utf8');
  globalCss += '\n\n/* Layout styles */\n' + fs.readFileSync(layoutCssPath, 'utf8');
  fs.writeFileSync(globalCssPath, globalCss, 'utf8');
  console.log('  ✓ Merged layout.css into global.css');

  // Create js directory (for vanilla JS files)
  const jsOutDir = path.join(outDir, 'js');
  fs.mkdirSync(jsOutDir, { recursive: true });
  console.log('  ✓ Created js/ directory');

  // Copy vanilla JS files if they exist
  const jsSourceDir = path.join(process.cwd(), 'public', 'js');
  if (fs.existsSync(jsSourceDir)) {
    copyDir(jsSourceDir, jsOutDir);
    console.log('  ✓ Copied JavaScript files');
  }

  console.log('\n✅ Build complete!');
  console.log(`   Output directory: ${outDir}`);
  console.log(`   Total files: ${allPostIds.length + 1} HTML pages`);
  console.log('\nNext steps:');
  console.log('  1. Run: npm start');
  console.log('  2. Open: http://localhost:8000');
}

// Run build
build().catch(err => {
  console.error('Build failed:', err);
  process.exit(1);
});
