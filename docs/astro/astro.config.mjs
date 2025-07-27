// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import { ion } from "starlight-ion-theme";
import sitemap from '@astrojs/sitemap';

// https://astro.build/config
export default defineConfig({
  integrations: [
    starlight({
      title: 'Embedded Rust Examples',
      social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/Hecatron/Embedded.Rust.Examples' }],
      logo: {
        dark: "./src/assets/ion-logo.svg",
        light: "./src/assets/ion-logo-light.svg",
      },
      lastUpdated: true,
      pagination: false,
      sidebar: [
	{
          label: "[home] Home",
          link: "/",
        },
        {
          label: '[book] Boards',
          autogenerate: { directory: 'boards' },
        },
      ],
      customCss: [
        "@fontsource-variable/space-grotesk/index.css",
        "@fontsource/space-mono/400.css",
        "@fontsource/space-mono/700.css",
        "./src/styles/global.css",
      ],
      plugins: [
        ion({
          icons: {
            iconDir: "./src/icons",
          },
          footer: {
            text: "Hecatronic 2025",
            links: [
              {
                text: "Homepage",
                href: "https://www.hecatron.com",
              },
            ],
            icons: [
              {
                name: "github",
                href: "https://github.com/Hecatron/Embedded.Rust.Examples",
              },
            ],
          },
        })
      ],
    }),
  sitemap()
  ],
  output: "static",
});
