<script setup lang="ts">

import { open } from '@tauri-apps/api/dialog';
import { onMounted, ref } from 'vue'
import { appConfigDir, appLogDir } from '@tauri-apps/api/path';
import { invoke } from '@tauri-apps/api';
import { AppSettings, getAppSettings, getDefaultSettings, saveAppSettings } from '../settings';




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


const snackbar = ref({
    show: false,
    message: '',
    color: 'success'
})

const appSettings = ref<AppSettings>({} as AppSettings)

onMounted(async () => {
    appSettings.value = await getAppSettings()
})

async function resetSettings() {
    appSettings.value = await getDefaultSettings()
    snackbar.value = {
        show: true,
        message: '重置设置成功',
        color: 'success'
    }
}

async function saveSettings() {
    try {
        await saveAppSettings(appSettings.value)
        snackbar.value = {
            show: true,
            message: '保存设置成功',
            color: 'success'
        }
    } catch (error) {
        snackbar.value = {
            show: true,
            message: `保存设置失败: ${error}`,
            color: 'error'
        }
    }
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
            snackbar.value = {
                show: true,
                message: '未检测到战争雷霆游戏安装目录',
                color: 'error'
            }
            return
        }
        appSettings.value.wt_install_path = response as string

        snackbar.value = {
            show: true,
            message: '自动检测到战争雷霆游戏安装目录',
            color: 'success'
        }
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
            snackbar.value = {
                show: true,
                message: '未检测到战争雷霆游戏设置目录',
                color: 'error'
            }
            return
        }
        appSettings.value.wt_setting_path = response as string
        snackbar.value = {
            show: true,
            message: '自动检测到战争雷霆游戏设置目录',
            color: 'success'
        }
    })
}

function selectSkinBackupPath() {
    selectPath(appSettings.value.wt_skins_backup_path).then((path) => {
        if (typeof path === 'string') {
            appSettings.value.wt_skins_backup_path = path
        } else if (Array.isArray(path)) {
            appSettings.value.wt_skins_backup_path = path[0]
        }
    })
}


async function openSettingFolder() {
    let path = await appConfigDir()
    await invoke('show_in_folder', { path: path })
}

async function openLogFolder() {
    let path = await appLogDir()
    await invoke('show_in_folder', { path: path })
}

async function showFolder(path: string) {
    await invoke('show_in_folder', { path: path })
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
                                        <v-tooltip activator="parent" location="bottom">小工具将自动检测游戏安装路径</v-tooltip>
                                    </v-btn>
                                </v-col>
                                <v-col cols="auto">
                                    <v-btn color="primary" @click="selectWTInstallPath">手动选择</v-btn>
                                </v-col>
                                <v-col cols="auto">
                                    <v-btn color="info" @click="showFolder(appSettings.wt_install_path)">查看目录</v-btn>
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
                                        <v-tooltip activator="parent" location="bottom">小工具将自动检测游戏设置路径</v-tooltip>
                                    </v-btn>
                                </v-col>
                                <v-col cols="auto">
                                    <v-btn color="primary" @click="selectWTSettingPath">手动选择</v-btn>
                                </v-col>
                                <v-col cols="auto">
                                    <v-btn color="info" @click="showFolder(appSettings.wt_setting_path)">查看目录</v-btn>
                                </v-col>
                            </v-row>
                        </v-container>
                    </template>
                </v-text-field>
            </v-col>
            <v-col cols="12">
                <v-text-field v-model="appSettings.wt_skins_backup_path" label="战争雷霆自定义涂装备份目录"
                    placeholder="请选择战争雷霆自定义涂装的备份目录" type="text" variant="outlined" clearable readonly>
                    <template v-slot:append>
                        <v-container>
                            <v-row>
                                <v-col cols="auto">
                                    <v-btn color="primary" @click="selectSkinBackupPath">选择目录</v-btn>
                                </v-col>
                                <v-col cols="auto">
                                    <v-btn color="info"
                                        @click="showFolder(appSettings.wt_skins_backup_path)">查看目录</v-btn>
                                </v-col>
                            </v-row>
                        </v-container>
                    </template>
                </v-text-field>
            </v-col>
            <v-col cols="12">
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
                        <v-col cols="auto">
                            <v-btn color="info" @click="openLogFolder">
                                打开日志文件夹</v-btn>
                        </v-col>
                    </v-row>
                </v-container>
            </v-col>
        </v-row>
    </v-container>
    <v-snackbar vertical v-model="snackbar.show" :color="snackbar.color">
        {{ snackbar.message }}
    </v-snackbar>
</template>

<style scoped></style>