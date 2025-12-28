import { mdsvex } from 'mdsvex';
import adapter from 'svelte-adapter-bun';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: [vitePreprocess(), mdsvex()],

	kit: {
		adapter: adapter({
            out: './build',
            envPrefix: 'SITE_',
            precompress: true,
            serveAssets: true
        }),

        experimental: {
            remoteFunctions: true
        },

        appDir: 'qts',

        output: {
            bundleStrategy: 'split',
            preloadStrategy: 'modulepreload'
        },

        env: {
            dir: '../infra/env'
        },

        router: {
            type: 'pathname',
            resolution: 'server'
        },

        alias: {
            $lib: './src/lib',
            $components: './src/components',
            $sections: './src/sections',
            $i18n: './src/lib/i18n'
        }
	},
    compilerOptions: {
        runes: true,

        experimental: {
            async: true
        }
    },
	extensions: ['.svelte', '.svx']
};

export default config;
