<script setup lang="ts">
import { onMounted, ref, shallowRef, watch } from "vue";

import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";
import { listen } from "@tauri-apps/api/event";

import { useI18n } from "vue-i18n";
import UserSightCard from "../components/card/UserSightCard.vue";
import LoadUserSightDialog from "../components/dialog/LoadUserSightDialog.vue";
import CommonSnackbar from "../components/snackbar/CommonSnackbar.vue";
import { get_error_msg } from "../error_msg";

const { t } = useI18n();

const breadcrumbsItems = [
  {
    title: t("app.nav_drawer.home"),
    disabled: false,
    href: "/",
  },
  {
    title: t("app.nav_drawer.sub_header.wt_tools"),
    disabled: true,
  },
  {
    title: t("app.nav_drawer.wt_sight"),
    disabled: true,
    href: "/wt-sight",
  },
];

const appSettings = ref<{
  wt_root_path: string;
  wt_setting_path: string;
}>({} as any);

onMounted(async () => {
  appSettings.value = await invoke("get_app_config");
  await loadUserSights();

  await listen("tauri://file-drop", async (event) => {
    if (appSettings.value.wt_root_path == null) {
      snackbar.value = {
        show: true,
        message: t("wt_sight.prepare_wt_root_path"),
        color: "warning",
      };
      return;
    }

    let payload: string[] = event.payload as string[];
    pathToLoad.value = payload[0];
  });
});

const userSights = ref<any>([]);

const showEmptyState = ref(false);
const pathToLoad = ref("");

const snackbar = ref({
  show: false,
  message: "",
  color: "success",
});

async function loadUserSights() {
  let wtInstallPath = appSettings.value.wt_root_path;
  if (!wtInstallPath) {
    showEmptyState.value = true;
    return;
  }

  userSights.value = await invoke("get_user_sights", {
    wtInstallPath: wtInstallPath,
  });
  countTotalSize();
  snackbar.value = {
    show: true,
    message: t("wt_sight.load_user_sights_success"),
    color: "success",
  };
}

const sizeInStr = ref("");

function countTotalSize() {
  let totalSize = 0;
  userSights.value.forEach((skin: any) => {
    totalSize += skin.folder_size / 1024 / 1024;
  });
  if (totalSize > 1024) {
    sizeInStr.value = (totalSize / 1024).toFixed(2) + " GB";
  } else {
    sizeInStr.value = totalSize.toFixed(2) + " MB";
  }
}

async function selectSightPath(directory: boolean) {
  let filters = [
    {
      name: t("wt_sight.file_type.zip_files"),
      extensions: ["zip", "7z"],
    },
    {
      name: t("wt_sight.file_type.all_files"),
      extensions: [""],
    },
  ];
  let selectedPath = await open({
    directory: directory,
    multiple: false,
    filters: filters,
  });
  if (typeof selectedPath === "string") {
    pathToLoad.value = selectedPath;
  } else if (Array.isArray(selectedPath)) {
    pathToLoad.value = selectedPath[0];
  }
}

const showConfirmSightDialog = ref(false);

watch(pathToLoad, async (newVal) => {
  if (newVal) {
    showConfirmSightDialog.value = true;
  }
});

watch(showConfirmSightDialog, async (newVal) => {
  if (!newVal) {
    pathToLoad.value = "";
  }
});

const page = ref(1);
const search = shallowRef("");

function filterUserSights(value: string, query: string, _item?: any) {
  if (!query) return true;
  if (value.toLowerCase().includes(query.toLowerCase())) {
    return true;
  }
  return false;
}

const deleteSightDialog = ref({
  show: false,
  data: {} as any,
});

async function openDeleteSightDialog(data: any) {
  deleteSightDialog.value = {
    show: true,
    data: data,
  };
}

async function deleteSight(folder_path: string) {
  await invoke("delete_folder", { path: folder_path });
  deleteSightDialog.value = {
    show: false,
    data: {},
  };
  await loadUserSights();
}

const installLoading = ref(false);
async function startLoadSight() {
  try {
    installLoading.value = true;
    await invoke("install_user_sight", {
      sightPath: pathToLoad.value,
      wtInstallPath: appSettings.value.wt_root_path,
    });
    await loadUserSights();
    pathToLoad.value = "";
    snackbar.value = {
      show: true,
      message: t("wt_sight.install_sight_success"),
      color: "success",
    };
  } catch (error) {
    pathToLoad.value = "";
    snackbar.value = {
      show: true,
      message: get_error_msg(error),
      color: "error",
    };
  } finally {
    installLoading.value = false;
  }
}
</script>

