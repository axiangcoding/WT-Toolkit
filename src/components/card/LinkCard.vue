<script setup lang="ts">
import { open } from "@tauri-apps/api/shell";

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
    height="140"
  >
    <template v-slot:title>
      {{ props.data.title }}
      <v-chip
        v-show="props.data.isOfficial"
        variant="elevated"
        color="primary"
        density="comfortable"
      >
        官方
      </v-chip>
      <v-chip
        v-show="!props.data.isOfficial"
        variant="elevated"
        color="info"
        density="comfortable"
      >
        社区
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
