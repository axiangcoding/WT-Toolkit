<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { computed, onMounted, ref } from "vue";
import { get_error_msg } from "../error_msg";
import CommonSnackbar from "../components/snackbar/CommonSnackbar.vue";
import VersionCmd from "../components/wt_ext_cli_cmd_card/VersionCmd.vue";
import HelpCmd from "../components/wt_ext_cli_cmd_card/HelpCmd.vue";
import UnpackRawBlkCmd from "../components/wt_ext_cli_cmd_card/UnpackRawBlkCmd.vue";
import UnpackDxpAndGrpCmd from "../components/wt_ext_cli_cmd_card/UnpackDxpAndGrpCmd.vue";
import UnpackVromf from "../components/wt_ext_cli_cmd_card/UnpackVromf.vue";
import VromfVersion from "../components/wt_ext_cli_cmd_card/VromfVersion.vue";

import { CmdResult } from "../schema";
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
    title: t("app.nav_drawer.wt_ext_cli"),
    disabled: true,
    href: "/setting",
  },
]);

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
          :title="t('wt_ext_cli.usage.title')"
          variant="tonal"
          closable
          type="info"
        >
          <p>
            {{ t("wt_ext_cli.usage.content1") }}
          </p>
          <p>
            {{ t("wt_ext_cli.usage.content2") }}
          </p>
        </v-alert>
        <v-divider class="my-1" thickness="0"></v-divider>
        <v-alert
          icon="mdi-alert-box"
          :title="t('wt_ext_cli.disclaimer.title')"
          variant="tonal"
          type="warning"
          closable
        >
          <p>
            {{ t("wt_ext_cli.disclaimer.content1") }}
          </p>
          <strong>
            {{ t("wt_ext_cli.disclaimer.content2") }}
          </strong>
        </v-alert>
      </v-col>
      <v-col cols="12">
        {{ t("wt_ext_cli.tool_detected_and_version") }}
        <v-chip color="primary">{{ wtExtCliVersion }}</v-chip>
      </v-col>
      <v-col cols="12">
        <v-card>
          <v-tabs v-model="commandTab" color="primary">
            <v-tab value="unpackVromf">
              {{ t("wt_ext_cli.tab.unpack_vromf") }}</v-tab
            >
            <v-tab value="unpackDxpAndGrp">
              {{ t("wt_ext_cli.tab.unpack_dxp_and_grp") }}</v-tab
            >
            <v-tab value="unpackRawBlk">
              {{ t("wt_ext_cli.tab.unpack_raw_blk") }}</v-tab
            >
            <v-tab value="vromfVersion">
              {{ t("wt_ext_cli.tab.vromf_version") }}</v-tab
            >
            <v-tab value="version"> {{ t("wt_ext_cli.tab.version") }}</v-tab>
            <v-tab value="help"> {{ t("wt_ext_cli.tab.help") }}</v-tab>
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
