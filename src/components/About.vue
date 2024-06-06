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


const items = [
    {
        color: 'success',
        icon: 'mdi-star-check',
        title: 'v0.0.1',
        description: [
            '摸鱼又开摆的三三开发的“三三的战雷小工具”第一个版本！梦开始的地方！',
            '这个版本实现了针对战争雷霆自定义涂装管理的基本功能，方便使用者管理自己安装在游戏中的自定义涂装文件。',
            '同时还实现了一些工具的基本功能，包括设置界面，关于界面等，确保用户能有良好的体验。',
        ],
    },
]
</script>

<template>
    <v-breadcrumbs :items="breadcrumbsItems"></v-breadcrumbs>
    <v-container>
        <v-row>
            <v-col>
                <v-timeline side="end">
                    <v-timeline-item v-for="item in items" :key="item.title" size="small" :dot-color="item.color">
                        <v-card :color="item.color" :prepend-icon="item.icon" :value="true" variant="outlined">
                            <template v-slot:title>
                                {{ item.title }} {{ item.color === 'success' ? '- 最新版本' : '' }}
                            </template>
                            <v-card-text>
                                <div v-for="desc in item.description" :key="desc">{{ desc }}</div>
                            </v-card-text>

                        </v-card>
                    </v-timeline-item>
                </v-timeline>
            </v-col>
        </v-row>
    </v-container>
</template>

<style scoped></style>