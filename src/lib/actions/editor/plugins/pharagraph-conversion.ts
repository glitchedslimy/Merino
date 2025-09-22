import Paragraph from '@editorjs/paragraph';
import type { BlockTool, BlockAPI, API, BlockToolData, ConversionConfig } from '@editorjs/editorjs';

export default class CustomParagraphTool extends Paragraph implements BlockTool {
    constructor({ data, config, api, readOnly, block }: { data: BlockToolData, config: any, api: API, readOnly: boolean, block: BlockAPI }) {
        super({ data, config, api, readOnly, block });

        this.render = this.render.bind(this);
    }

    static get conversionConfig(): ConversionConfig {
        return {
            export: (data) => data.text,
            import: (string) => ({ text: string })
        };
    }

    render(): HTMLElement {
        const element = super.render();

        element.addEventListener('keydown', async (event: KeyboardEvent) => {
            // Lógica para todos los atajos de Markdown (ejecutar en el evento de espacio)
            if (event.key === ' ') {
                const text = element.textContent?.trim();
                if (!text) return;

                const currentBlock = this.api.blocks.getBlockByIndex(this.api.blocks.getCurrentBlockIndex());
                if (!currentBlock) return;

                // Comprobación más específica: Checklist
                const checklistMatch = text.match(/^-\[]$/);
                if (checklistMatch) {
                    event.preventDefault();
                    try {
                        await this.api.blocks.convert(currentBlock.id, 'list', {
                            style: 'checklist',
                            items: [{
                                text: '',
                                checked: false
                            }]
                        });
                        this.api.caret.setToBlock(this.api.blocks.getCurrentBlockIndex(), "start");
                    } catch (e) {
                        console.error('Error during block conversion to checklist:', e);
                    }
                    return;
                }

                // Comprobación más general: Unordered list
                const ulMatch = text.match(/^-$/);
                if (ulMatch) {
                    event.preventDefault();
                    try {
                        await this.api.blocks.convert(currentBlock.id, 'list', { style: 'unordered', items: [''] });
                        this.api.caret.setToBlock(this.api.blocks.getCurrentBlockIndex(), "start");
                    } catch (e) {
                        console.error('Error during block conversion to list:', e);
                    }
                    return;
                }

                // Lógica para listas ordenadas
                const olMatch = text.match(/^(\d+\.)$/);
                if (olMatch) {
                    event.preventDefault();
                    try {
                        await this.api.blocks.convert(currentBlock.id, 'list', { style: 'ordered', items: [''] });
                        this.api.caret.setToBlock(this.api.blocks.getCurrentBlockIndex(), "start");
                    } catch (e) {
                        console.error('Error during block conversion to ordered list:', e);
                    }
                    return;
                }

                // Lógica para Headers
                const levelMatch = text.match(/^(#{1,6})$/);
                if (levelMatch) {
                    event.preventDefault();
                    const level = levelMatch[1].length;
                    try {
                        await this.api.blocks.convert(currentBlock.id, 'header', { text: '', level: level });
                        this.api.caret.setToBlock(this.api.blocks.getCurrentBlockIndex(), "start");
                    } catch (e) {
                        console.error('Error during block conversion to header:', e);
                    }
                    return;
                }
            }

            // Lógica para Bold (negritas)
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

                        const strongTag = document.createElement('b');
                        strongTag.textContent = content;

                        const range = document.createRange();
                        range.setStart(textNode, matchIndex);
                        range.setEnd(textNode, matchIndex + fullMatch.length);
                        range.deleteContents();
                        range.insertNode(strongTag);

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