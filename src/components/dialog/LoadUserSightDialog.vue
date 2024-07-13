<script setup lang="ts">
import { useI18n } from "vue-i18n";

const show = defineModel<boolean>();
const props = defineProps<{
  sightPath: string;
}>();

const { t } = useI18n();

const emit = defineEmits(["confirm"]);

function confirm() {
  show.value = false;
  emit("confirm");
}
</script>

<template>
  <v-dialog v-model="show" width="auto">
    <v-card prepend-icon="mdi-alert">
      <template v-slot:title>
        {{ t("wt_sight.load_dialog.title") }}
      </template>
      <v-card-subtitle>
        {{ t("wt_sight.load_dialog.selected_path") }} {{ props.sightPath }}
      </v-card-subtitle>
      <v-card-text>
        {{ t("wt_sight.load_dialog.tip1") }}
      </v-card-text>
      <v-card-actions>
        <v-btn
          color="error"
          :text="t('wt_sight.load_dialog.cancel')"
          @click="show = false"
        ></v-btn>
        <v-btn
          color="success"
          :text="t('wt_sight.load_dialog.confirm')"
          @click="confirm"
        ></v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<style scoped></style>
