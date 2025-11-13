import { useState } from 'react'
import styles from './TableOfContents.module.css'

// Minimal HTML entity decoder for display purposes (keeps IDs unchanged)
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
            // Numeric entity
            const isHex = code[1] === 'x' || code[1] === 'X';
            const num = parseInt(code.slice(isHex ? 2 : 1), isHex ? 16 : 10);
            if (!isNaN(num)) {
                try { return String.fromCodePoint(num); } catch { return String.fromCharCode(num); }
            }
            return match;
        }
        // Named entity
        return Object.prototype.hasOwnProperty.call(named, code) ? named[code] : match;
    });
}

export default function TableOfContents({ content, title, titleId }) {
    const [isExpanded, setIsExpanded] = useState(false);
    const getHeadings = (content) => {
        const regex = /<h([1-6]).*?>(.*?)<\/h[1-6]>/g;
        const headings = [];
        let match;

        while ((match = regex.exec(content)) !== null) {
            const level = Number(match[1]);
            const rawText = match[2].replace(/<[^>]*>/g, '');
            // Decode HTML entities for both display and ID generation
            const decodedText = decodeHTMLEntities(rawText);
            // IDs must remain consistent with how they are generated in the post renderer
            const id = decodedText.toLowerCase().replace(/[^a-z0-9]+/g, '-');
            headings.push({ level, text: decodedText, id });
        }

        return headings;
    };

    const headings = getHeadings(content);

    if (headings.length === 0) {
        return null;
    }

    return (
        <nav className={styles.toc}>
            <div className={styles.tocHeader}>
                {title && titleId ? (
                    <h2><a href={`#${titleId}`}>{title}</a></h2>
                ) : (
                    <h2>Contents</h2>
                )}
                <button
                    className={styles.toggleButton}
                    onClick={() => setIsExpanded(!isExpanded)}
                    aria-expanded={isExpanded}
                    aria-label={isExpanded ? 'Collapse table of contents' : 'Expand table of contents'}
                >
                    {isExpanded ? '△' : '▽'}
                </button>
            </div>
            <ul className={`${styles.tocList} ${isExpanded ? styles.expanded : styles.collapsed}`}>
                {headings.map((heading, index) => (
                    <li
                        key={index}
                        style={{ marginLeft: `${Math.max(0, (heading.level - 1) * 12)}px` }}
                    >
                        <a href={`#${heading.id}`}>{heading.text}</a>
                    </li>
                ))}
            </ul>
        </nav>
    );
}