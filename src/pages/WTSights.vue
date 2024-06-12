<script setup lang="ts">
import { onMounted, ref, shallowRef, watch } from "vue";
import { AppSettings, getAppSettings } from "../settings";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";

import UserSightCard from "../components/card/UserSightCard.vue";
import CommonSnackbar from "../components/snackbar/CommonSnackbar.vue";
import LoadUserSightDialog from "../components/dialog/LoadUserSightDialog.vue";

const breadcrumbsItems = [
  {
    title: "主页",
    disabled: false,
    href: "/",
  },
  {
    title: "战雷小工具",
    disabled: true,
  },
  {
    title: "自定义瞄具管理",
    disabled: true,
    href: "/wt-sight",
  },
];

const appSettings = ref<AppSettings>({} as AppSettings);

onMounted(async () => {
  appSettings.value = await getAppSettings();
  await loadUserSights();

  await listen("tauri://file-drop", async (event) => {
    if (appSettings.value.wt_install_path == null) {
      snackbar.value = {
        show: true,
        message: "请先配置战争雷霆游戏安装目录",
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
  let wtInstallPath = appSettings.value.wt_install_path;
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
    message: "自定义涂装列表加载成功",
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
      name: "压缩包",
      extensions: ["zip", "7z"],
    },
    {
      name: "文件夹",
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
      wtInstallPath: appSettings.value.wt_install_path,
    });
    await loadUserSights();
    pathToLoad.value = "";
    snackbar.value = {
      show: true,
      message: "自定义涂装安装成功",
      color: "success",
    };
  } catch (error) {
    pathToLoad.value = "";
    snackbar.value = {
      show: true,
      message: error as string,
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
          title="使用说明"
          variant="tonal"
          closable
          text="下载了自定义瞄具后，你可以使用本工具进行一键安装。不过，在使用前，你还需要进入到 “设置”页面 配置好 “战争雷霆游戏安装目录” 配置项，这样小工具才能正确管理你的自定义瞄具"
          type="info"
        ></v-alert>
        <v-divider class="my-1" thickness="0"></v-divider>
        <v-alert
          icon="mdi-alert-box"
          title="免责声明"
          variant="tonal"
          type="warning"
          closable
        >
          本工具只会读取和写入《战争雷霆》游戏安装目录下的UserSights文件夹，这个文件夹是游戏官方提供的自定义瞄具文件夹，
          因此
          <strong> 使用本工具不存在任何导致游戏账号被封禁的风险 </strong>。
          <div>
            <strong>
              用户应当对自己所安装的瞄具来源和内容负责。因用户不当使用导致的任何问题，本工具和作者概不负责！
            </strong>
          </div>
        </v-alert>
      </v-col>

      <v-col cols="12">
        <span class="text-h5">一键安装自定义瞄具！</span>
      </v-col>
      <v-col cols="12" align="center">
        <v-card
          border="dashed"
          variant="outlined"
          :disabled="appSettings.wt_install_path == null"
        >
          <v-card-title>选择自定义瞄具的压缩包或者文件夹</v-card-title>
          <v-card-text>
            <div>小工具支持通过两种形式选择需要安装的自定义瞄具</div>
            <div>
              你可以直接将对应的文件夹或者压缩包拖拽到小工具上，或者点击下方的按钮进行手动选择
            </div>
            <div>
              压缩包支持的格式有：zip, 7z
              <strong>（暂不支持带密码压缩包）</strong>
            </div>
          </v-card-text>
          <v-card-actions>
            <v-btn color="primary" @click="selectSightPath(true)"
              >选择文件夹</v-btn
            >
            <v-btn color="primary" @click="selectSightPath(false)"
              >选择压缩包</v-btn
            >
          </v-card-actions>
        </v-card>
      </v-col>
      <v-col cols="12">
        <span class="text-h5">已加载的自定义瞄具</span>
      </v-col>
      <v-col cols="12" v-show="showEmptyState">
        <v-empty-state
          icon="mdi-alert-circle-outline"
          headline="无法识别目录"
          title="请检查“设置”中的“战争雷霆游戏安装目录”是否配置正确"
        >
          <template v-slot:media>
            <v-icon color="warning"></v-icon>
          </template>

          <template v-slot:text>
            <div class="">
              跳转到
              <v-btn to="setting" color="info" variant="text">设置</v-btn>
              界面检查配置项是否设置正确，只有在目录正确的情况下小工具才能正确检测到您的自定义瞄具目录
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
              <v-chip> 总空间占用：{{ sizeInStr }} </v-chip>

              <v-spacer></v-spacer>
              <v-text-field
                v-model="search"
                density="comfortable"
                placeholder="筛选"
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
          删除自定义涂装 {{ deleteSightDialog.data.skin_name }}
        </span>
      </template>
      <v-card-title></v-card-title>
      <v-card-text>
        删除后无法恢复，确定要删除这个自定义皮肤吗？我们建议您备份后再删除
      </v-card-text>
      <template v-slot:actions>
        <v-btn
          color="error"
          text="确定"
          @click="deleteSight(deleteSightDialog.data.full_path)"
        ></v-btn>
        <v-btn text="取消" @click="deleteSightDialog.show = false"></v-btn>
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
