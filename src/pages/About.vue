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
        title: 'v0.0.1',
        latest: false,
        subtitle: '三三开发的“三三的战雷小工具”第一个版本！梦开始的地方！',
        description: [
            '这个版本实现了针对战争雷霆自定义涂装管理的基本功能，方便使用者管理自己安装在游戏中的自定义涂装文件。',
            '同时还实现了一些工具的基本功能，包括设置界面，关于界面等，确保用户能有良好的体验。',
        ],
    },
    {
        title: 'v0.0.2',
        latest: false,
        subtitle: '这个版本主要完善了用户体验，调整了界面布局等',
        description: [
            '修复了无法设置配置项的问题',
            '优化了自定义涂装管理界面的体验',
        ],
    },
    {
        title: 'v0.0.3',
        latest: false,
        subtitle: '这个版本主要强化自定义涂装管理功能，并增加了一些额外的新功能',
        description: [
            '自定义涂装管理支持嵌套目录',
        ],
    },
    {
        title: 'v0.0.4',
        latest: true,
        subtitle: 'TBD',
        description: [
            'TBD',
            'TBD',
        ],
    }
]
</script>

<template>
    <v-breadcrumbs :items="breadcrumbsItems"></v-breadcrumbs>
    <v-container>
        <v-row>
            <v-col cols="6">
                <v-timeline side="end">
                    <v-timeline-item v-for="item in items" :key="item.title" size="small"
                        :dot-color="item.latest ? 'success' : 'primary'">
                        <v-card :color="item.latest ? 'success' : 'primary'"
                            :prepend-icon="item.latest ? 'mdi-star-check' : 'mdi-star'" :value="true" variant="outlined"
                            width="480">
                            <template v-slot:title>
                                {{ item.title }} {{ item.latest ? '- 最新版本' : '' }}
                            </template>
                            <v-card-subtitle>{{ item.subtitle }}</v-card-subtitle>
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