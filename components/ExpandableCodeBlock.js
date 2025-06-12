import { useState } from 'react';
import styles from './ExpandableCodeBlock.module.css';

export default function ExpandableCodeBlock({ children, lineCount }) {
    const [isExpanded, setIsExpanded] = useState(false);

    if (lineCount <= 20) {
        return <div dangerouslySetInnerHTML={{ __html: children }} />;
    }

    return (
        <div className={styles.expandableContainer}>
            <div
                className={`${styles.codeBlock} ${!isExpanded ? styles.collapsed : ''}`}
                dangerouslySetInnerHTML={{ __html: children }}
            />
            {!isExpanded && <div className={styles.fadeOverlay} />}
            <button
                className={styles.expandButton}
                onClick={() => setIsExpanded(!isExpanded)}
            >
                {isExpanded ? 'Show Less' : `Show More (${lineCount} lines)`}
            </button>
        </div>
    );
}
