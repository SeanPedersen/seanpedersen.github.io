const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');
const sharp = require('sharp');

const outDir = path.join(process.cwd(), 'out');

console.log('Starting optimization...\n');

// 1. Minify CSS files
console.log('Minifying CSS files...');
const cssFiles = [
  'styles/global.css',
  'styles/layout.css',
  'styles/utils.css',
  'styles/prism-tomorrow.css'
];

for (const cssFile of cssFiles) {
  const filePath = path.join(outDir, cssFile);
  if (fs.existsSync(filePath)) {
    try {
      execSync(`npx csso-cli ${filePath} -o ${filePath}`, { stdio: 'inherit' });
      console.log(`  ✓ Minified ${cssFile}`);
    } catch (err) {
      console.error(`  ✗ Failed to minify ${cssFile}`);
    }
  }
}

// 2. Minify JavaScript files
console.log('\nMinifying JavaScript files...');
const jsDir = path.join(outDir, 'js');
if (fs.existsSync(jsDir)) {
  const jsFiles = fs.readdirSync(jsDir).filter(f => f.endsWith('.js'));
  for (const jsFile of jsFiles) {
    const filePath = path.join(jsDir, jsFile);
    try {
      execSync(`npx terser ${filePath} -o ${filePath} --compress --mangle`, { stdio: 'inherit' });
      console.log(`  ✓ Minified ${jsFile}`);
    } catch (err) {
      console.error(`  ✗ Failed to minify ${jsFile}`);
    }
  }
}

// 3. Minify HTML files
console.log('\nMinifying HTML files...');
function minifyHTMLFiles(dir) {
  const entries = fs.readdirSync(dir, { withFileTypes: true });

  for (const entry of entries) {
    const fullPath = path.join(dir, entry.name);

    if (entry.isDirectory()) {
      minifyHTMLFiles(fullPath);
    } else if (entry.name.endsWith('.html')) {
      try {
        execSync(`npx html-minifier-terser ${fullPath} -o ${fullPath} --collapse-whitespace --remove-comments --minify-css true --minify-js true`, { stdio: 'inherit' });
        console.log(`  ✓ Minified ${path.relative(outDir, fullPath)}`);
      } catch (err) {
        console.error(`  ✗ Failed to minify ${path.relative(outDir, fullPath)}`);
      }
    }
  }
}
minifyHTMLFiles(outDir);

// 4. Optimize images
console.log('\nOptimizing images...');
const imagesDir = path.join(outDir, 'images');

async function optimizeImages() {
  if (!fs.existsSync(imagesDir)) {
    console.log('  No images directory found');
    return;
  }

  const imageFiles = fs.readdirSync(imagesDir);

  for (const imageFile of imageFiles) {
    const filePath = path.join(imagesDir, imageFile);
    const ext = path.extname(imageFile).toLowerCase();

    try {
      if (ext === '.png') {
        await sharp(filePath)
          .png({ quality: 85, compressionLevel: 9 })
          .toFile(filePath + '.tmp');
        fs.renameSync(filePath + '.tmp', filePath);
        console.log(`  ✓ Optimized ${imageFile}`);
      } else if (ext === '.jpg' || ext === '.jpeg') {
        await sharp(filePath)
          .jpeg({ quality: 85, progressive: true })
          .toFile(filePath + '.tmp');
        fs.renameSync(filePath + '.tmp', filePath);
        console.log(`  ✓ Optimized ${imageFile}`);
      } else if (ext === '.webp') {
        await sharp(filePath)
          .webp({ quality: 85 })
          .toFile(filePath + '.tmp');
        fs.renameSync(filePath + '.tmp', filePath);
        console.log(`  ✓ Optimized ${imageFile}`);
      } else {
        console.log(`  ⊘ Skipped ${imageFile} (${ext})`);
      }
    } catch (err) {
      console.error(`  ✗ Failed to optimize ${imageFile}: ${err.message}`);
    }
  }
}

// Run async optimization
optimizeImages().then(() => {
  console.log('\n✅ Optimization complete!');

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
}).catch(err => {
  console.error('Optimization failed:', err);
  process.exit(1);
});
