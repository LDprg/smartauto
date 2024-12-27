import commonjs from '@rollup/plugin-commonjs';
import resolve from '@rollup/plugin-node-resolve';
import svelte from 'rollup-plugin-svelte';
import { sveltePreprocess } from 'svelte-preprocess';
import terser from '@rollup/plugin-terser';
import typescript from '@rollup/plugin-typescript';

const production = !process.env.ROLLUP_WATCH;

export default {
  input: ['src/App.svelte', 'src/main.ts'],
  output: {
    dir: 'build',
    format: 'es',
    sourcemap: true,
  },
  // external: ["svelte"]s,
  plugins: [
    svelte({
      // This tells svelte to run some preprocessing
      preprocess: sveltePreprocess({
        postcss: true, // And tells it to specifically run postcss!
        // typescript: {
        //   tsconfigFile: './tsconfig.json'
        // }
      }),
      emitCss: false,
    }),
    // Tell any third-party plugins that we're building for the browser
    resolve({
      emitCss: false,
      browser: true,
      // exportConditions: ['svelte'],
      // extensions: ['.svelte'],
    }),
    commonjs(),
    typescript(),

    production && terser(),
  ],
};
