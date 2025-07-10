import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    // Consult https://svelte.dev/docs/kit/integrations
    // for more information about preprocessors
    preprocess: vitePreprocess(),

    kit: {
        adapter: adapter({
            // Pages will be built to this directory
            pages: 'build',
            // Assets will be built here
            assets: 'build',
            // Fallback page for SPA mode
            fallback: 'index.html',
            // Precompress files with gzip and brotli
            precompress: false,
            // Don't require all routes to be prerenderable
            strict: false
        })
    }
};

export default config;