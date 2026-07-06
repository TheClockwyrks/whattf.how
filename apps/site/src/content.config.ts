import { defineCollection, z } from "astro:content";
import { glob } from "astro/loaders";

// The blog Content Collection. Posts are Markdown/MDX files under
// src/content/blog/; the file name (without extension) becomes the post's `id`
// and its URL slug. MDX posts may import and embed island components (e.g. wasm
// demos).
const blog = defineCollection({
  loader: glob({ base: "./src/content/blog", pattern: "**/*.{md,mdx}" }),
  schema: z.object({
    title: z.string(),
    description: z.string(),
    pubDate: z.coerce.date(),
    updatedDate: z.coerce.date().optional(),
    tags: z.array(z.string()).default([]),
    // Draft posts are excluded from the index, feed, and static routes.
    draft: z.boolean().default(false),
  }),
});

export const collections = { blog };
