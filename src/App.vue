<script setup lang="ts">
import { onMounted, ref } from "vue";
import { getVersion } from "@tauri-apps/api/app";
import { open } from "@tauri-apps/api/shell";
import BugReportDialog from "./components/dialog/BugReportDialog.vue";

const appVersion = ref("");
const drawer = ref(false);
const feedbackDialog = ref(false);

const homeList = [
  {
    icon: "mdi-home",
    title: "主页",
    to: "/",
  },
];

const wtTool = [
  {
    icon: "mdi-format-paint",
    title: "自定义涂装管理",
    to: "/wt-skins",
  },
  {
    icon: "mdi-crosshairs",
    title: "自定义瞄具管理",
    to: "/wt-sight",
  },
];

const appInfo = [
  {
    icon: "mdi-cog",
    title: "设置",
    to: "/setting",
  },
  {
    icon: "mdi-information",
    title: "关于",
    to: "/about",
  },
];

onMounted(async () => {
  appVersion.value = await getVersion();
});

async function jumpToBiliBili() {
  await open("https://space.bilibili.com/8696650");
}
</script>

<template>
  <v-app>
    <v-app-bar :elevation="2" color="surface-variant">
      <template v-slot:prepend>
        <v-app-bar-nav-icon @click.stop="drawer = !drawer"></v-app-bar-nav-icon>
      </template>

      <v-app-bar-title>
        战雷工具箱
        <v-chip color="green" variant="flat" rounded>
          v{{ appVersion }}
        </v-chip>
      </v-app-bar-title>

      <template v-slot:append>
        <v-btn prepend-icon="mdi-heart" @click="jumpToBiliBili">
          <template v-slot:prepend>
            <v-icon color="red"></v-icon>
          </template>
          关注作者 - 摸鱼又开摆的三三
        </v-btn>
        <v-btn prepend-icon="mdi-bug" @click="feedbackDialog = true">
          <template v-slot:prepend>
            <v-icon></v-icon>
          </template>
          反馈问题
        </v-btn>
      </template>
    </v-app-bar>

    <v-navigation-drawer
      v-model="drawer"
      :location="$vuetify.display.mobile ? 'bottom' : undefined"
      temporary
    >
      <v-list
        nav
        active-strategy="single-leaf"
        activated="wt-skins"
        color="primary"
      >
        <v-list-item v-for="item in homeList" :key="item.to" :to="item.to">
          <template v-slot:prepend>
            <v-icon :icon="item.icon"></v-icon>
          </template>
          <v-list-item-title>{{ item.title }}</v-list-item-title>
        </v-list-item>

        <v-list-subheader>战雷小工具</v-list-subheader>
        <v-list-item v-for="item in wtTool" :key="item.to" :to="item.to">
          <template v-slot:prepend>
            <v-icon :icon="item.icon"></v-icon>
          </template>
          <v-list-item-title>{{ item.title }}</v-list-item-title>
        </v-list-item>
        <v-list-subheader>APP 信息</v-list-subheader>
        <v-list-item v-for="item in appInfo" :key="item.to" :to="item.to">
          <template v-slot:prepend>
            <v-icon :icon="item.icon"></v-icon>
          </template>
          <v-list-item-title>{{ item.title }}</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-navigation-drawer>
    <v-main class="align-center justify-center">
      <RouterView />
    </v-main>
  </v-app>
  <BugReportDialog v-model="feedbackDialog" />
</template>

<style scoped></style>
