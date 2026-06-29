import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'

/** @type {import("@sveltejs/vite-plugin-svelte").SvelteConfig} */
export default {
  // Consult https://svelte.dev/docs#compile-time-svelte-preprocess
  // for more information about preprocessors
  preprocess: vitePreprocess(),
  // Override Svelte 5 / vite-plugin-svelte's default `'external'` css mode
  // for this project.
  //
  // With `'external'` (the default) the compiled css lives in a separate
  // `?svelte&type=style&lang.css` virtual module that vite-plugin-svelte
  // loads via `getModuleInfo(...).meta.svelte.css`. On cold `pnpm tauri dev`
  // starts that meta isn't populated yet when the css virtual module is
  // first requested — the plugin logs `failed to load virtual css module`
  // for every component (AppShell, Stage, AppearancePanel, etc.) and vite
  // then routes the raw Svelte file (the `<script>` block and all) through
  // the css pipeline, postcss errors on `Unknown word onMount`, and the
  // postcss pipeline aborts with an internal server error. The css DOES
  // land on the next request (HTTP 200), so the user-visible UI works, but
  // the stderr noise masks real errors and the postcss failure prevents
  // the very first paint from applying styles until the next reload.
  //
  // `emitCss: false` puts vite-plugin-svelte in css-`'injected'` mode,
  // which inlines css as a JS string in the same module the .svelte file
  // produces — so there is no parallel virtual-module load and no race.
  // Cost: one extra JS payload per component (~5KB pre-gzip for this
  // project's css); not material for a control-UI bundle, and the browser
  // only parses each chunk once anyway.
  vitePlugin: {
    emitCss: false,
  },
}
