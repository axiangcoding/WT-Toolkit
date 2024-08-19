import { createApp } from "vue";

// Vuetify
import "vuetify/styles";
import "@mdi/font/css/materialdesignicons.css";
import "@/styles/global.css";
import { createVuetify } from "vuetify";
import router from "./router";

// Posthog
import posthogPlugin from "./plugins/posthog";

// Components
import App from "./App.vue";
import { md3 } from "vuetify/blueprints";
import i18n from "./i18n";

const vuetify = createVuetify({ blueprint: md3 });

const app = createApp(App);

app.use(vuetify).use(i18n).use(router).use(posthogPlugin).mount("#app");
