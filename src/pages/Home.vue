<script setup lang="ts">
import { useRouter } from 'vue-router';
const router = useRouter()




const appMenu = [
    {
        key: "wt-skins",
        title: "自定义涂装管理",
        subtitle: "管理《战争雷霆》的自定义涂装",
        description: "战雷的自定义涂装安装/卸载太麻烦？不知道游戏的目录在哪里？想快捷管理自定义涂装？试试这个工具吧",
    },
    {
        key: "wt-sight",
        title: "自定义瞄具管理",
        subtitle: "管理《战争雷霆》的自定义瞄具",
        description: "战雷的自定义瞄具安装/卸载太麻烦？试试这个工具吧",
    },
    {
        key: "setting",
        title: "设置",
        subtitle: "一些功能需要进行设置才能正常使用",
        description: "小工具的正确运行需要一些设置，比如设置战雷的安装目录，设置目录等",
    },
    {
        key: "about",
        title: "关于APP",
        subtitle: "关于这个APP的一些信息",
        description: "由 B站UP主 摸鱼又开摆的三三 开发的一个小工具",
    }
]

import image1 from '@/assets/images/worldwar_window_bg_battle_for_caen.png'
import image2 from '@/assets/images/worldwar_window_bg_chinese_farm.png'
import image3 from '@/assets/images/worldwar_window_bg_el_alamein.png'

const banner = [
    {
        src: image1,
        cover: true
    },
    {
        src: image2,
        cover: true
    },
    {
        src: image3,
        cover: true
    }
]

function navigateTo(to: string) {
    router.push({ name: to })
}

</script>


<template>
    <v-carousel hide-delimiters height="180" :cycle="true">
        <v-carousel-item v-for="(item, i) in banner" :key="i" :src="item.src" :cover="item.cover"
            @contextmenu.prevent></v-carousel-item>
    </v-carousel>
    <v-container>
        <v-row>
            <v-col cols="6" v-for="item in appMenu" :key="item.key">
                <v-card class="" color="primary" hover @click="navigateTo(item.key)" height="130">
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


<style scoped></style>