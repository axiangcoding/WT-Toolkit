<script setup lang="ts">

import { onMounted, ref } from 'vue';
import { getVersion } from '@tauri-apps/api/app';
import { open } from '@tauri-apps/api/shell';
import { getAppSettings } from './settings';

const appVersion = ref('');

const drawer = ref(false);

const feedbackDialog = ref(false);

import qqGroupQrCode from '@/assets/images/qrcode.jpg';

onMounted(async () => {
  appVersion.value = await getVersion();
  await getAppSettings();
})

async function jumpToBiliBili() {
  await open('https://space.bilibili.com/8696650')
}

</script>

<template>
  <v-app>
    <v-app-bar :elevation="2" color="surface-variant">
      <template v-slot:prepend>
        <v-app-bar-nav-icon @click.stop="drawer = !drawer"></v-app-bar-nav-icon>
      </template>

      <v-app-bar-title>三三的战雷小工具
        <v-chip color="info">
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
          反馈问题</v-btn>
      </template>
    </v-app-bar>

    <v-navigation-drawer v-model="drawer" :location="$vuetify.display.mobile ? 'bottom' : undefined" temporary>
      <v-list nav active-strategy="single-leaf" activated="wt-skins" color="primary">
        <v-list-item to="/">
          <template v-slot:prepend>
            <v-icon icon="mdi-home"></v-icon>
          </template>
          <v-list-item-title>主页</v-list-item-title>
        </v-list-item>
        <v-list-subheader>战雷小工具</v-list-subheader>
        <v-list-item to="wt-skins">
          <template v-slot:prepend>
            <v-icon icon="mdi-palette-outline"></v-icon>
          </template>
          <v-list-item-title>自定义涂装管理</v-list-item-title>
        </v-list-item>
        <v-list-subheader>APP 信息</v-list-subheader>
        <v-list-item to="setting">
          <template v-slot:prepend>
            <v-icon icon="mdi-cog"></v-icon>
          </template>
          <v-list-item-title>设置</v-list-item-title>
        </v-list-item>
        <v-list-item to="about">
          <template v-slot:prepend>
            <v-icon icon="mdi-information-outline"></v-icon>
          </template>
          <v-list-item-title>关于</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-navigation-drawer>
    <v-main class="align-center justify-center">
      <RouterView />
    </v-main>
  </v-app>
  <v-dialog v-model="feedbackDialog" width="auto" transition="dialog-top-transition">
    <v-card max-width="400" prepend-icon="mdi-bug" s>
      <template v-slot:title>
        报告 BUG
      </template>
      <v-card-text>
        加入开发者QQ群反馈问题

        <v-img :src="qqGroupQrCode" width="auto" />
      </v-card-text>
      <v-card-actions><v-btn color="success" text="好的" @click="feedbackDialog = false"></v-btn></v-card-actions>
    </v-card>
  </v-dialog>
</template>

<style scoped></style>
