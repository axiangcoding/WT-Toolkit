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
      title: '自定义涂装管理',
      disabled: true,
      href: '/wt-paint',
    },
  ]
</script>


<template>
<v-breadcrumbs :items="breadcrumbsItems"></v-breadcrumbs>
    
</template>

<style scoped></style>