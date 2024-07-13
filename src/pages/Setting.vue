<script setup lang="ts">
import { open } from "@tauri-apps/api/dialog";
import { computed, onMounted, ref } from "vue";
import { invoke, shell } from "@tauri-apps/api";

import CommonSnackbar from "../components/snackbar/CommonSnackbar.vue";
import { get_error_msg } from "../error_msg";
import { AppSettings } from "../schema";

import { useI18n } from "vue-i18n";

const { t } = useI18n();

const breadcrumbsItems = computed(() => [
  {
    title: t("app.nav_drawer.home"),
    disabled: false,
    href: "/",
  },
  {
    title: t("app.nav_drawer.sub_header.app_info"),
    disabled: true,
  },
  {
    title: t("app.nav_drawer.settings"),
    disabled: true,
    href: "/setting",
  },
]);

const snackbar = ref({
  show: false,
  message: "",
  color: "success",
});

const appSettings = ref<AppSettings>({} as any);

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
      message: t("setting.save_setting_success"),
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
      message: t("setting.auto_detected_wt_root_path_success"),
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
      message: t("setting.auto_detected_wt_setting_path_success"),
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
  await invoke("show_folder", { path: path });
}

async function openLogFolder() {
  let path = await invoke("get_app_log_dir");
  await invoke("show_folder", { path: path });
}

async function showFolder(path: string) {
  await invoke("show_folder", { path: path });
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
          :label="t('setting.wt_root_path.label')"
          :placeholder="t('setting.wt_root_path.placeholder')"
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
                    {{ t("setting.button.auto_detect") }}
                    <v-tooltip activator="parent" location="bottom">
                      {{ t("setting.wt_root_path.auto_detect_tooltip") }}
                    </v-tooltip>
                  </v-btn>
                </v-col>
                <v-col cols="auto">
                  <v-btn color="primary" @click="selectWTInstallPath">
                    {{ t("setting.button.manual_select") }}
                  </v-btn>
                </v-col>
                <v-col cols="auto">
                  <v-btn
                    color="info"
                    :disabled="appSettings.wt_root_path == null"
                    @click="showFolder(appSettings.wt_root_path)"
                  >
                    {{ t("setting.button.open_folder") }}
                  </v-btn>
                </v-col>
              </v-row>
            </v-container>
          </template>
        </v-text-field>
      </v-col>
      <v-col cols="12">
        <v-text-field
          v-model="appSettings.wt_setting_path"
          :label="t('setting.wt_setting_path.label')"
          :placeholder="t('setting.wt_setting_path.placeholder')"
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
                    {{ t("setting.button.auto_detect") }}
                    <v-tooltip activator="parent" location="bottom">
                      {{ t("setting.wt_setting_path.auto_detect_tooltip") }}
                    </v-tooltip>
                  </v-btn>
                </v-col>
                <v-col cols="auto">
                  <v-btn color="primary" @click="selectWTSettingPath">
                    {{ t("setting.button.manual_select") }}
                  </v-btn>
                </v-col>
                <v-col cols="auto">
                  <v-btn
                    color="info"
                    :disabled="appSettings.wt_setting_path == null"
                    @click="showFolder(appSettings.wt_setting_path)"
                  >
                    {{ t("setting.button.open_folder") }}</v-btn
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
          :label="t('setting.wt_ext_cli_path.label')"
          :placeholder="t('setting.wt_ext_cli_path.placeholder')"
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
                    {{ t("setting.button.download_from_github") }}
                    <v-tooltip activator="parent" location="bottom">
                      {{ t("setting.button.download_from_github") }}
                    </v-tooltip>
                  </v-btn>
                </v-col>
                <v-col cols="auto">
                  <v-btn color="primary" @click="selectWTExtCliPath">
                    {{ t("setting.button.select_folder") }}</v-btn
                  >
                </v-col>
                <v-col cols="auto">
                  <v-btn
                    color="info"
                    :disabled="appSettings.wt_ext_cli_path == null"
                    @click="showFolder(appSettings.wt_ext_cli_path)"
                  >
                    {{ t("setting.button.open_folder") }}</v-btn
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
              <v-btn color="success" @click="saveSettings">
                {{ t("setting.button.save") }}
              </v-btn>
            </v-col>
            <v-col cols="auto">
              <v-btn color="info" @click="openSettingFolder">
                {{ t("setting.button.open_cfg_folder") }}
              </v-btn>
            </v-col>
            <v-col cols="auto">
              <v-btn color="info" @click="openLogFolder">
                {{ t("setting.button.open_log_folder") }}
              </v-btn>
            </v-col>
          </v-row>
        </v-container>
      </v-col>
    </v-row>
  </v-container>
  <CommonSnackbar v-model="snackbar" />
</template>

<style scoped></style>
