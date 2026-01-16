import { parentPort, workerData } from 'worker_threads';
import sharp from 'sharp';
import fs from 'fs';
import path from 'path';
import { minify as minifyHTML } from 'html-minifier-terser';
import { minify as minifyJS } from 'terser';
import { minify as minifyCSS } from 'csso';

const { type, filePath, relativePath } = workerData;

async function processFile() {
  try {
    let result = { success: true, type, relativePath };

    switch (type) {
      case 'css': {
        const css = fs.readFileSync(filePath, 'utf8');
        const minified = minifyCSS(css);
        fs.writeFileSync(filePath, minified.css, 'utf8');
        break;
      }

      case 'js': {
        const js = fs.readFileSync(filePath, 'utf8');
        const minified = await minifyJS(js, {
          compress: true,
          mangle: true
        });
        fs.writeFileSync(filePath, minified.code, 'utf8');
        break;
      }

      case 'html': {
        const html = fs.readFileSync(filePath, 'utf8');
        const minified = await minifyHTML(html, {
          collapseWhitespace: true,
          removeComments: true,
          minifyCSS: true,
          minifyJS: true
        });
        fs.writeFileSync(filePath, minified, 'utf8');
        break;
      }

      case 'image':
        const ext = path.extname(filePath).toLowerCase();
        if (ext === '.png') {
          await sharp(filePath)
            .png({ quality: 85, compressionLevel: 9 })
            .toFile(filePath + '.tmp');
          fs.renameSync(filePath + '.tmp', filePath);
          result.optimized = true;
        } else if (ext === '.jpg' || ext === '.jpeg') {
          await sharp(filePath)
            .jpeg({ quality: 85, progressive: true })
            .toFile(filePath + '.tmp');
          fs.renameSync(filePath + '.tmp', filePath);
          result.optimized = true;
        } else if (ext === '.webp') {
          await sharp(filePath)
            .webp({ quality: 85 })
            .toFile(filePath + '.tmp');
          fs.renameSync(filePath + '.tmp', filePath);
          result.optimized = true;
        } else {
          result.skipped = true;
        }
        break;

      default:
        throw new Error(`Unknown file type: ${type}`);
    }

    parentPort.postMessage(result);
  } catch (error) {
    parentPort.postMessage({
      success: false,
      type,
      relativePath,
      error: error.message
    });
  }
}

processFile();
