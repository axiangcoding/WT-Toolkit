<script setup lang="ts">
import { computed, onMounted, ref, shallowRef, watch } from "vue";
import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/api/dialog";
import UserSkinCard from "../components/card/UserSkinCard.vue";
import CommonSnackbar from "../components/snackbar/CommonSnackbar.vue";
import LoadUserSkinDialog from "../components/dialog/LoadUserSkinDialog.vue";
import { get_error_msg } from "../error_msg";
import { useI18n } from "vue-i18n";

const { t } = useI18n();
const breadcrumbsItems = computed(() => [
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
    title: t("app.nav_drawer.wt_skins"),
    disabled: true,
    href: "/wt-skins",
  },
]);

const appSettings = ref<{
  wt_root_path: string;
  wt_setting_path: string;
}>({} as any);

const userSkins = ref<any>([]);

const showEmptyState = ref(false);

const showConfirmSkinDialog = ref(false);

const pathToLoad = ref("");
const installLoading = ref(false);

const snackbar = ref({
  show: false,
  message: "",
  color: "success",
});

onMounted(async () => {
  appSettings.value = await invoke("get_app_config");
  await loadUserSkins();

  await listen("tauri://file-drop", async (event) => {
    if (appSettings.value.wt_root_path == null) {
      snackbar.value = {
        show: true,
        message: t("wt_skins.prepare_wt_root_path"),
        color: "warning",
      };
      return;
    }

    let payload: string[] = event.payload as string[];
    pathToLoad.value = payload[0];
  });
});

async function loadUserSkins() {
  let wtInstallPath = appSettings.value.wt_root_path;
  if (!wtInstallPath) {
    showEmptyState.value = true;
    return;
  }

  userSkins.value = await invoke("get_user_skins", {
    wtInstallPath: wtInstallPath,
  });
  countTotalSize();
  snackbar.value = {
    show: true,
    message: t("wt_skins.load_user_skins_success"),
    color: "success",
  };
}

const deleteSkinDialog = ref({
  show: false,
  data: {} as any,
});

async function openDeleteSkinDialog(skin: any) {
  deleteSkinDialog.value = {
    show: true,
    data: skin,
  };
}

async function deleteSkin(skin_folder_path: string) {
  await invoke("delete_folder", { path: skin_folder_path });
  deleteSkinDialog.value = {
    show: false,
    data: {},
  };
  await loadUserSkins();
}

const sizeInStr = ref("");

function countTotalSize() {
  let totalSize = 0;
  userSkins.value.forEach((skin: any) => {
    totalSize += skin.folder_size / 1024 / 1024;
  });
  if (totalSize > 1024) {
    sizeInStr.value = (totalSize / 1024).toFixed(2) + " GB";
  } else {
    sizeInStr.value = totalSize.toFixed(2) + " MB";
  }
}

