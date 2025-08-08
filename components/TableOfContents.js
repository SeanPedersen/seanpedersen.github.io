import styles from './TableOfContents.module.css'

export default function TableOfContents({ content, title, titleId }) {
    const getHeadings = (content) => {
        const regex = /<h([1-6]).*?>(.*?)<\/h[1-6]>/g;
        const headings = [];
        let match;

        while ((match = regex.exec(content)) !== null) {
            const level = Number(match[1]);
            if (level < 2) continue; // skip H1 to avoid duplicating the page title
            const text = match[2].replace(/<[^>]*>/g, '');
            const id = text.toLowerCase().replace(/[^a-z0-9]+/g, '-');
            headings.push({ level, text, id });
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