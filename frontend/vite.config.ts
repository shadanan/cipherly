import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import { purgeCss } from "vite-plugin-tailwind-purgecss";

export default defineConfig({
  plugins: [
    sveltekit(),
    purgeCss({
      safelist: {
        greedy: [/^hljs-/],
      },
    }),
  ],
  server: {
    proxy: {
      "/api": {
        target: "http://localhost:8000",
        changeOrigin: true,
        secure: false,
      },
    },
  },
});
