<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";

import { CmdResult } from "../../schema";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const cmdOutput = ref<CmdResult>({} as any);

const args = ref(["--version"]);

async function exec() {
  loading.value = true;
  cmdOutput.value = await invoke("exec_wt_ext_cli", { args: args.value });
  loading.value = false;
}

async function cleanOutput() {
  cmdOutput.value = {} as any;
}

const loading = ref(false);
</script>

<template>
  <v-list>
    <v-list-item>
      <v-list-item-title>
        {{ t("wt_ext_cli.cmd_card.label.command") }}
      </v-list-item-title>
      <v-chip color="primary">wt-ext-cli --version</v-chip>
    </v-list-item>
    <v-list-item>
      <v-list-item-title>
        {{ t("wt_ext_cli.cmd_card.label.description") }}
      </v-list-item-title>
      {{ t("wt_ext_cli.cmd_card.version.description") }}
    </v-list-item>
    <v-list-item>
      <v-list-item-title>
        {{ t("wt_ext_cli.cmd_card.label.args") }}
      </v-list-item-title>
      {{ t("wt_ext_cli.cmd_card.label.none") }}
    </v-list-item>
  </v-list>
  <div class="d-flex ga-2">
    <v-btn color="primary" @click="exec">
      {{ t("wt_ext_cli.cmd_card.button.execute") }}
    </v-btn>
    <v-btn color="warning" @click="cleanOutput">
      {{ t("wt_ext_cli.cmd_card.button.clean_output") }}
    </v-btn>
  </div>

  <v-divider class="my-3"></v-divider>

  <v-list>
    <v-list-item>
      <v-list-item-title>
        {{ t("wt_ext_cli.cmd_card.label.exec_result") }}
      </v-list-item-title>
      <v-chip
        variant="elevated"
        color="success"
        v-if="cmdOutput.code != null && cmdOutput.code == 0"
      >
        {{ t("wt_ext_cli.cmd_card.label.exec_success") }}
      </v-chip>
      <v-chip
        variant="elevated"
        color="error"
        v-if="cmdOutput.code != null && cmdOutput.code != 0"
      >
        {{ t("wt_ext_cli.cmd_card.label.exec_error") }}
      </v-chip>
      <v-progress-linear
        v-if="loading"
        color="primary"
        indeterminate
        striped
        height="10"
      ></v-progress-linear>
    </v-list-item>
    <v-list-item>
      <v-list-item-title>
        {{ t("wt_ext_cli.cmd_card.label.console_output") }}
      </v-list-item-title>
      <v-code class="console-box border-success">
        {{ cmdOutput.stdout ? cmdOutput.stdout : cmdOutput.stderr }}
      </v-code>
    </v-list-item>
  </v-list>
</template>

<style scoped></style>
