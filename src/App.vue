<script setup lang="ts">

import { onMounted, ref } from 'vue';
const router = useRouter()
import { useRouter } from 'vue-router';
import { getVersion } from '@tauri-apps/api/app';


const appVersion = ref<string>('');

const drawer = ref(false);

function navigateTo(to: string) {
  router.push({ name: to })
}

onMounted(async () => {
  appVersion.value = await getVersion();
})

</script>

<template>
  <v-app>
    <v-app-bar :elevation="2" color="surface-variant">
      <template v-slot:prepend>
        <v-app-bar-nav-icon @click.stop="drawer = !drawer"></v-app-bar-nav-icon>
      </template>

      <v-app-bar-title>WT小助手
        <v-chip color="info" >
          v{{ appVersion }}
        </v-chip>

      </v-app-bar-title>

      <template v-slot:append>
        <v-btn icon="mdi-heart"></v-btn>

        <v-btn icon="mdi-magnify"></v-btn>

        <v-btn icon="mdi-dots-vertical"></v-btn>
      </template>
    </v-app-bar>

    <v-navigation-drawer v-model="drawer" :location="$vuetify.display.mobile ? 'bottom' : undefined" temporary>
      <v-list nav active-strategy="single-leaf">
        <v-list-item color="primary" key="home" value="home" @click="navigateTo('home')">
          <template v-slot:prepend>
            <v-icon icon="mdi-home"></v-icon>
          </template>
          <v-list-item-title>主页</v-list-item-title>
        </v-list-item>
        <v-list-subheader>小工具</v-list-subheader>
        <v-list-item color="primary" key="wt-paint" value="wt-paint" @click="navigateTo('wt-paint')">
          <template v-slot:prepend>
            <v-icon icon="mdi-palette-outline"></v-icon>
          </template>
          <v-list-item-title>涂装</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-navigation-drawer>
    <v-main class="align-center justify-center">
      <RouterView />
    </v-main>
    <!-- <v-footer>
      33的神秘APP
    </v-footer> -->
  </v-app>
</template>

<style scoped></style>
