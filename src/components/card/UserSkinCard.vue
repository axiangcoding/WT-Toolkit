<script setup lang="ts">
import image1 from '@/assets/images/china_ground_newmodificationresearch.png'
import { invoke } from '@tauri-apps/api';

const props = defineProps<{
    skinMetadata: any
}>()

async function showSkin(skin_folder_path: string) {
    await invoke('show_in_folder', { path: skin_folder_path })
}


</script>


<template>
    <v-card>
        <v-img class="align-end text-white" height="200" :src="image1" cover>
            <v-card-title>{{ props.skinMetadata.name }}</v-card-title>
        </v-img>

        <v-card-text>
            <!-- <div>涂装介绍：[无]</div> -->
            <div>载具 {{ props.skinMetadata.vehicle_id }}</div>
            <div>空间占用 {{ (props.skinMetadata.size_bytes / 1024 / 1024).toFixed(2) }}MB</div>
        </v-card-text>

        <v-card-actions>
            <v-btn color="primary" text="查看" @click="showSkin(props.skinMetadata.path)"></v-btn>
            <!-- <v-btn color="warning" text="备份"></v-btn> -->
            <v-btn color="error" text="删除" @click="$emit('delete-skin', props.skinMetadata)"></v-btn>

        </v-card-actions>
    </v-card>
</template>


<style scoped></style>