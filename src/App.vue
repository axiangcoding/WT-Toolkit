<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { getVersion } from "@tauri-apps/api/app";
import { open } from "@tauri-apps/api/shell";
import BugReportDialog from "./components/dialog/BugReportDialog.vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api";
import { AppSettings } from "./schema";

const appVersion = ref("");
const latestVersion = ref("");
const isOutdated = ref(false);
const drawer = ref(false);
const feedbackDialog = ref(false);

const { t, locale } = useI18n();

const homeList = computed(() => [
  {
    icon: "mdi-home",
    title: t("app.nav_drawer.home"),
    to: "/",
  },
]);

const wtTool = computed(() => [
  {
    icon: "mdi-format-paint",
    title: t("app.nav_drawer.wt_skins"),
    to: "/wt-skins",
  },
  {
    icon: "mdi-crosshairs",
    title: t("app.nav_drawer.wt_sight"),
    to: "/wt-sight",
  },
  {
    icon: "mdi-package-variant",
    title: t("app.nav_drawer.wt_ext_cli"),
    to: "/wt-ext-cli",
  },
  {
    icon: "mdi-web",
    title: t("app.nav_drawer.wt_live"),
    to: "/wt-live",
  },
]);

const appInfo = computed(() => [
  {
    icon: "mdi-cog",
    title: t("app.nav_drawer.settings"),
    to: "/setting",
  },
]);

const followLinks = computed(() => [
  {
    text: t("app.follow_links.bilibili"),
    url: "https://space.bilibili.com/8696650",
    icon: "mdi-television-classic",
  },
  {
    text: t("app.follow_links.github"),
    url: "https://github.com/axiangcoding/WT-Toolkit",
    icon: "mdi-github",
  },
]);

onMounted(async () => {
  appVersion.value = await getVersion();

  // check if the app is outdated
  /*
  try {
    function normalizeVersion(version: string): string {
      return version.replace(/^v/, "").trim();
    }
    const rawAppVersion = await getVersion();
    appVersion.value = normalizeVersion(rawAppVersion);
    const response = await fetch("https://api.github.com/repos/axiangcoding/WT-Toolkit/releases/latest");
    const data = await response.json();
    latestVersion.value = normalizeVersion(data.tag_name);

    if (appVersion.value !== latestVersion.value) {
      isOutdated.value = true;
      console.log(`Outdated! The latest version is v${latestVersion.value} - current version is v${appVersion.value}`);
    } else {
      isOutdated.value = false;
      console.log(`Up to date. The latest version is v${latestVersion.value} - current version is v${appVersion.value}`);
    }
  } catch (error) {
    console.error("Failed to fetch the latest version:", error);
  }
    */

  let cfg: AppSettings = await invoke("get_app_config");
  if (cfg.language) {
    console.log(cfg.language);
    locale.value = cfg.language;
  }
});

async function jumpTo(url: string) {
  await open(url);
}

async function switchLanguage(target: string) {
  locale.value = target;
  await invoke("update_app_config", {
    key: "language",
    value: target,
  });
}
</script>

<template>
  <v-app>
    <v-app-bar :elevation="2" color="surface-variant">
      <template v-slot:prepend>
        <v-app-bar-nav-icon @click.stop="drawer = !drawer"></v-app-bar-nav-icon>
      </template>

      <v-app-bar-title>
        {{ t("app.title") }}
        <v-chip :color="isOutdated ? 'red' : 'green'" variant="flat" rounded>
          v{{ appVersion }}
        </v-chip>
      </v-app-bar-title>

      <template v-slot:append>
        <v-btn prepend-icon="mdi-bug" @click="feedbackDialog = true">
          <template v-slot:prepend>
            <v-icon></v-icon>
          </template>
          {{ t("app.report_bug") }}
        </v-btn>

        <v-menu open-on-hover>
          <template v-slot:activator="{ props }">
            <v-btn v-bind="props" prepend-icon="mdi-account-group">
              {{ t("app.follow") }}
            </v-btn>
          </template>

          <v-list>
            <v-list-item v-for="(item, index) in followLinks" :key="index" :value="index" append-icon="mdi-open-in-new"
              :prepend-icon="item.icon" @click="jumpTo(item.url)">
              {{ item.text }}
            </v-list-item>
          </v-list>
        </v-menu>
        <v-menu open-on-hover>
          <template v-slot:activator="{ props }">
            <v-btn v-bind="props" prepend-icon="mdi-translate">
              {{ t("app.language") }}
            </v-btn>
          </template>
          <v-list>
            <v-list-item @click="switchLanguage('en_us')">English</v-list-item>
            <v-list-item @click="switchLanguage('zh_cn')">简体中文</v-list-item>
          </v-list>
        </v-menu>
      </template>
    </v-app-bar>

    <v-navigation-drawer v-model="drawer" :location="$vuetify.display.mobile ? 'bottom' : undefined" temporary>
      <v-list nav active-strategy="single-leaf" activated="wt-skins" color="primary">
        <v-list-item v-for="item in homeList" :key="item.to" :to="item.to">
          <template v-slot:prepend>
            <v-icon :icon="item.icon"></v-icon>
          </template>
          <v-list-item-title>{{ item.title }}</v-list-item-title>
        </v-list-item>

        <v-list-subheader>
          {{ t("app.nav_drawer.sub_header.wt_tools") }}
        </v-list-subheader>
        <v-list-item v-for="item in wtTool" :key="item.to" :to="item.to">
          <template v-slot:prepend>
            <v-icon :icon="item.icon"></v-icon>
          </template>
          <v-list-item-title>{{ item.title }}</v-list-item-title>
        </v-list-item>
        <v-list-subheader>
          {{ t("app.nav_drawer.sub_header.app_info") }}
        </v-list-subheader>
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