<template>
  <v-breadcrumbs :items="breadcrumbsItems"></v-breadcrumbs>
  <v-container>
    <v-row>
      <v-col cols="12">
        <v-alert
          icon="mdi-tooltip"
          :title="t('wt_sight.usage.title')"
          variant="tonal"
          closable
          type="info"
        >
          {{ t("wt_sight.usage.content1") }}
        </v-alert>
        <v-divider class="my-1" thickness="0"></v-divider>
        <v-alert
          icon="mdi-alert-box"
          :title="t('wt_sight.disclaimer.title')"
          variant="tonal"
          type="warning"
          closable
        >
          <div>
            {{ t("wt_sight.disclaimer.content1") }}
          </div>
          <strong>
            {{ t("wt_sight.disclaimer.content2") }}
          </strong>
        </v-alert>
      </v-col>

      <v-col cols="12">
        <span class="text-h5">
          {{ t("wt_sight.install_sight") }}
        </span>
      </v-col>
      <v-col cols="12" align="center">
        <v-card
          border="dashed"
          variant="outlined"
          :disabled="appSettings.wt_root_path == null"
        >
          <v-card-title>
            {{ t("wt_sight.select_sight") }}
          </v-card-title>
          <v-card-text>
            <div>{{ t("wt_sight.select_sight_tip1") }}</div>
            <div>
              {{ t("wt_sight.select_sight_tip2") }}
            </div>
            <div>
              {{ t("wt_sight.select_sight_tip3") }}
            </div>
          </v-card-text>
          <v-card-actions>
            <v-btn color="primary" @click="selectSightPath(true)">{{
              t("wt_sight.button.select_folder")
            }}</v-btn>
            <v-btn color="primary" @click="selectSightPath(false)">
              {{ t("wt_sight.button.select_zip_file") }}
            </v-btn>
          </v-card-actions>
        </v-card>
      </v-col>
      <v-col cols="12">
        <span class="text-h5">{{ t("wt_sight.loaded_sight") }}</span>
      </v-col>
      <v-col cols="12" v-show="showEmptyState">
        <v-empty-state
          icon="mdi-alert-circle-outline"
          :headline="t('wt_sight.empty_state.title')"
          :title="t('wt_sight.empty_state.content')"
        >
          <template v-slot:media>
            <v-icon color="warning"></v-icon>
          </template>

          <template v-slot:text>
            <div class="">
              {{ t("wt_sight.empty_state.jump_to1") }}
              <v-btn to="setting" color="info" variant="text">
                {{ t("app.nav_drawer.settings") }}
              </v-btn>
              {{ t("wt_sight.empty_state.jump_to2") }}
            </div>
          </template>
        </v-empty-state>
      </v-col>
      <v-col cols="12" v-show="!showEmptyState">
        <v-data-iterator
          :items="userSights"
          :items-per-page="6"
          :search="search"
          :custom-filter="filterUserSights"
          filter-keys="vehicle_id"
          :page="page"
          :sort-by="[{ key: 'vehicle_id', order: 'asc' }]"
        >
          <template v-slot:header>
            <v-toolbar color="white">
              <v-chip> {{ t("wt_sight.total_space") }} {{ sizeInStr }} </v-chip>

              <v-spacer></v-spacer>
              <v-text-field
                v-model="search"
                density="comfortable"
                :placeholder="t('wt_sight.filter_placeholder')"
                prepend-inner-icon="mdi-magnify"
                style="max-width: 300px"
                variant="outlined"
                clearable
                hide-details
              ></v-text-field>

              <v-btn
                class="mx-2"
                color="primary"
                icon="mdi-refresh"
                @click="loadUserSights"
              ></v-btn>
            </v-toolbar>
          </template>
          <template v-slot:default="{ items }">
            <v-row>
              <v-col v-for="(item, i) in items" :key="i" cols="auto" md="4">
                <!-- {{ item.raw }} -->
                <UserSightCard
                  :sightMetadata="item.raw"
                  @delete-sight="openDeleteSightDialog"
                />
              </v-col>
            </v-row>
          </template>

          <template v-slot:footer="{ pageCount, prevPage, nextPage }">
            <v-pagination
              v-model="page"
              :length="pageCount"
              @next="nextPage"
              @prev="prevPage"
            ></v-pagination>
          </template>
        </v-data-iterator>
      </v-col>
    </v-row>
  </v-container>
  <v-dialog v-model="deleteSightDialog.show" width="auto">
    <v-card prepend-icon="mdi-alert">
      <template v-slot:title>
        <span class="font-weight-black">
          {{ t("wt_sight.dialog.delete_user_skin") }}
          {{ deleteSightDialog.data.skin_name }}
        </span>
      </template>
      <v-card-title></v-card-title>
      <v-card-text>
        {{ t("wt_sight.dialog.tip1") }}
      </v-card-text>
      <template v-slot:actions>
        <v-btn
          color="error"
          :text="t('wt_sight.dialog.confirm')"
          @click="deleteSight(deleteSightDialog.data.full_path)"
        ></v-btn>
        <v-btn
          :text="t('wt_sight.dialog.cancel')"
          @click="deleteSightDialog.show = false"
        ></v-btn>
      </template>
    </v-card>
  </v-dialog>
  <CommonSnackbar v-model="snackbar" />
  <LoadUserSightDialog
    v-model="showConfirmSightDialog"
    :sight-path="pathToLoad"
    @confirm="startLoadSight"
  />
  <v-overlay
    :model-value="installLoading"
    class="align-center justify-center"
    persistent
  >
    <v-progress-circular
      :size="70"
      :width="7"
      color="primary"
      indeterminate
    ></v-progress-circular>
  </v-overlay>
</template>

<style scoped></style>
