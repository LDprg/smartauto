import { mount } from 'svelte';
import App from './App.svelte';

const app = mount(App, {
  target: document.getElementById('app')!,
  props: {
    name: 'test123',
  },
});

export default app;
