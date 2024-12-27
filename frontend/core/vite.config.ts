import { defineConfig } from 'vite';

const importMap = {
  //   lit: 'https://cdn.jsdelivr.net/gh/lit/dist@3/all/lit-all.min.js',
};

export default defineConfig({
  plugins: [
    {
      name: 'importmap-resolver',
      enforce: 'pre',
      resolveId(source) {
        // Check against your import map or conditions to rewrite the source
        if (source in importMap) {
          console.log(source);
          return { id: importMap[source], external: true };
        }
      },
    },
  ],
});