async function selectSkinPath(directory: boolean) {
  let filters = [
    {
      name: t("wt_skins.file_type.zip_files"),
      extensions: ["zip", "7z"],
    },
    {
      name: t("wt_skins.file_type.all_files"),
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

async function startLoadSkin() {
  try {
    installLoading.value = true;
    await invoke("install_user_skin", {
      skinPath: pathToLoad.value,
    });
    await loadUserSkins();
    pathToLoad.value = "";
    snackbar.value = {
      show: true,
      message: t("wt_skins.install_skin_success"),
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

watch(pathToLoad, async (newVal) => {
  if (newVal) {
    showConfirmSkinDialog.value = true;
  }
});

watch(showConfirmSkinDialog, async (newVal) => {
  if (!newVal) {
    pathToLoad.value = "";
  }
});

const page = ref(1);
const search = shallowRef("");

function filterUserSkins(value: string, query: string, _item?: any) {
  if (!query) return true;
  if (value.toLowerCase().includes(query.toLowerCase())) {
    return true;
  }
  return false;
}
</script>

<template>
  <v-breadcrumbs :items="breadcrumbsItems"></v-breadcrumbs>
  <v-container>
    <v-row>
      <v-col cols="12">
        <v-alert
          icon="mdi-tooltip"
          :title="t('wt_skins.usage.title')"
          variant="tonal"
          closable
          type="info"
        >
          {{ t("wt_skins.usage.content1") }}
        </v-alert>
        <v-divider class="my-1" thickness="0"></v-divider>
        <v-alert
          icon="mdi-alert-box"
          :title="t('wt_skins.disclaimer.title')"
          variant="tonal"
          type="warning"
          closable
        >
          {{ t("wt_skins.disclaimer.content1") }}
          <div>
            <strong>
              {{ t("wt_skins.disclaimer.content2") }}
            </strong>
          </div>
        </v-alert>
      </v-col>

      <v-col cols="12">
        <span class="text-h5">
          {{ t("wt_skins.install_skin") }}
        </span>
      </v-col>
      <v-col cols="12" align="center">
        <v-card
          border="dashed"
          variant="outlined"
          :disabled="appSettings.wt_root_path == null"
        >
          <v-card-title>
            {{ t("wt_skins.select_skin") }}
          </v-card-title>
          <v-card-text>
            <div>{{ t("wt_skins.select_skin_tip1") }}</div>
            <div>
              {{ t("wt_skins.select_skin_tip2") }}
            </div>
            <div>
              {{ t("wt_skins.select_skin_tip3") }}
            </div>
          </v-card-text>
          <v-card-actions>
            <v-btn color="primary" @click="selectSkinPath(true)">
              {{ t("wt_skins.button.select_folder") }}
            </v-btn>
            <v-btn color="primary" @click="selectSkinPath(false)">
              {{ t("wt_skins.button.select_zip_file") }}
            </v-btn>
          </v-card-actions>
        </v-card>
      </v-col>
      <v-col cols="12">
        <span class="text-h5">{{ t("wt_skins.loaded_skins") }}</span>
      </v-col>
      <v-col cols="12" v-show="showEmptyState">
        <v-empty-state
          icon="mdi-alert-circle-outline"
          :headline="t('wt_skins.empty_state.title')"
          :title="t('wt_skins.empty_state.content')"
        >
          <template v-slot:media>
            <v-icon color="warning"></v-icon>
          </template>

          <template v-slot:text>
            <div class="">
              {{ t("wt_skins.empty_state.jump_to1") }}
              <v-btn to="setting" color="info" variant="text">
                {{ t("app.nav_drawer.settings") }}
              </v-btn>
              {{ t("wt_skins.empty_state.jump_to2") }}
            </div>
          </template>
        </v-empty-state>
      </v-col>
      <v-col cols="12" v-show="!showEmptyState">
        <v-data-iterator
          :items="userSkins"
          :items-per-page="6"
          :search="search"
          :custom-filter="filterUserSkins"
          filter-keys="vehicle_id"
          :page="page"
          :sort-by="[{ key: 'vehicle_id', order: 'desc' }]"
        >
          <template v-slot:header>
            <v-toolbar color="white">
              <v-chip> {{ t("wt_skins.total_space") }} {{ sizeInStr }} </v-chip>

              <v-spacer></v-spacer>
              <v-text-field
                v-model="search"
                density="comfortable"
                :placeholder="t('wt_skins.filter_placeholder')"
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
                @click="loadUserSkins"
              ></v-btn>
            </v-toolbar>
          </template>
          <template v-slot:default="{ items }">
            <v-row>
              <v-col v-for="(item, i) in items" :key="i" cols="auto" md="4">
                <UserSkinCard
                  :skinMetadata="item.raw"
                  @delete-skin="openDeleteSkinDialog"
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
  <v-dialog v-model="deleteSkinDialog.show" width="auto">
    <v-card prepend-icon="mdi-alert">
      <template v-slot:title>
        <span class="font-weight-black">
          {{ t("wt_skins.dialog.delete_user_skin") }}
          {{ deleteSkinDialog.data.skin_name }}
        </span>
      </template>
      <v-card-title></v-card-title>
      <v-card-text>
        {{ t("wt_skins.dialog.tip1") }}
      </v-card-text>
      <template v-slot:actions>
        <v-btn
          color="error"
          :text="t('wt_skins.dialog.confirm')"
          @click="deleteSkin(deleteSkinDialog.data.full_path)"
        ></v-btn>
        <v-btn
          :text="t('wt_skins.dialog.cancel')"
          @click="deleteSkinDialog.show = false"
        ></v-btn>
      </template>
    </v-card>
  </v-dialog>
  <CommonSnackbar v-model="snackbar" />
  <LoadUserSkinDialog
    v-model="showConfirmSkinDialog"
    :skin-path="pathToLoad"
    @confirm="startLoadSkin"
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
