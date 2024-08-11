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
                    collapsed: true,
                    autogenerate: { directory: 'stm32' },
                },
                {
                    label: 'ESP32',
                    collapsed: true,
                    autogenerate: { directory: 'esp32' },
                },
                {
                    label: 'RP2040',
                    collapsed: true,
                    autogenerate: { directory: 'rp2040' },
                },
                {
                    label: 'Debugging',
                    collapsed: true,
                    autogenerate: { directory: 'debug' },
                },
                {
                    label: 'Debugging Hardware',
                    collapsed: true,
                    autogenerate: { directory: 'debug-hardware' },
                },
            ],
        }),
    ],
});
