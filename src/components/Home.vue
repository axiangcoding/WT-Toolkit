<script setup lang="ts">
import { useRouter } from 'vue-router';
const router = useRouter()




const appMenu = [
    {
        key: "wt-paint",
        title: "一键安装涂装",
        subtitle: "一键安装《战争雷霆》的涂装文件",
        description: "战雷的涂装安装/卸载太麻烦？不知道游戏的目录在哪里？试试我们的工具，一键安装/卸载涂装",
    },
    {
        key: "TBD1",
        title: "占位符1",
        subtitle: "占位符1",
        description: "占位符1",
    },
    {
        key: "TBD2",
        title: "占位符2",
        subtitle: "占位符2",
        description: "占位符2",
    },
    {
        key: "TBD3",
        title: "占位符3",
        subtitle: "占位符3",
        description: "占位符3",
    },
    {
        key: "TBD4",
        title: "占位符4",
        subtitle: "占位符4",
        description: "占位符4",
    },
    {
        key: "TBD5",
        title: "占位符5",
        subtitle: "占位符5",
        description: "占位符5",
    },
    {
        key: "TBD6",
        title: "占位符6",
        subtitle: "占位符6",
        description: "占位符6",
    },
]

import url1 from '@/assets/images/worldwar_window_bg_battle_for_caen.png'
import url2 from '@/assets/images/worldwar_window_bg_chinese_farm.png'
import url3 from '@/assets/images/worldwar_window_bg_el_alamein.png'

const banner = [
    {
        src: url1,
        cover: true
    },
    {
        src: url2,
        cover: true
    },
    {
        src: url3,
        cover: true
    }
]

function navigateTo(to: string) {
    router.push({ name: to })
}

</script>


<template>
    <v-carousel hide-delimiters height="180" :cycle="true">
        <v-carousel-item v-for="(item, i) in banner" :key="i" :src="item.src" :cover="item.cover"></v-carousel-item>
    </v-carousel>
    <!-- <div class="text-h4">来试试我们的小工具吧</div> -->
    <v-container>
        <v-row>
            <v-col cols="6" v-for="item in appMenu" :key="item.key">
                <v-card class="" color="primary" hover @click="navigateTo(item.key)">
                    <v-card-item>
                        <v-card-title>
                            {{ item.title }}
                        </v-card-title>

                        <v-card-subtitle>
                            {{ item.subtitle }}
                        </v-card-subtitle>
                    </v-card-item>

                    <v-card-text>
                        {{ item.description }}
                    </v-card-text>
                </v-card>
            </v-col>
        </v-row>
    </v-container>
</template>


<style scoped>


</style>