<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { onMounted, ref } from "vue";
import { get_error_msg } from "../error_msg";
import CommonSnackbar from "../components/snackbar/CommonSnackbar.vue";
import VersionCmd from "../components/wt_ext_cli_cmd_card/VersionCmd.vue";
import HelpCmd from "../components/wt_ext_cli_cmd_card/HelpCmd.vue";
import UnpackRawBlkCmd from "../components/wt_ext_cli_cmd_card/UnpackRawBlkCmd.vue";
import UnpackDxpAndGrpCmd from "../components/wt_ext_cli_cmd_card/UnpackDxpAndGrpCmd.vue";
import UnpackVromf from "../components/wt_ext_cli_cmd_card/UnpackVromf.vue";
import VromfVersion from "../components/wt_ext_cli_cmd_card/VromfVersion.vue";

import { CmdResult } from "../schema";

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
    title: "解包工具",
    disabled: true,
    href: "/setting",
  },
];

const wtExtCliVersion = ref("");

const commandTab = ref("unpackVromf");

const snackbar = ref({
  show: false,
  message: "",
  color: "success",
});

onMounted(async () => {
  try {
    let cmdResult: CmdResult = await invoke("exec_wt_ext_cli", {
      args: ["--version"],
    });
    wtExtCliVersion.value = cmdResult.stdout ?? "";
  } catch (error) {
    snackbar.value = {
      show: true,
      message: get_error_msg(error),
      color: "error",
    };
  }
});
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
          type="info"
        >
          <p>
            本工具是对开源解包工具 wt_ext_cli
            的封装，用于解包游戏资源文件，旨在降低解包门槛，提升玩家获取第一手游戏资源的能力。
          </p>
          <p>
            目前兼容的 wt_ext_cli 版本为 >
            v0.5.3，如果出现不兼容的情况，请联系开发者。
          </p>
        </v-alert>
        <v-divider class="my-1" thickness="0"></v-divider>
        <v-alert
          icon="mdi-alert-box"
          title="免责声明"
          variant="tonal"
          type="warning"
          closable
        >
          <p>
            本工具将调用 wt_ext_cli 进行解包，请自行确保你所指定的 wt_ext_cli
            路径是正确的。我们强烈建议在游戏关闭的情况下，将游戏资源文件复制到一个新的目录中进行解包，以免解包过程中对游戏文件造成损坏。
          </p>
          <strong>
            解包游戏资源是一项复杂的工作，如果你是一名普通玩家，而且不知道什么是解包，也不知道解包会带来什么风险，请不要使用本工具！
          </strong>
        </v-alert>
      </v-col>
      <v-col cols="12">
        检测到 wt_ext_cli 工具，当前的版本为：
        <v-chip color="primary">{{ wtExtCliVersion }}</v-chip>
      </v-col>
      <v-col cols="12">
        <v-card>
          <v-tabs v-model="commandTab" color="primary">
            <v-tab value="unpackVromf">解包 Vromf 文件</v-tab>
            <v-tab value="unpackDxpAndGrp">解包 DXP 和 GRP 文件</v-tab>
            <v-tab value="unpackRawBlk">解包二进制 blk 文件</v-tab>
            <v-tab value="vromfVersion">解析 Vromf 版本</v-tab>
            <v-tab value="version">版本</v-tab>
            <v-tab value="help">帮助</v-tab>
          </v-tabs>

          <v-card-text>
            <v-tabs-window v-model="commandTab">
              <v-tabs-window-item value="unpackVromf">
                <UnpackVromf />
              </v-tabs-window-item>
              <v-tabs-window-item value="unpackDxpAndGrp">
                <UnpackDxpAndGrpCmd />
              </v-tabs-window-item>
              <v-tabs-window-item value="vromfVersion">
                <VromfVersion />
              </v-tabs-window-item>
              <v-tabs-window-item value="unpackRawBlk">
                <UnpackRawBlkCmd />
              </v-tabs-window-item>
              <v-tabs-window-item value="version">
                <VersionCmd />
              </v-tabs-window-item>
              <v-tabs-window-item value="help">
                <HelpCmd />
              </v-tabs-window-item>
            </v-tabs-window>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
  <CommonSnackbar v-model="snackbar" />
</template>
