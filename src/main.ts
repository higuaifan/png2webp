import { createApp } from 'vue';
import './styles.css';
import App from './App.vue';
import { createMUI } from 'shuimo-ui';
import 'shuimo-ui/dist/style.css';

createApp(App).use(createMUI())
  .mount('#app');
