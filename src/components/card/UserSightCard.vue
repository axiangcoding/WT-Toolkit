<script setup lang="ts">
import image1 from "@/assets/images/china_heli_newmodificationresearch.png";
import { invoke } from "@tauri-apps/api";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const props = defineProps<{
  sightMetadata: any;
}>();

async function showSight(folder_path: string) {
  await invoke("show_folder", { path: folder_path });
}
</script>

<template>
  <v-card>
    <v-img class="align-end text-white" height="200" :src="image1" cover>
      <v-card-title class="card-title">
        {{ props.sightMetadata.folder_name }}
      </v-card-title>
    </v-img>

    <v-card-text>
      <div>
        <strong>{{ t("wt_sight.card.vehicle_id") }}</strong>
        {{ props.sightMetadata.vehicle_id }}
      </div>
      <div class="mt-1">
        <strong>{{ t("wt_sight.card.space_occupancy") }}</strong>
        {{ (props.sightMetadata.folder_size / 1024).toFixed(2) }}KB
      </div>

      <strong>{{ t("wt_sight.card.included_sights") }}</strong>
      <v-chip
        class="mx-1"
        variant="elevated"
        color="primary"
        v-for="item in props.sightMetadata.sight_names"
        :key="item"
        >{{ item }}</v-chip
      >

      <!-- {{ props.sightMetadata }} -->
    </v-card-text>

    <v-card-actions>
      <v-btn
        color="primary"
        :text="t('wt_sight.card.show_folder')"
        @click="showSight(props.sightMetadata.full_path)"
      ></v-btn>
      <v-btn
        color="error"
        :text="t('wt_sight.card.delete')"
        @click="$emit('delete-sight', props.sightMetadata)"
      ></v-btn>
    </v-card-actions>
  </v-card>
</template>

<style scoped>
.card-title {
  background-color: rgba(0, 0, 0, 0.5);
}
</style>
