<script setup lang="ts">
const show = defineModel<boolean>()
const props = defineProps<{
    skinPath: string
}>()
const emit = defineEmits(['confirm'])


function confirm() {
    show.value = false
    emit('confirm')
}
</script>



<template>
    <v-dialog v-model="show" width="auto">
        <v-card prepend-icon="mdi-alert">
            <template v-slot:title>
                你确定要加载自定义涂装到游戏文件夹中吗？
            </template>
            <v-card-subtitle>
                选中的路径：{{ props.skinPath }}
            </v-card-subtitle>
            <v-card-text>
                请务必安装来自可信任来源的自定义涂装文件，否则可能会导致游戏无法正常运行，甚至更严重的问题。
            </v-card-text>
            <v-card-actions>
                <v-btn color="error" text="取消" @click="show = false"></v-btn>
                <v-btn color="success" text="同意并安装" @click="confirm"></v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>


<style scoped></style>