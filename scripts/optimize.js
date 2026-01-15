import fs from 'fs';
import path from 'path';
import { execSync } from 'child_process';
import { Worker } from 'worker_threads';
import os from 'os';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const outDir = path.join(process.cwd(), 'out');
const MAX_WORKERS = os.cpus().length * 2;

console.log('Starting optimization...\n');

// Worker pool to process files
async function processFilesWithWorkers(files, type, label) {
  if (files.length === 0) return { total: 0, successful: 0, skipped: 0 };

  console.log(`[${label}] Processing ${files.length} files using ${MAX_WORKERS} workers...`);
  const startTime = Date.now();

  let completed = 0;
  let successful = 0;
  let skipped = 0;
  let activeWorkers = 0;
  const queue = [...files];
  const errors = [];

  await new Promise((resolve) => {
    function startWorker() {
      if (queue.length === 0) {
        if (activeWorkers === 0) {
          resolve();
        }
        return;
      }

      const file = queue.shift();
      activeWorkers++;

      const worker = new Worker(path.join(__dirname, 'workers', 'optimize-worker.js'), {
        workerData: {
          type,
          filePath: file.path,
          relativePath: file.relative
        }
      });

      worker.on('message', (result) => {
        if (result.success) {
          completed++;
          if (result.optimized) successful++;
          if (result.skipped) skipped++;

          if (type === 'html') {
            if (completed % 10 === 0 || completed === files.length) {
              console.log(`[${label}] [${completed}/${files.length}] files completed...`);
            }
          } else {
            console.log(`[${label}] [${completed}/${files.length}] ${result.relativePath}`);
          }
        } else {
          errors.push(`Failed to process ${result.relativePath}: ${result.error}`);
        }
        activeWorkers--;
        startWorker();
      });

      worker.on('error', (error) => {
        errors.push(`Worker error for ${file.relative}: ${error.message}`);
        activeWorkers--;
        startWorker();
      });

      worker.on('exit', (code) => {
        if (code !== 0) {
          errors.push(`Worker stopped with exit code ${code} for ${file.relative}`);
        }
      });
    }

    // Start initial batch of workers
    for (let i = 0; i < Math.min(MAX_WORKERS, files.length); i++) {
      startWorker();
    }
  });

  if (errors.length > 0) {
    console.error(`[${label}] ⚠️  Some files failed:`);
    errors.forEach(err => console.error(`  ${err}`));
  }

  const duration = ((Date.now() - startTime) / 1000).toFixed(2);
  console.log(`[${label}] ✓ Done in ${duration}s`);

  return { total: files.length, successful, skipped };
}

// 1. Minify CSS files
async function minifyCSS() {
  const cssFiles = [
    'styles/global.css',
    'styles/layout.css',
    'styles/utils.css',
    'styles/prism-tomorrow.css'
  ];

  const files = cssFiles
    .filter(f => fs.existsSync(path.join(outDir, f)))
    .map(f => ({
      path: path.join(outDir, f),
      relative: f
    }));

  await processFilesWithWorkers(files, 'css', 'CSS');
}

// 2. Minify JavaScript files
async function minifyJS() {
  const jsDir = path.join(outDir, 'js');
  if (!fs.existsSync(jsDir)) return;

  const jsFiles = fs.readdirSync(jsDir).filter(f => f.endsWith('.js'));
  const files = jsFiles.map(f => ({
    path: path.join(jsDir, f),
    relative: f
  }));

  await processFilesWithWorkers(files, 'js', 'JS');
}

// 3. Minify HTML files
async function minifyHTML() {
  const htmlFiles = [];

  function collectHTMLFiles(dir) {
    const entries = fs.readdirSync(dir, { withFileTypes: true });
    for (const entry of entries) {
      const fullPath = path.join(dir, entry.name);
      if (entry.isDirectory()) {
        collectHTMLFiles(fullPath);
      } else if (entry.name.endsWith('.html')) {
        htmlFiles.push({
          path: fullPath,
          relative: path.relative(outDir, fullPath)
        });
      }
    }
  }

  collectHTMLFiles(outDir);
  await processFilesWithWorkers(htmlFiles, 'html', 'HTML');
}

// 4. Optimize images
async function optimizeImages() {
  const imagesDir = path.join(outDir, 'images');
  if (!fs.existsSync(imagesDir)) return;

  const imageFiles = fs.readdirSync(imagesDir).map(f => ({
    path: path.join(imagesDir, f),
    relative: f
  }));

  const result = await processFilesWithWorkers(imageFiles, 'image', 'IMG');
  console.log(`[IMG] ${result.successful} optimized, ${result.skipped} skipped`);
}

// Run all optimization tasks in parallel
async function optimize() {
  try {
    const startTime = Date.now();
    console.log('Running optimizations in parallel...\n');

    await Promise.all([
      minifyCSS(),
      minifyJS(),
      minifyHTML(),
      optimizeImages()
    ]);

    const totalDuration = ((Date.now() - startTime) / 1000).toFixed(2);
    console.log(`\n✅ Optimization complete in ${totalDuration}s!`);

    // Calculate total size
    function getDirSize(dir) {
      let size = 0;
      const entries = fs.readdirSync(dir, { withFileTypes: true });
      for (const entry of entries) {
        const fullPath = path.join(dir, entry.name);
        if (entry.isDirectory()) {
          size += getDirSize(fullPath);
        } else {
          size += fs.statSync(fullPath).size;
        }
      }
      return size;
    }

    const totalSize = getDirSize(outDir);
    console.log(`\nTotal output size: ${(totalSize / 1024 / 1024).toFixed(2)} MB`);
  } catch (err) {
    console.error('Optimization failed:', err);
    process.exit(1);
  }
}

optimize();
