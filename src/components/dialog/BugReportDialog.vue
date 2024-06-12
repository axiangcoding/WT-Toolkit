<script setup lang="ts">
import qqGroupQrCode from "@/assets/images/qrcode.jpg";

const show = defineModel<boolean>();
</script>

<template>
  <v-dialog v-model="show" width="auto">
    <v-card max-width="450" prepend-icon="mdi-bug">
      <template v-slot:title> 报告 BUG </template>
      <v-card-text>
        加入开发者QQ群反馈问题
        <v-img :src="qqGroupQrCode" width="auto" />
      </v-card-text>
      <v-card-actions
        ><v-btn color="success" text="好的" @click="show = false"></v-btn
      ></v-card-actions>
    </v-card>
  </v-dialog>
</template>

<style scoped></style>
