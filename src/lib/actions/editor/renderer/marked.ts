export const renderer = {
  strong(text: any) {
   const content = typeof text === 'object' && text.text ? text.text : text;
    return `<b>${content}</b>`;
  },
  em(text: any) {
        // Check if the input is an object and get the text property
        const content = typeof text === 'object' && text.text ? text.text : text;
        return `<i>${content}</i>`;
    }
};