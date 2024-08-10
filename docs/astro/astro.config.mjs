import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
  site: 'https://louisescher.github.io',
  base: '/starlight-ion-theme',
  integrations: [starlight({
    title: 'Ion Theme',
    logo: {
      src: './src/assets/ion-logo.svg'
    },
    social: {
      // TODO
      github: 'https://github.com/comet-analytics/ion-theme'
    },
    sidebar: [{
      label: '[home] Home',
      link: '/'
    }, {
      label: 'STM32',
      autogenerate: {
        directory: 'stm32'
      }
    }, {
      label: 'ESP32',
      autogenerate: {
        directory: 'esp32'
      }
    }, {
      label: 'Hardware', // can also be [book]
      autogenerate: {
        directory: 'hardware'
      }
    }],
    components: {
      ThemeProvider: './src/components/ThemeProvider.astro',
      ThemeSelect: './src/components/ThemeSelect.astro',
      SiteTitle: './src/components/SiteTitle.astro',
      Sidebar: './src/components/Sidebar.astro',
      Pagination: './src/components/Pagination.astro',
      Hero: './src/components/Hero.astro',
    },
    customCss: [
      '@fontsource-variable/space-grotesk/index.css',
      '@fontsource/space-mono/400.css',
      '@fontsource/space-mono/700.css',
      './src/styles/theme.css'
    ],
    expressiveCode: {
      themes: ['github-dark']
    },
    pagination: false,
    lastUpdated: true
  })],
  output: "static"
});