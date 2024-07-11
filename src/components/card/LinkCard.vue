<script setup lang="ts">
import { open } from "@tauri-apps/api/shell";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const props = defineProps<{
  data: any;
}>();

async function navigateTo(url: string) {
  open(url);
}
</script>

<template>
  <v-card
    class=""
    color="secondary"
    hover
    prepend-icon="mdi-link-variant"
    @click="navigateTo(props.data.url)"
    height="160"
  >
    <template v-slot:title>
      {{ props.data.title }}
      <v-chip
        class="mx-1"
        v-show="props.data.isOfficial"
        variant="elevated"
        color="primary"
        density="comfortable"
      >
        {{ t("home.link_card.tag.official") }}
      </v-chip>
      <v-chip
        class="mx-1"
        v-show="!props.data.isOfficial"
        variant="elevated"
        color="info"
        density="comfortable"
      >
        {{ t("home.link_card.tag.community") }}
      </v-chip>
      <v-chip
        class="mx-1"
        v-show="props.data.isLocal"
        variant="elevated"
        color="success"
        density="comfortable"
      >
        {{ t("home.link_card.tag.local") }}
      </v-chip>
    </template>
    <v-card-subtitle>
      {{ props.data.url }}
    </v-card-subtitle>
    <v-card-text>
      {{ props.data.description }}
    </v-card-text>
  </v-card>
</template>

<style scoped></style>
