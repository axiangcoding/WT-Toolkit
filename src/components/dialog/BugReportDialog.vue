<script setup lang="ts">
import qqGroupQrCode from "@/assets/images/qrcode.jpg";
import { useI18n } from "vue-i18n";

const show = defineModel<boolean>();

const { t } = useI18n();
</script>

<template>
  <v-dialog v-model="show" width="auto">
    <v-card max-width="450" prepend-icon="mdi-bug">
      <template v-slot:title> {{ t("bug_report.title") }} </template>
      <v-card-text>
        {{ t("bug_report.tip1") }}
        <v-img :src="qqGroupQrCode" width="auto" />
      </v-card-text>
      <v-card-actions
        ><v-btn
          color=""
          :text="t('bug_report.confirm')"
          @click="show = false"
        ></v-btn
      ></v-card-actions>
    </v-card>
  </v-dialog>
</template>

<style scoped></style>
