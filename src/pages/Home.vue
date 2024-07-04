<script setup lang="ts">
import FunctionCard from "../components/card/FunctionCard.vue";
import LinkCard from "../components/card/LinkCard.vue";

const { t } = useI18n();

const commonFunctions = computed(() => [
  {
    key: "wt-skins",
    icon: "mdi-format-paint",
    title: t("home.function.wt_skins.title"),
    subtitle: t("home.function.wt_skins.subtitle"),
    description: t("home.function.wt_skins.description"),
  },
  {
    key: "wt-sight",
    icon: "mdi-crosshairs",
    title: t("home.function.wt_sight.title"),
    subtitle: t("home.function.wt_sight.subtitle"),
    description: t("home.function.wt_sight.description"),
  },
  {
    key: "wt-ext-cli",
    icon: "mdi-package-variant",
    title: t("home.function.wt_ext_cli.title"),
    subtitle: t("home.function.wt_ext_cli.subtitle"),
    description: t("home.function.wt_ext_cli.description"),
  },
  {
    key: "setting",
    icon: "mdi-cog",
    title: t("home.function.setting.title"),
    subtitle: t("home.function.setting.subtitle"),
    description: t("home.function.setting.description"),
  },
  {
    key: "about",
    icon: "mdi-information",
    title: t("home.function.about.title"),
    subtitle: t("home.function.about.subtitle"),
    description: t("home.function.about.description"),
  },
]);

const usefulLinks = computed(() => [
  {
    key: "wtlive",
    url: "https://live.warthunder.com/",
    isOfficial: true,
    title: t("home.link.wtlive.title"),
    description: t("home.link.wtlive.description"),
  },
  {
    key: "wtwiki",
    url: "https://wiki.warthunder.com/",
    isOfficial: true,
    title: t("home.link.wtwiki.title"),
    description: t("home.link.wtwiki.description"),
  },
  {
    key: "wtforum",
    url: "https://forum.warthunder.com/",
    isOfficial: true,
    title: t("home.link.wtforum.title"),
    description: t("home.link.wtforum.description"),
  },
  {
    key: "wtliker",
    url: "https://wtliker.com/",
    isOfficial: false,
    title: t("home.link.wtliker.title"),
    description: t("home.link.wtliker.description"),
  },
  {
    key: "wtdata",
    url: "https://wt.controlnet.space/",
    isOfficial: false,
    title: t("home.link.wtdata.title"),
    description: t("home.link.wtdata.description"),
  },
  {
    key: "local8111",
    url: "http://localhost:8111",
    isOfficial: true,
    isLocal: true,
    title: t("home.link.local8111.title"),
    description: t("home.link.local8111.description"),
  },
]);

import image1 from "@/assets/images/worldwar_window_bg_battle_for_caen.png";
import image2 from "@/assets/images/worldwar_window_bg_chinese_farm.png";
import image3 from "@/assets/images/worldwar_window_bg_el_alamein.png";
import { computed } from "vue";
import { useI18n } from "vue-i18n";

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
        <span class="text-h5 border-b-lg border-primary">
          {{ t("home.common_function") }}
        </span>
      </v-col>
      <v-col cols="4" v-for="item in commonFunctions" :key="item.key">
        <FunctionCard :data="item"></FunctionCard>
      </v-col>
      <v-col cols="12">
        <span class="text-h5 border-b-lg border-primary">
          {{ t("home.quick_link") }}
        </span>
      </v-col>
      <v-col cols="4" v-for="item in usefulLinks" :key="item.key">
        <LinkCard :data="item"></LinkCard>
      </v-col>
    </v-row>
    <v-footer app color="surface-light" class="justify-center">
      <strong>{{ t("home.copyright") }}</strong>
    </v-footer>
  </v-container>
</template>

<style scoped></style>
