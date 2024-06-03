import { createMemoryHistory, createRouter } from 'vue-router'



const routes = [
    { path: '/', name: "home", component: () => import("@/components/Home.vue") },
    { path: '/wt-paint', name: 'wt-paint', component: () => import("@/components/WTPaint.vue") },
]

const router = createRouter({
    history: createMemoryHistory(),
    routes,
})

export default router