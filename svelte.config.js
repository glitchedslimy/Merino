// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter(),
    alias: {
      '@atoms': './src/lib/components/atoms/index.ts',
      '@molecules': './src/lib/components/molecules/index.ts',
      '@organisms': './src/lib/components/organisms/index.ts',
      '@components/*': './src/lib/components/*',
      '@stores/*': './src/lib/stores/*',
      '@services/internal/*': './src/lib/services/internal/*',
      '@services/external/*': './src/lib/services/external/*'
    }
  },
};

export default config;
