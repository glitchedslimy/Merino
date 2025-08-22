import Paragraph from '@editorjs/paragraph';
import type { BlockTool, BlockAPI, API, BlockToolData, ConversionConfig } from '@editorjs/editorjs';

export default class CustomParagraphTool extends Paragraph implements BlockTool {
    constructor({ data, config, api, readOnly, block }: { data: BlockToolData, config: any, api: API, readOnly: boolean, block: BlockAPI }) {
        super({ data, config, api, readOnly, block });

        this.render = this.render.bind(this);
    }

    render(): HTMLElement {
        const element = super.render();
        
        element.addEventListener('keydown', async (event: KeyboardEvent) => {
            if (event.key === ' ') {
                const text = element.textContent;
                
                // This is the fixed regex to correctly capture all hashtags
                const levelMatch = text?.match(/^(#{1,6})$/);
                const matchBold = text?.match(/\*{.*?}\s/g) || []; 


                if (levelMatch) {
                    // The level is now correctly calculated from the captured group
                    const level = levelMatch[1].length;
                    const newText = '';
                    
                    const currentBlock = this.api.blocks.getBlockByIndex(this.api.blocks.getCurrentBlockIndex());

                    if (!currentBlock) {
                        return;
                    }

                    try {
                        await this.api.blocks.convert(
                            currentBlock.id,
                            'header',
                            {
                                text: newText,
                                level: level,
                            }
                        );
                        const newBlockIndex = this.api.blocks.getCurrentBlockIndex();
                        this.api.caret.setToBlock(newBlockIndex, "start");
                    } catch (e) {
                        console.error('Error during block conversion:', e);
                    }

                    event.preventDefault();
                }
            }

            if (event.key === '*') {
                setTimeout(() => {
                    const textNode = element.firstChild;
                    if (!textNode || textNode.nodeType !== Node.TEXT_NODE) return;

                    const textContent = textNode.textContent || '';
                    const boldMatch = textContent.match(/\*\*([^\*]+)\*\*$/);

                    if (boldMatch) {
                        const fullMatch = boldMatch[0];
                        const content = boldMatch[1];
                        const matchIndex = textContent.indexOf(fullMatch);
                        
                        // Create the new <strong> element
                        const strongTag = document.createElement('b');
                        strongTag.textContent = content;

                        // Create a new Range to perform the replacement
                        const range = document.createRange();
                        range.setStart(textNode, matchIndex);
                        range.setEnd(textNode, matchIndex + fullMatch.length);

                        // Delete the old text and insert the new strong tag
                        range.deleteContents();
                        range.insertNode(strongTag);

                        // Position the cursor at the end of the new tag
                        range.setStartAfter(strongTag);
                        range.collapse(true);
                        
                        const selection = window.getSelection();
                        selection?.removeAllRanges();
                        selection?.addRange(range);
                    }
                }, 0);
            }

        });
        
        return element;
    }
}