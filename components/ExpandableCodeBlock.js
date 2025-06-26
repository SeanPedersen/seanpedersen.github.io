import { useState } from 'react';
import styles from './ExpandableCodeBlock.module.css';

export default function ExpandableCodeBlock({ children, lineCount }) {
    const [isExpanded, setIsExpanded] = useState(false);
    const [copied, setCopied] = useState(false);

    const extractCodeText = (htmlContent) => {
        // Create a temporary div to parse the HTML
        const tempDiv = document.createElement('div');
        tempDiv.innerHTML = htmlContent;

        // Find the code element and extract its text content
        const codeElement = tempDiv.querySelector('code');
        return codeElement ? codeElement.textContent : '';
    };

    const handleCopy = async () => {
        try {
            const codeText = extractCodeText(children);
            await navigator.clipboard.writeText(codeText);
            setCopied(true);
            setTimeout(() => setCopied(false), 2000);
        } catch (err) {
            console.error('Failed to copy code:', err);
        }
    };

    const CopyIcon = ({ copied }) => (
        <svg
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            {copied ? (
                <path
                    d="M20 6L9 17l-5-5"
                    stroke="currentColor"
                    strokeWidth="2.5"
                    strokeLinecap="round"
                    strokeLinejoin="round"
                />
            ) : (
                <>
                    <rect
                        x="3"
                        y="3"
                        width="13"
                        height="13"
                        rx="2"
                        stroke="currentColor"
                        strokeWidth="1.5"
                        fill="none"
                    />
                    <rect
                        x="7"
                        y="7"
                        width="13"
                        height="13"
                        rx="2"
                        stroke="currentColor"
                        strokeWidth="1.5"
                        fill="none"
                    />
                </>
            )}
        </svg>
    );

    return (
        <div className={styles.codeBlockWrapper}>
            <div className={styles.codeBlockContainer}>
                <div
                    className={`${styles.codeBlock} ${!isExpanded && lineCount > 20 ? styles.collapsed : ''}`}
                    dangerouslySetInnerHTML={{ __html: children }}
                />
                {!isExpanded && lineCount > 20 && <div className={styles.fadeOverlay} />}
            </div>
            <button
                className={lineCount > 20 ? isExpanded ? styles.copyButtonExpanded : styles.copyButton : styles.copyButtonNormal}
                onClick={handleCopy}
                title="Copy code"
            >
                <CopyIcon copied={copied} />
            </button>
            {lineCount > 20 && (
                <button
                    className={styles.expandButton}
                    onClick={() => setIsExpanded(!isExpanded)}
                >
                    {isExpanded ? 'Show Less' : `Show More (${lineCount} lines)`}
                </button>
            )}
        </div>
    );
}
