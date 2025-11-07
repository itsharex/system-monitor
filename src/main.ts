import { createApp } from "vue";
import { createPinia } from "pinia";
import naive from 'naive-ui'
import router from './router'
import './assets/main.css'
import 'vfonts/Lato.css'
import 'vfonts/FiraCode.css'
import App from "./App.vue";

const app = createApp(App);
const pinia = createPinia();

app.use(pinia);
app.use(router);
app.use(naive);
app.mount("#app");
