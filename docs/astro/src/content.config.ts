import { defineCollection } from 'astro:content';
import { docsLoader } from '@astrojs/starlight/loaders';
//import { glob } from 'astro/loaders';
import { docsSchema } from '@astrojs/starlight/schema';

export const collections = {
  docs: defineCollection({
    loader: docsLoader(),
    //loader: glob({ pattern: '**/*.(md|mdx)', base: '../docs' }),
    schema: docsSchema()
  }),
};
