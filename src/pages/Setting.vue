<script setup lang="ts">
import { open } from "@tauri-apps/api/dialog";
import { onMounted, ref } from "vue";
import { invoke, shell } from "@tauri-apps/api";

import CommonSnackbar from "../components/snackbar/CommonSnackbar.vue";
import { get_error_msg } from "../error_msg";

const breadcrumbsItems = [
  {
    title: "主页",
    disabled: false,
    href: "/",
  },
  {
    title: "APP 信息",
    disabled: true,
  },
  {
    title: "设置",
    disabled: true,
    href: "/setting",
  },
];

const snackbar = ref({
  show: false,
  message: "",
  color: "success",
});

const appSettings = ref<{
  wt_root_path: string;
  wt_setting_path: string;
  wt_ext_cli_path: string;
}>({} as any);

onMounted(async () => {
  try {
    appSettings.value = await invoke("get_app_config");
  } catch (error) {
    snackbar.value = {
      show: true,
      message: get_error_msg(error),
      color: "error",
    };
  }
});

async function saveSettings() {
  try {
    await invoke("save_app_config", { config: appSettings.value });
    snackbar.value = {
      show: true,
      message: "保存设置成功",
      color: "success",
    };
  } catch (error) {
    snackbar.value = {
      show: true,
      message: get_error_msg(error),
      color: "error",
    };
  }
}

function selectWTInstallPath() {
  selectPath(appSettings.value.wt_root_path).then((path) => {
    if (typeof path === "string") {
      appSettings.value.wt_root_path = path;
    } else if (Array.isArray(path)) {
      appSettings.value.wt_root_path = path[0];
    }
  });
}

async function autoSelectWTInstallPath() {
  try {
    appSettings.value.wt_root_path = await invoke("auto_detected_wt_root_path");
    snackbar.value = {
      show: true,
      message: "自动检测到战争雷霆游戏安装目录！",
      color: "success",
    };
  } catch (error) {
    snackbar.value = {
      show: true,
      message: get_error_msg(error),
      color: "error",
    };
  }
}

function selectWTSettingPath() {
  selectPath(appSettings.value.wt_setting_path).then((path) => {
    if (typeof path === "string") {
      appSettings.value.wt_setting_path = path;
    } else if (Array.isArray(path)) {
      appSettings.value.wt_setting_path = path[0];
    }
  });
}

async function autoSelectWTSettingPath() {
  try {
    appSettings.value.wt_setting_path = await invoke(
      "auto_detected_wt_setting_path",
    );
    snackbar.value = {
      show: true,
      message: "自动检测到战争雷霆游戏设置目录",
      color: "success",
    };
  } catch (error) {
    snackbar.value = {
      show: true,
      message: get_error_msg(error),
      color: "error",
    };
  }
}

async function openSettingFolder() {
  let path = await invoke("get_app_config_dir");
  await invoke("show_in_folder", { path: path });
}

async function openLogFolder() {
  let path = await invoke("get_app_log_dir");
  await invoke("show_in_folder", { path: path });
}

async function showFolder(path: string) {
  await invoke("show_in_folder", { path: path });
}

async function selectPath(defaultPath: string) {
  let selectedPath = await open({
    directory: true,
    multiple: false,
    defaultPath: defaultPath,
  });
  return selectedPath;
}

async function openWTExtCliDownloadPage() {
  await shell.open(
    "https://github.com/Warthunder-Open-Source-Foundation/wt_ext_cli/releases/latest",
  );
}

async function selectWTExtCliPath() {
  selectPath(appSettings.value.wt_ext_cli_path).then((path) => {
    if (typeof path === "string") {
      appSettings.value.wt_ext_cli_path = path;
    } else if (Array.isArray(path)) {
      appSettings.value.wt_ext_cli_path = path[0];
    }
  });
}
</script>

<template>
  <v-breadcrumbs :items="breadcrumbsItems"></v-breadcrumbs>

  <v-container>
    <v-row>
      <v-col cols="12">
        <v-text-field
          v-model="appSettings.wt_root_path"
          label="战争雷霆游戏安装目录"
          placeholder="请选择战争雷霆游戏的安装目录，用来管理和游戏相关的资源"
          type="text"
          variant="outlined"
          clearable
          readonly
        >
          <template v-slot:append>
            <v-container>
              <v-row>
                <v-col cols="auto">
                  <v-btn color="warning" @click="autoSelectWTInstallPath">
                    自动检测
                    <v-tooltip activator="parent" location="bottom">
                      工具箱将自动检测游戏安装路径
                    </v-tooltip>
                  </v-btn>
                </v-col>
                <v-col cols="auto">
                  <v-btn color="primary" @click="selectWTInstallPath"
                    >手动选择</v-btn
                  >
                </v-col>
                <v-col cols="auto">
                  <v-btn
                    color="info"
                    @click="showFolder(appSettings.wt_root_path)"
                    >查看目录</v-btn
                  >
                </v-col>
              </v-row>
            </v-container>
          </template>
        </v-text-field>
      </v-col>
      <v-col cols="12">
        <v-text-field
          v-model="appSettings.wt_setting_path"
          label="战争雷霆游戏设置目录"
          placeholder="请选择战争雷霆游戏的设置目录，用来管理和游戏设置相关的资源"
          type="text"
          variant="outlined"
          clearable
          readonly
        >
          <template v-slot:append>
            <v-container>
              <v-row>
                <v-col cols="auto">
                  <v-btn color="warning" @click="autoSelectWTSettingPath">
                    自动检测
                    <v-tooltip activator="parent" location="bottom">
                      工具箱将自动检测游戏设置路径
                    </v-tooltip>
                  </v-btn>
                </v-col>
                <v-col cols="auto">
                  <v-btn color="primary" @click="selectWTSettingPath"
                    >手动选择</v-btn
                  >
                </v-col>
                <v-col cols="auto">
                  <v-btn
                    color="info"
                    @click="showFolder(appSettings.wt_setting_path)"
                    >查看目录</v-btn
                  >
                </v-col>
              </v-row>
            </v-container>
          </template>
        </v-text-field>
      </v-col>
      <v-col cols="12">
        <v-text-field
          v-model="appSettings.wt_ext_cli_path"
          label="wt_ext_cli解包工具目录"
          placeholder="如果你需要使用wt_ext_cli解包工具，请选择解包工具的目录"
          type="text"
          variant="outlined"
          clearable
          readonly
        >
          <template v-slot:append>
            <v-container>
              <v-row>
                <v-col cols="auto">
                  <v-btn color="warning" @click="openWTExtCliDownloadPage">
                    从官方仓库下载
                    <v-tooltip activator="parent" location="bottom">
                      从wt_ext_cli仓库下载工具并正确安装到你的操作系统中</v-tooltip
                    >
                  </v-btn>
                </v-col>
                <v-col cols="auto">
                  <v-btn color="primary" @click="selectWTExtCliPath"
                    >选择目录</v-btn
                  >
                </v-col>
                <v-col cols="auto">
                  <v-btn
                    color="info"
                    @click="showFolder(appSettings.wt_ext_cli_path)"
                    >查看目录</v-btn
                  >
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
              <v-btn color="info" @click="openSettingFolder"
                >打开配置文件夹</v-btn
              >
            </v-col>
            <v-col cols="auto">
              <v-btn color="info" @click="openLogFolder"> 打开日志文件夹</v-btn>
            </v-col>
          </v-row>
        </v-container>
      </v-col>
    </v-row>
  </v-container>
  <CommonSnackbar v-model="snackbar" />
</template>

<style scoped></style>
