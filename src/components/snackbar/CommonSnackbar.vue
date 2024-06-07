<script setup lang="ts">
const snackBar = defineModel<{
    show: boolean,
    message: string,
    color: string
}>({
    required: true
})
</script>


<template>
    <v-snackbar vertical v-model="snackBar.show" :color="snackBar.color">
        {{ snackBar.message }}
    </v-snackbar>
</template>

<style scoped></style>