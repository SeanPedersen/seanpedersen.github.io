import styles from './TableOfContents.module.css'

export default function TableOfContents({ content }) {
    const getHeadings = (content) => {
        const regex = /<h([1-6]).*?>(.*?)<\/h[1-6]>/g;
        const headings = [];
        let match;

        while ((match = regex.exec(content)) !== null) {
            headings.push({
                level: Number(match[1]),
                text: match[2].replace(/<[^>]*>/g, ''),
                id: match[2].toLowerCase().replace(/[^a-z0-9]+/g, '-')
            });
        }

        return headings;
    };

    const headings = getHeadings(content);

    if (headings.length === 0) {
        return null;
    }

    return (
        <nav className={styles.toc}>
            <h2>Contents</h2>
            <ul>
                {headings.map((heading, index) => (
                    <li
                        key={index}
                        style={{ marginLeft: `${(heading.level - 1) * 12}px` }}
                    >
                        <a href={`#${heading.id}`}>{heading.text}</a>
                    </li>
                ))}
            </ul>
        </nav>
    );
}