import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
    integrations: [
        starlight({
            title: 'My Docs',
            social: {
                github: 'https://github.com/withastro/starlight',
            },
	    customCss: [
                './src/styles/theme-colors.css',
            ],
            sidebar: [
	        {
                    label: 'STM32',
                    autogenerate: { directory: 'stm32' },
                },
	        {
                    label: 'ESP32',
                    autogenerate: { directory: 'esp32' },
                },
                {
                    label: 'Debugging',
                    autogenerate: { directory: 'debug' },
                },
            ],
        }),
    ],
});
