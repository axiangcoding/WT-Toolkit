<script setup lang="ts">

import { onMounted, ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api';
import { AppSettings, getAppSettings } from '../settings';
import { listen } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/api/dialog';
import UserSkinCard from '../components/card/UserSkinCard.vue';
import CommonSnackbar from '../components/snackbar/CommonSnackbar.vue';
import LoadUserSkinDialog from '../components/dialog/LoadUserSkinDialog.vue';


const breadcrumbsItems = [
  {
    title: '主页',
    disabled: false,
    href: '/',
  },
  {
    title: "战雷小工具",
    disabled: true,
  },
  {
    title: '自定义涂装管理',
    disabled: true,
    href: '/wt-skins',
  },
]

const appSettings = ref<AppSettings>({} as AppSettings)

const userSkins = ref<any>([])
const totalUserSkinsSizeInMB = ref(0)

const showEmptyState = ref(false)

const showConfirmSkinDialog = ref(false)

const pathToLoad = ref('')
const installLoading = ref(false)

const snackbar = ref({
  show: false,
  message: '',
  color: 'success'
})

onMounted(async () => {
  appSettings.value = await getAppSettings()
  await loadUserSkins()

  await listen("tauri://file-drop", async (event) => {
    if (appSettings.value.wt_install_path == null) {
      snackbar.value = {
        show: true,
        message: '请先配置战争雷霆游戏安装目录',
        color: 'warning'
      }
      return
    }

    let payload: string[] = event.payload as string[]
    pathToLoad.value = payload[0]
  })
})

async function loadUserSkins() {
  let user_skin_path = appSettings.value.wt_install_path
  if (!user_skin_path) {
    showEmptyState.value = true
    return
  }

  userSkins.value = await invoke('get_user_skins', { wtInstallPath: user_skin_path })
  countTotalSize()
  // sortUserSKins()
  snackbar.value = {
    show: true,
    message: '自定义涂装列表加载成功',
    color: 'success'
  }
}

const deleteSkinDialog = ref({
  show: false,
  data: {} as any
})



async function openDeleteSkinDialog(skin: any) {
  deleteSkinDialog.value = {
    show: true,
    data: skin

  }
}

async function deleteSkin(skin_folder_path: string) {
  await invoke('delete_folder', { path: skin_folder_path })
  deleteSkinDialog.value = {
    show: false,
    data: {}
  }
  await loadUserSkins()
}

function countTotalSize() {
  let totalSize = 0
  userSkins.value.forEach((skin: any) => {
    totalSize += (skin.folder_size / 1024 / 1024)
  })
  totalUserSkinsSizeInMB.value = totalSize
}

async function selectSkinPath(directory: boolean) {
  let filters = [
    {
      name: '压缩包',
      extensions: ['zip', "7z"]
    },
    {
      name: '文件夹',
      extensions: ['']
    }
  ]
  let selectedPath = await open({
    directory: directory,
    multiple: false,
    filters: filters
  })
  if (typeof selectedPath === 'string') {
    pathToLoad.value = selectedPath
  } else if (Array.isArray(selectedPath)) {
    pathToLoad.value = selectedPath[0]
  }
}

async function startLoadSkin() {
  try {
    installLoading.value = true
    await invoke('install_user_skin', { skinPath: pathToLoad.value, wtInstallPath: appSettings.value.wt_install_path })
    await loadUserSkins()
    pathToLoad.value = ''
    snackbar.value = {
      show: true,
      message: '自定义涂装安装成功',
      color: 'success'
    }
  } catch (error) {
    pathToLoad.value = ''
    snackbar.value = {
      show: true,
      message: error as string,
      color: 'error'
    }
  } finally {
    installLoading.value = false
  }
}

const sortOrder = ref(1)

function sortUserSKins() {
  userSkins.value = userSkins.value.sort((a: any, b: any) => {
    if (sortOrder.value === 1) {
      return a.skin_name.localeCompare(b.skin_name)
    } else {
      return b.skin_name.localeCompare(a.skin_name)
    }
  })
  sortOrder.value = sortOrder.value * -1
}

watch(pathToLoad, async (newVal) => {
  if (newVal) {
    showConfirmSkinDialog.value = true
  }
})

watch(showConfirmSkinDialog, async (newVal) => {
  if (!newVal) {
    pathToLoad.value = ''
  }
})

</script>


<template>

  <v-breadcrumbs :items="breadcrumbsItems"></v-breadcrumbs>
  <v-container>
    <v-row>
      <v-col cols="12">
        <v-alert icon="mdi-tooltip" title="使用说明" variant="tonal" closable
          text="下载了自定义涂装后，你可以使用本工具进行一键安装。不过，在使用前，你还需要进入到 “设置”页面 配置好 “战争雷霆游戏安装目录” 配置项，这样小工具才能正确管理你的自定义涂装"
          type="info"></v-alert>
        <v-divider class="my-1" thickness="0"></v-divider>
        <v-alert icon="mdi-alert-box" title="免责声明" variant="tonal" type="warning" closable>
          本工具只会读取和写入《战争雷霆》游戏安装目录下的UserSkins文件夹，这个文件夹是游戏官方提供的自定义涂装文件夹，
          因此<strong>使用本工具不存在任何导致游戏账号被封禁的风险</strong>。
          <div><strong>用户应当对自己所安装的涂装来源和内容负责。因用户不当使用导致的任何问题，本工具和作者概不负责！</strong></div>
        </v-alert>
      </v-col>

      <v-col cols="12">
        <span class="text-h5">一键安装自定义涂装！</span>
      </v-col>
      <v-col cols="12" align="center">

        <v-card border="dashed" variant="outlined" :disabled="appSettings.wt_install_path == null">
          <v-card-title>选择自定义涂装的压缩包或者文件夹</v-card-title>
          <v-card-text>
            <div>小工具支持通过两种形式选择需要安装的自定义涂装</div>
            <div>你可以直接将对应的文件夹或者压缩包拖拽到小工具上，或者点击下方的按钮进行手动选择</div>
            <div>压缩包支持的格式有：zip, 7z <strong>（暂不支持带密码压缩包）</strong></div>
          </v-card-text>
          <v-card-actions>
            <v-btn color="primary" @click="selectSkinPath(true)">选择文件夹</v-btn>
            <v-btn color="primary" @click="selectSkinPath(false)">选择压缩包</v-btn>

          </v-card-actions>

        </v-card>
      </v-col>
      <v-col cols="6">
        <div class="text-h5">已安装的自定义涂装</div>
      </v-col>
      <v-col cols="6" align="right">

        <v-chip>总空间占用：{{ totalUserSkinsSizeInMB.toFixed(2) }} MB</v-chip>
        <v-divider class="mx-3" vertical />
        <v-btn color="primary" icon="mdi-sort" @click="sortUserSKins"></v-btn>
        <v-divider class="mx-1" vertical />
        <v-btn color="primary" icon="mdi-refresh" @click="loadUserSkins"></v-btn>
      </v-col>

      <v-col cols="12" v-show="showEmptyState">
        <v-empty-state icon="mdi-alert-circle-outline" headline="无法识别目录" title="请检查“设置”中的“战争雷霆游戏安装目录”是否配置正确">
          <template v-slot:media>
            <v-icon color="warning"></v-icon>
          </template>

          <template v-slot:text>
            <div class="">
              跳转到 <v-btn to="setting" color="info" variant="text">设置</v-btn>
              界面检查配置项是否设置正确，只有在目录正确的情况下小工具才能正确检测到您的自定义涂装目录
            </div>

          </template>
        </v-empty-state>
      </v-col>

      <v-col cols="4" v-for="skin in userSkins" :key="skin.name" v-show="!showEmptyState">
        <UserSkinCard :skinMetadata="skin" @delete-skin="openDeleteSkinDialog" />
      </v-col>
    </v-row>
    <!-- <v-row>
      <v-col cols="6" offset="6" align="right">
        <v-btn color="info">查看备份文件夹</v-btn>
      </v-col>
    </v-row> -->
  </v-container>
  <v-dialog v-model="deleteSkinDialog.show" width="auto">
    <v-card prepend-icon="mdi-alert">
      <template v-slot:title>
        <span class="font-weight-black">删除自定义涂装 {{ deleteSkinDialog.data.skin_name }}</span>
      </template>
      <v-card-title></v-card-title>
      <v-card-text>删除后无法恢复，确定要删除这个自定义皮肤吗？我们建议您备份后再删除</v-card-text>
      <template v-slot:actions>
        <v-btn color="error" text="确定" @click="deleteSkin(deleteSkinDialog.data.full_path)"></v-btn>
        <v-btn text="取消" @click="deleteSkinDialog.show = false"></v-btn>
      </template>
    </v-card>
  </v-dialog>
  <CommonSnackbar v-model="snackbar" />
  <LoadUserSkinDialog v-model="showConfirmSkinDialog" :skin-path="pathToLoad" @confirm="startLoadSkin" />
  <v-overlay :model-value="installLoading" class="align-center justify-center" persistent>
    <v-progress-circular :size="70" :width="7" color="primary" indeterminate></v-progress-circular>
  </v-overlay>
</template>

<style scoped></style>