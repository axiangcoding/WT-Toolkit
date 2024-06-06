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
const vuetify = createVuetify()

initFolder()

createApp(App).use(vuetify).use(router).mount('#app')
