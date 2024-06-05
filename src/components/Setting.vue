<script setup lang="ts">

import { open } from '@tauri-apps/api/dialog';
import { onMounted, ref } from 'vue'
import { writeTextFile, BaseDirectory, createDir, exists, removeFile, readTextFile } from '@tauri-apps/api/fs';
import { appConfigDir } from '@tauri-apps/api/path';
import { invoke } from '@tauri-apps/api';


onMounted(() => {
    readSettings()
})

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
        title: '设置',
        disabled: true,
        href: '/setting',
    },
]

const snackbar = ref(false)
const snackbarMessage = ref('')
const snackbarColor = ref('success')

const appSettings = ref({
    wt_install_path: '',
    wt_setting_path: '',
})

const defaultSettings = {
    wt_install_path: '',
    wt_setting_path: '',
}

function resetSettings() {
    appSettings.value = defaultSettings
    snackbarMessage.value = '重置配置项为默认值'
    snackbar.value = true
}

async function readSettings() {
    let fileName = 'settings.json'
    let setting_str = ''
    if (await exists(fileName, { dir: BaseDirectory.AppConfig })) {
        setting_str = await readTextFile(fileName, { dir: BaseDirectory.AppConfig })
    }
    if (setting_str === '') {
        appSettings.value = defaultSettings
    } else {
        appSettings.value = JSON.parse(setting_str)
    }
}

async function saveSettings() {
    let fileName = 'settings.json'
    snackbarMessage.value = '保存配置成功'
    snackbar.value = true
    let setting_str = JSON.stringify(appSettings.value, null, 2)
    await createDir("", { dir: BaseDirectory.AppConfig, recursive: true })
    if (await exists(fileName, { dir: BaseDirectory.AppConfig })) {
        await removeFile(fileName, { dir: BaseDirectory.AppConfig })
    }
    await writeTextFile(fileName, setting_str, { dir: BaseDirectory.AppConfig });
}

function selectWTInstallPath() {
    selectPath(appSettings.value.wt_install_path).then((path) => {
        if (typeof path === 'string') {
            appSettings.value.wt_install_path = path
        } else if (Array.isArray(path)) {
            appSettings.value.wt_install_path = path[0]
        }
    })
}

function autoSelectWTInstallPath() {
    invoke('auto_detected_wt_install_path').then((response) => {
        if (response === null) {
            snackbarMessage.value = '未检测到战争雷霆游戏安装目录'
            snackbarColor.value = 'error'
            snackbar.value = true
            return
        }
        appSettings.value.wt_install_path = response as string
        snackbarMessage.value = '自动检测到战争雷霆游戏安装目录'
        snackbar.value = true
    })
}

function selectWTSettingPath() {
    selectPath(appSettings.value.wt_setting_path).then((path) => {
        if (typeof path === 'string') {
            appSettings.value.wt_setting_path = path
        } else if (Array.isArray(path)) {
            appSettings.value.wt_setting_path = path[0]
        }
    })
}

function autoSelectWTSettingPath() {
    invoke('auto_detected_wt_setting_path').then((response) => {
        if (response === null) {
            snackbarMessage.value = '未检测到战争雷霆游戏设置目录'
            snackbarColor.value = 'error'
            snackbar.value = true
            return
        }
        appSettings.value.wt_setting_path = response as string
        snackbarMessage.value = '自动检测到战争雷霆游戏设置目录'
        snackbar.value = true
    })
}


async function openSettingFolder() {
    let appConfigDirPath = await appConfigDir()
    console.log(appConfigDirPath)
    await invoke('show_in_folder', { path: appConfigDirPath })
}

async function selectPath(defaultPath: string) {
    let selectedPath = await open({
        directory: true,
        multiple: false,
        defaultPath: defaultPath
    })
    return selectedPath
}

</script>

<template>
    <v-breadcrumbs :items="breadcrumbsItems"></v-breadcrumbs>

    <v-form>
        <v-container>
            <v-row>
                <v-col cols="12">
                    <v-text-field v-model="appSettings.wt_install_path" label="战争雷霆游戏安装目录"
                        placeholder="请选择战争雷霆游戏的安装目录，用来管理和游戏相关的资源" type="text" variant="outlined" clearable readonly>
                        <template v-slot:append>
                            <v-container>
                                <v-row>
                                    <v-col cols="auto">
                                        <v-btn color="warning" @click="autoSelectWTInstallPath">
                                            自动检测
                                            <v-tooltip activator="parent" location="bottom">程序将自动检测游戏安装路径</v-tooltip>
                                        </v-btn>
                                    </v-col>
                                    <v-col cols="auto">
                                        <v-btn color="info" @click="selectWTInstallPath">手动选择</v-btn>
                                    </v-col>

                                </v-row>
                            </v-container>
                        </template>
                    </v-text-field>
                </v-col>
                <v-col cols="12">
                    <v-text-field v-model="appSettings.wt_setting_path" label="战争雷霆游戏设置目录"
                        placeholder="请选择战争雷霆游戏的设置目录，用来管理和游戏设置相关的资源" type="text" variant="outlined" clearable readonly>
                        <template v-slot:append>
                            <v-container>
                                <v-row>
                                    <v-col cols="auto">
                                        <v-btn color="warning" @click="autoSelectWTSettingPath">
                                            自动检测
                                            <v-tooltip activator="parent" location="bottom">程序将自动检测游戏设置路径</v-tooltip>
                                        </v-btn>
                                    </v-col>
                                    <v-col cols="auto">
                                        <v-btn color="info" @click="selectWTSettingPath">手动选择</v-btn>
                                    </v-col>

                                </v-row>
                            </v-container>
                        </template>
                    </v-text-field>
                </v-col>
                <v-col cols="6" offset="6">
                    <v-container>
                        <v-row justify="end">
                            <v-col cols="auto">
                                <v-btn color="success" @click="saveSettings">保存设置</v-btn>
                            </v-col>
                            <v-col cols="auto">
                                <v-btn color="error" @click="resetSettings">重置为默认</v-btn>
                            </v-col>
                            <v-col cols="auto">
                                <v-btn color="info" @click="openSettingFolder">打开配置文件夹</v-btn>
                            </v-col>
                            <v-snackbar vertical v-model="snackbar" :color="snackbarColor">
                                {{ snackbarMessage }}
                            </v-snackbar>
                        </v-row>
                    </v-container>
                </v-col>
            </v-row>
        </v-container>
    </v-form>

</template>

<style scoped></style>