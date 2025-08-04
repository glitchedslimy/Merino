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
      '@atoms': './src/shared/components/atoms/index.ts',
      '@molecules': './src/shared/components/molecules/index.ts',
      '@organisms': './src/shared/components/organisms/index.ts',
      '@components/*': './src/shared/components/*',
      '@stores/*': './src/shared/stores/*',
      '@scripts/*': './src/shared/scripts/*'
    }
  },
};

export default config;
