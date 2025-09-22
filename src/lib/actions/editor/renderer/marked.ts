export const renderer = {
  strong(text: any) {
    const content = typeof text === 'object' && text.text ? text.text : text;
    return `<b>${content}</b>`;
  },
  em(text: any) {
    const content = typeof text === 'object' && text.text ? text.text : text;
    return `<i>${content}</i>`;
  },
  link(args: { href: string; text: string }) {
    if (args.text === 'Embed Source') {
      return `<iframe src="${args.href}" style="width:100%; height:600px;" frameborder="0" allowfullscreen></iframe>`;
    }
    return `<a href="${args.href}">${args.text}</a>`;
  },
  blockquote(quote: any) {
    if (typeof quote !== 'string') {
      return `<blockquote><p>${quote.text ?? ''}</p></blockquote>`;
    }
    const match = quote.match(/(.+)—\s*(.+)/);
    let quoteText = quote;
    let citeHtml = '';
    if (match) {
      quoteText = match[1].trim();
      const citeText = match[2].trim();
      if (citeText.length > 0 && citeText !== 'undefined') {
        citeHtml = `<br><footer><cite>— ${citeText}</cite></footer>`;
      }
    }
    return `<blockquote><p>${quoteText}</p>${citeHtml}</blockquote>`;
  },
  listitem(text: string | undefined, task: boolean, checked: boolean) {
    const content = text ?? '';
    if (task) {
      return `<li data-checked="${checked}"><input type="checkbox" ${checked ? 'checked' : ''}> ${content}</li>`;
    }
    return `<li>${content}</li>`;
  },
  list(items: any, parentStyle: 'ordered' | 'unordered' | 'checklist' = 'unordered') {
    // normalize: ensure items is array
    const itemArray = Array.isArray(items) ? items : items.items || [];

    const renderItem = (item: any): string => {
      if (!item) return '';
      const text = item.content || item.text || ' ';
      let nested = '';
      if (Array.isArray(item.items) && item.items.length > 0) {
        const nestedStyle = item.style || 'unordered';
        nested = this.list(item.items, nestedStyle);
      }
      if (item.meta?.checked !== undefined) {
        return `<li data-checked="${item.meta.checked}"><input type="checkbox" ${item.meta.checked ? 'checked' : ''}> ${text}${nested}</li>`;
      }
      return `<li>${text}${nested}</li>`;
    };

    const type = parentStyle === 'ordered' ? 'ol' : 'ul';
    const className = parentStyle === 'checklist' ? ' class="task-list"' : '';
    return `<${type}${className}>${itemArray.map(renderItem).join('')}</${type}>`;
  }
};
