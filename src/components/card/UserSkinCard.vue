<script setup lang="ts">
import image1 from "@/assets/images/china_ground_newmodificationresearch.png";
import { invoke } from "@tauri-apps/api";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const props = defineProps<{
  skinMetadata: any;
}>();

async function showSkin(skin_folder_path: string) {
  await invoke("show_folder", { path: skin_folder_path });
}
</script>

<template>
  <v-card>
    <v-img class="align-end text-white" height="200" :src="image1" cover>
      <v-card-title class="card-title">
        {{ props.skinMetadata.skin_name }}
      </v-card-title>
    </v-img>

    <v-card-text>
      <div>
        <strong>
          {{ t("wt_skins.card.vehicle_id") }}
        </strong>
        {{ props.skinMetadata.vehicle_id }}
      </div>
      <div>
        <strong> {{ t("wt_skins.card.space_occupancy") }}</strong>
        {{ (props.skinMetadata.folder_size / 1024 / 1024).toFixed(2) }}MB
      </div>
    </v-card-text>

    <v-card-actions>
      <v-btn
        color="primary"
        :text="t('wt_skins.card.show_folder')"
        @click="showSkin(props.skinMetadata.full_path)"
      ></v-btn>
      <v-btn
        color="error"
        :text="t('wt_skins.card.delete')"
        @click="$emit('delete-skin', props.skinMetadata)"
      ></v-btn>
    </v-card-actions>
  </v-card>
</template>

<style scoped>
.card-title {
  background-color: rgba(0, 0, 0, 0.5);
}
</style>
