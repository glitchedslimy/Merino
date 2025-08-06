import type { OutputData } from "@editorjs/editorjs";

export function convertJsonToMarkdown(data: OutputData | undefined): string {
    if(!data || !data.blocks) {
        return '';
    }

    const markdownBlocks = data.blocks.map(block => {
        switch(block.type) {
            case 'paragraph': { 
                return block.data.text || '';
            }
            case 'header': {
                const level = block.data.level || 1;
                const hashes = '#'.repeat(level);
                return `${hashes} ${block.data.text || ''}`;
            }
            case 'list': {
                const style = block.data.style;
                if(style === 'unordered' && block.data.items) {
                    
                    return block.data.items.map((item: { content: string}) => {
                        console.log("Item in lists", item)
                        return `* ${item.content}`
                    }).join('\n');
                } else if (style === 'ordered' && block.data.items){  
                    return block.data.items.map((item: { content: string }, index: number) => `${index + 1}. ${item.content}`)
                }
                return '';
            }
            default:
                console.warn(`Block type ${block.type} not supported by the markdown converter.`)
                return ''
        }
    });
    return markdownBlocks.join('\n\n');
}

export function convertMarkdownToJson(markdown: string): OutputData {
    if (!markdown) {
        return {
            time: Date.now(),
            blocks: [],
            version: '2.31.0-rc.7'
        }
    }

    const blocks: any[] = [];
    const lines = markdown.split('\n');
    let currentBlock: any | null = null;
    
    // Regular expressions for common Markdown elements
    const headerRegex = /^(#{1,3})\s(.+)/;
    const listRegex = /^[*+-]\s(.+)/;
    
    for (const line of lines) {
        // Skip empty lines
        if (line.trim() === '') {
            if (currentBlock && currentBlock.type !== 'list') {
                // End the current block and start a new one
                blocks.push(currentBlock);
                currentBlock = null;
            }
            continue;
        }

        const headerMatch = line.match(headerRegex);
        const listMatch = line.match(listRegex);
        
        if (headerMatch) {
            // If there's a current block, push it before starting a new one
            if (currentBlock) {
                blocks.push(currentBlock);
            }
            const level = headerMatch[1].length;
            const text = headerMatch[2];
            currentBlock = {
                type: 'header',
                data: {
                    text: text,
                    level: level
                }
            };
        } else if (listMatch) {
            // Handle lists
            if (currentBlock?.type === 'list') {
                // If we are already in a list, just add the new item
                currentBlock.data.items.push(listMatch[1]);
            } else {
                // If it's the start of a new list, push the current block (if any) and create a new list block
                if (currentBlock) {
                    blocks.push(currentBlock);
                }
                currentBlock = {
                    type: 'list',
                    data: {
                        style: 'unordered',
                        items: [listMatch[1]]
                    }
                };
            }
        } else {
            // Handle paragraphs
            if (currentBlock?.type === 'paragraph') {
                // Append text to the current paragraph block
                currentBlock.data.text += '\n' + line;
            } else {
                // If it's a new paragraph, push the current block (if any) and create a new paragraph block
                if (currentBlock) {
                    blocks.push(currentBlock);
                }
                currentBlock = {
                    type: 'paragraph',
                    data: {
                        text: line
                    }
                };
            }
        }
    }

    // Push the last block if it exists
    if (currentBlock) {
        blocks.push(currentBlock);
    }

    return {
        time: Date.now(),
        blocks: blocks,
        version: '2.31.0-rc.7' // Use your current version
    };
}