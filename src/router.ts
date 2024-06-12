import { createMemoryHistory, createRouter } from "vue-router";

const routes = [
  { path: "/", name: "home", component: () => import("@/pages/Home.vue") },
  {
    path: "/wt-sight",
    name: "wt-sight",
    component: () => import("@/pages/WTSights.vue"),
  },
  {
    path: "/wt-skins",
    name: "wt-skins",
    component: () => import("@/pages/WTSkins.vue"),
  },
  {
    path: "/about",
    name: "about",
    component: () => import("@/pages/About.vue"),
  },
  {
    path: "/setting",
    name: "setting",
    component: () => import("@/pages/Setting.vue"),
  },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
