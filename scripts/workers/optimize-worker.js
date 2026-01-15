import { parentPort, workerData } from 'worker_threads';
import { execSync } from 'child_process';
import sharp from 'sharp';
import fs from 'fs';
import path from 'path';

const { type, filePath, relativePath } = workerData;

async function processFile() {
  try {
    let result = { success: true, type, relativePath };

    switch (type) {
      case 'css':
        execSync(`npx csso-cli ${filePath} -o ${filePath}`, { stdio: 'pipe' });
        break;

      case 'js':
        execSync(`npx terser ${filePath} -o ${filePath} --compress --mangle`, { stdio: 'pipe' });
        break;

      case 'html':
        execSync(`npx html-minifier-terser ${filePath} -o ${filePath} --collapse-whitespace --remove-comments --minify-css true --minify-js true`, { stdio: 'pipe' });
        break;

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
