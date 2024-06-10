<script setup lang="ts">
const breadcrumbsItems = [
    {
        title: '主页',
        disabled: false,
        href: '/',
    },
    {
        title: "战雷小工具",
        disabled: true,
    },
    {
        title: '自定义瞄具管理',
        disabled: true,
        href: '/wt-sight',
    },
]

</script>



<template>
    <v-breadcrumbs :items="breadcrumbsItems"></v-breadcrumbs>
</template>



<style scoped></style>