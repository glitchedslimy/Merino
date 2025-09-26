import { derived, writable } from 'svelte/store';

type Messages = Record<string, any>;

const locales: Record<string, () => Promise<Messages>> = {
    en: () => import('./locales/en.json'),
    es: () => import('./locales/es.json')
}

export const locale = writable<'en' | 'es'>('en');
const messages = writable<Messages>({});

locale.subscribe(async (lang) => {
    const mod = await locales[lang]();
    messages.set(mod.default);
})

function format(str: string, params?: Record<string, any>) {
    if (!params) return str;
    return str.replace(/\{(.*?)\}/g, (_, key) => params[key] ?? '');
}

function plural(
    forms: Record<string, string>,
    count: number,
    params?: Record<string, any>
) {
    let key: string = 'other';
    if (count === 0 && forms.zero) key = "zero";
    else if (count === 1 && forms.one) key = "one";
    return format(forms[key], { ...params, count });
}

export const t = derived(
    [messages, locale],
    ([$messages]) => (key: string, params?: Record<string, any>): string => {
        const keys = key.split('.');
        let value: any = $messages;

        for (const k of keys) value = value?.[k];
        if (!value) return key;

        if (typeof value === 'object' && 'other' in value) {
            const count = params?.count ?? 0;
            return plural(value, count, params);
        }

        return format(value, params);
    }
)