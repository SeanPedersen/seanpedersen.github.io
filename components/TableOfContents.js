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
    const getHeadings = (content) => {
        const regex = /<h([1-6]).*?>(.*?)<\/h[1-6]>/g;
        const headings = [];
        let match;

        while ((match = regex.exec(content)) !== null) {
            const level = Number(match[1]);
            if (level < 2) continue; // skip H1 to avoid duplicating the page title
            const rawText = match[2].replace(/<[^>]*>/g, '');
            // Display text should decode entities like &#x26; -> &
            const displayText = decodeHTMLEntities(rawText);
            // IDs must remain consistent with how they are generated in the post renderer
            const id = rawText.toLowerCase().replace(/[^a-z0-9]+/g, '-');
            headings.push({ level, text: displayText, id });
        }

        return headings;
    };

    const headings = getHeadings(content);

    if (headings.length === 0) {
        return null;
    }

    return (
        <nav className={styles.toc}>
            {title && titleId ? (
                <h2><a href={`#${titleId}`}>{title}</a></h2>
            ) : (
                <h2>Contents</h2>
            )}
            <ul>
                {headings.map((heading, index) => (
                    <li
                        key={index}
                        style={{ marginLeft: `${Math.max(0, (heading.level - 2) * 12)}px` }}
                    >
                        <a href={`#${heading.id}`}>{heading.text}</a>
                    </li>
                ))}
            </ul>
        </nav>
    );
}