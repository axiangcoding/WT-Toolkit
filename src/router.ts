import { createMemoryHistory, createRouter } from 'vue-router'



const routes = [
    { path: '/', name: "home", component: () => import("@/components/Home.vue") },
    { path: '/wt-paint', name: 'wt-paint', component: () => import("@/components/WTPaint.vue") },
    { path: '/about', name: 'about', component: () => import("@/components/About.vue") },
    { path: '/setting', name: 'setting', component: () => import("@/components/Setting.vue") }
]

const router = createRouter({
    history: createMemoryHistory(),
    routes,
})

export default router