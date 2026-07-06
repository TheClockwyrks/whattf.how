// @ts-check
import { defineConfig } from "astro/config";
import mdx from "@astrojs/mdx";
import react from "@astrojs/react";
import sitemap from "@astrojs/sitemap";

// `astro build` runs Vite in production mode and writes production-optimized
// dependencies into the Vite cache dir — including React's production
// `jsx-dev-runtime`, whose `jsxDEV` is a no-op. If dev and build share one cache
// dir, running a build poisons the dev server's optimized deps and every React
// island then fails to hydrate with "_jsxDEV is not a function". Give build its
// own cache dir so the two can never clobber each other.
const isBuild = process.argv.includes("build");

// A static site. `site` is the production origin — used for canonical URLs, the
// sitemap, and RSS. Islands (React components with a `client:*` directive) are
// the only JavaScript shipped; everything else renders to static HTML.
export default defineConfig({
  site: "https://whattf.how",
  integrations: [mdx(), react(), sitemap()],
  vite: {
    cacheDir: isBuild ? "node_modules/.vite-build" : "node_modules/.vite",
  },
});
