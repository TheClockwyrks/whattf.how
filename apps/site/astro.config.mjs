// @ts-check
import { defineConfig } from "astro/config";
import mdx from "@astrojs/mdx";
import react from "@astrojs/react";
import sitemap from "@astrojs/sitemap";

// A static site. `site` is the production origin — used for canonical URLs, the
// sitemap, and RSS. Islands (React components with a `client:*` directive) are
// the only JavaScript shipped; everything else renders to static HTML.
export default defineConfig({
  site: "https://whattf.how",
  integrations: [mdx(), react(), sitemap()],
});
