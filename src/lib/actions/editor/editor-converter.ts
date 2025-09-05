import TurndownService from 'turndown';
import edjsHTML from 'editorjs-html';
import type { OutputData } from '@editorjs/editorjs';

// Initialize the necessary libraries once
const turndownService = new TurndownService({
    headingStyle: 'atx',
});
const edjsParser = edjsHTML();

turndownService.addRule('nbsp', {
    filter: node => node.nodeName === '#text' && node.nodeValue.includes('\u00A0'),
    replacement: content => content.replace(/\u00A0/g, ' ')
});

/**
 * Converts Editor.js JSON data to a Markdown string.
 * @param {OutputData | undefined} data The JSON output from Editor.js.
 * @returns {string} The converted markdown string.
 */
export function convertJsonToMarkdown(data: OutputData | undefined): string {
    if (!data || !data.blocks) {
        return '';
    }

    try {
        // 1. Convert Editor.js JSON data to an HTML array.
        const htmlString = edjsParser.parse(data);
        // 3. Use Turndown.js to convert the HTML string to Markdown.
        const markdown = turndownService.turndown(htmlString);

        return markdown;
    } catch (error) {
        console.error('Conversion from JSON to Markdown failed:', error);
        return '';
    }
}

