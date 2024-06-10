import { createApp } from 'vue'

// Vuetify
import 'vuetify/styles'
import '@mdi/font/css/materialdesignicons.css'
import '@/styles/global.css'
import { createVuetify } from 'vuetify'
import router from './router'

// Components
import App from './App.vue'
import { initFolder } from './settings'
import { md3 } from 'vuetify/blueprints'
import { createI18n } from 'vue-i18n'

const vuetify = createVuetify(
    { blueprint: md3 }
)

const i18n = createI18n({})

initFolder()

const app = createApp(App)

app.use(vuetify)
    .use(i18n)
    .use(router)
    .mount('#app')
