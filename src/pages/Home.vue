<script setup lang="ts">
import FunctionCard from "../components/card/FunctionCard.vue";
import LinkCard from "../components/card/LinkCard.vue";

const commonFunctions = [
  {
    key: "wt-skins",
    icon: "mdi-format-paint",
    title: "自定义涂装管理",
    subtitle: "管理《战争雷霆》的自定义涂装",
    description:
      "战雷的自定义涂装安装/卸载太麻烦？不知道游戏的目录在哪里？想快捷管理自定义涂装？试试这个工具吧",
  },
  {
    key: "wt-sight",
    icon: "mdi-crosshairs",
    title: "自定义瞄具管理",
    subtitle: "管理《战争雷霆》的自定义瞄具",
    description: "战雷的自定义瞄具安装/卸载太麻烦？试试这个工具吧",
  },
  {
    key: "setting",
    icon: "mdi-cog",
    title: "设置",
    subtitle: "小工具需要提前进行设置才能正常使用",
    description:
      "小工具的正确运行需要一些设置，比如设置战雷的安装目录，设置目录等",
  },
  {
    key: "about",
    icon: "mdi-information",
    title: "关于APP",
    subtitle: "关于这个APP的一些信息",
    description: "由 B站UP主 摸鱼又开摆的三三 开发的一个小工具",
  },
];

const usefulLinks = [
  {
    key: "wtlive",
    url: "https://live.warthunder.com/",
    isOfficial: true,
    title: "战雷官方 Live",
    description:
      "《战争雷霆》官方社区，玩家可以在这里分享自己的游戏截图、视频、涂装等内容",
  },
  {
    key: "wtwiki",
    url: "https://wiki.warthunder.com/",
    isOfficial: true,
    title: "战雷官方百科",
    description:
      "《战争雷霆》官方百科，包含了游戏的各种信息，比如车辆的性能数据、战术技巧等",
  },
  {
    key: "wtforum",
    url: "https://forum.warthunder.com/",
    isOfficial: true,
    title: "战雷官方论坛",
    description: "《战争雷霆》官方论坛，玩家可以在这里交流游戏心得、提出建议等",
  },
  {
    key: "wtliker",
    url: "https://wtliker.com/",
    isOfficial: false,
    title: "战雷涂装中文站",
    description:
      "国内最大的《战争雷霆》玩家创作研究、交流学习和分享涂装语音包等自定义内容的专业中文社交平台",
  },
];

import image1 from "@/assets/images/worldwar_window_bg_battle_for_caen.png";
import image2 from "@/assets/images/worldwar_window_bg_chinese_farm.png";
import image3 from "@/assets/images/worldwar_window_bg_el_alamein.png";

const images = [image1, image2, image3];

const banner = images.map((src) => ({
  src,
  cover: true,
}));
</script>

<template>
  <v-carousel hide-delimiters height="180" :cycle="true">
    <v-carousel-item
      v-for="(item, i) in banner"
      :key="i"
      :src="item.src"
      :cover="item.cover"
      @contextmenu.prevent
    ></v-carousel-item>
  </v-carousel>
  <v-container>
    <v-row>
      <v-col cols="12">
        <span class="text-h5 border-b-lg border-primary">常用的功能</span>
      </v-col>
      <v-col cols="4" v-for="item in commonFunctions" :key="item.key">
        <FunctionCard :data="item"></FunctionCard>
      </v-col>
      <v-col cols="12">
        <span class="text-h5 border-b-lg border-primary">快捷链接</span>
      </v-col>
      <v-col cols="4" v-for="item in usefulLinks" :key="item.key">
        <LinkCard :data="item"></LinkCard>
      </v-col>
    </v-row>
    <v-footer app color="surface-light" class="justify-center">
      <strong>由 摸鱼又开摆的三三 开发，保留所有权利</strong>
    </v-footer>
  </v-container>
</template>

<style scoped></style>
