<script setup lang="ts">
import { useRouter } from "vue-router";

const router = useRouter();

const props = defineProps<{
  data: any;
}>();

function navigateTo(to: string) {
  router.push({ name: to });
}
</script>

<template>
  <v-card
    class=""
    color="primary"
    hover
    :prepend-icon="props.data.icon"
    @click="navigateTo(props.data.key)"
    height="140"
  >
    <template v-slot:title>
      {{ props.data.title }}
    </template>

    <v-card-subtitle>
      {{ props.data.subtitle }}
    </v-card-subtitle>
    <v-card-text >
      {{ props.data.description }}
    </v-card-text>
  </v-card>
</template>

<style scoped></style>
