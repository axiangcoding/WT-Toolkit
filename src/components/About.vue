<script setup lang="ts">
const breadcrumbsItems = [
    {
        title: '主页',
        disabled: false,
        href: '/',
    },
    {
        title: "APP 信息",
        disabled: true,
    },
    {
        title: '关于',
        disabled: true,
        href: '/about',
    },
]
</script>

<template>
    <v-breadcrumbs :items="breadcrumbsItems"></v-breadcrumbs>
    
</template>

<style scoped></style>