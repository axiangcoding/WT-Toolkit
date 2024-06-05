<script setup lang="ts">
import image1 from '@/assets/images/china_ground_newmodificationresearch.png'
import { onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api';
import { AppSettings, getAppSettings } from '../settings';
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
const showEmptyState = ref(false)

onMounted(async () => {
  appSettings.value = await getAppSettings()
  await loadUserSkins()
})

async function loadUserSkins() {
  let user_skin_path = appSettings.value.wt_install_path
  if (!user_skin_path) {
    showEmptyState.value = true
    return
  }
  userSkins.value = await invoke('get_user_skins_info', { path: user_skin_path })
}

async function showSkin(skin_folder_path: string) {
  await invoke('show_in_folder', { path: skin_folder_path })
}

const deleteSkinDialog = ref(false)
const deleteSkinDialogData = ref<any>({})

async function openDeleteSkinDialog(skin: any) {
  deleteSkinDialog.value = true
  deleteSkinDialogData.value = skin
}

async function deleteSkin(skin_folder_path: string) {
  await invoke('delete_folder', { path: skin_folder_path })
  deleteSkinDialog.value = false
  await loadUserSkins()
}

</script>


<template>
  <v-breadcrumbs :items="breadcrumbsItems"></v-breadcrumbs>
  <v-container>
    <v-row>
      <v-col cols="12">
        <v-alert icon="mdi-tooltip" title="使用说明"
          text="当你从涂装站下载了自定义涂装的压缩包之后，你可以使用本工具进行一键安装。不过，在使用前，你还需要进入到 “设置”页面 配置好 “战争雷霆游戏安装目录” 配置项，这样程序才能正确管理你的自定义涂装"
          type="info"></v-alert>
      </v-col>
      <!-- <v-col cols="12">
        <v-alert icon="mdi-alert-box" title="免责声明"
          text="针对自定义涂装的任何操作均只限于读取和写入 战争雷霆游戏安装目录/UserSkins 下的文件，不会对游戏目录下的其他任何文件进行操作，因此不存在任何可能导致使用者游戏账号被封禁的可能。"
          type="warning"></v-alert>
      </v-col> -->
      <v-col cols="12">
        <span class="text-h4">一键安装自定义涂装！</span>
      </v-col>
      <v-col cols="12" variant="solo" align="center">
        <!-- <span>一键安装自定义涂装！</span> -->
        <v-btn color="primary" size="large">选择并加载自定义涂装</v-btn>
      </v-col>
      <v-col cols="6">
        <div class="text-h4">已安装的自定义涂装</div>
      </v-col>
      <v-col cols="6" align="right">
        <v-btn color="primary" icon="mdi-refresh" @click="loadUserSkins"></v-btn>
      </v-col>

      <v-col cols="12" v-show="showEmptyState">
        <v-empty-state icon="mdi-alert-circle-outline" headline="无法识别目录" title="请检查“设置”中的“战争雷霆游戏安装目录”是否配置正确">
          <template v-slot:media>
            <v-icon color="warning"></v-icon>
          </template>

          <template v-slot:text>
            <div class="">
              请检查 <v-btn to="setting" color="info" variant="tonal">设置</v-btn> 中的战争雷霆游戏安装目录是否正确，这样程序才能正确检测到您的自定义涂装目录
            </div>

          </template>
        </v-empty-state>
      </v-col>

      <v-col cols="4" v-for="skin in userSkins" :key="skin.name" v-show="!showEmptyState">

        <v-card>
          <v-img class="align-end text-white" height="200" :src="image1" cover>
            <v-card-title>{{ skin.name }}</v-card-title>
          </v-img>

          <v-card-text>
            <!-- <div>涂装介绍：[无]</div> -->
            <div>空间占用 {{ (skin.size_bytes / 1024 / 1024).toFixed(2) }}MB</div>
          </v-card-text>

          <v-card-actions>
            <v-btn color="primary" text="查看" @click="showSkin(skin.path)"></v-btn>
            <v-btn color="warning" text="备份"></v-btn>
            <v-btn color="error" text="删除" @click="openDeleteSkinDialog(skin)"></v-btn>

          </v-card-actions>
        </v-card>

      </v-col>
    </v-row>
    <v-row>
      <v-col cols="6" offset="6" align="right">
        <v-btn color="info">查看备份文件夹</v-btn>
      </v-col>
    </v-row>



  </v-container>
  <v-dialog v-model="deleteSkinDialog" width="auto">
    <v-card prepend-icon="mdi-alert">
      <template v-slot:title>
        <span class="font-weight-black">删除自定义涂装 {{ deleteSkinDialogData.name }}</span>
      </template>
      <v-card-title></v-card-title>
      <v-card-text>删除后无法恢复，确定要删除这个自定义皮肤吗？我们建议您备份后再删除</v-card-text>
      <template v-slot:actions>
        <v-btn color="error" text="确定" @click="deleteSkin(deleteSkinDialogData.path)"></v-btn>
        <v-btn text="取消" @click="deleteSkinDialog = false"></v-btn>
      </template>
    </v-card>
  </v-dialog>
</template>

<style scoped></style>