<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";
import { CmdResult } from "../../schema";
import { open } from "@tauri-apps/api/dialog";
import { useI18n } from "vue-i18n";

const cmdOutput = ref<CmdResult>({} as any);

const cmdArgs = ref<{
  inputDir: string;
  outputDir: string;
  keepSuffix: boolean;
  help: boolean;
}>({} as any);

const { t } = useI18n();
async function exec() {
  let args = ["unpack_dxp_and_grp"];
  if (cmdArgs.value.inputDir) {
    args.push("--input_dir", cmdArgs.value.inputDir);
  }
  if (cmdArgs.value.outputDir) {
    args.push("--output_dir", cmdArgs.value.outputDir);
  }
  if (cmdArgs.value.keepSuffix) {
    args.push("--keep_suffix");
  }
  if (cmdArgs.value.help) {
    args.push("--help");
  }
  loading.value = true;
  cmdOutput.value = await invoke("exec_wt_ext_cli", { args: args });
  loading.value = false;
}

async function cleanOutput() {
  cmdOutput.value = {} as any;
}

async function selectPath() {
  let selectedPath = await open({
    directory: true,
    multiple: false,
  });
  return selectedPath;
}

async function selectInputDir() {
  let selectedPath = await selectPath();
  if (selectedPath != null) {
    if (typeof selectedPath == "string") {
      cmdArgs.value.inputDir = selectedPath;
    } else {
      cmdArgs.value.inputDir = selectedPath[0];
    }
  }
}

async function selectOutputDir() {
  let selectedPath = await selectPath();
  if (selectedPath != null) {
    if (typeof selectedPath == "string") {
      cmdArgs.value.outputDir = selectedPath;
    } else {
      cmdArgs.value.outputDir = selectedPath[0];
    }
  }
}

async function showOutputDir() {
  await invoke("show_folder", { path: cmdArgs.value.outputDir });
}

const loading = ref(false);
</script>

<template>
  <v-list>
    <v-list-item>
      <v-list-item-title>
        {{ t("wt_ext_cli.cmd_card.label.command") }}
      </v-list-item-title>
      <v-chip color="primary">wt-ext-cli unpack_dxp_and_grp</v-chip>
    </v-list-item>
    <v-list-item>
      <v-list-item-title>
        {{ t("wt_ext_cli.cmd_card.label.description") }}
      </v-list-item-title>
      {{ t("wt_ext_cli.cmd_card.unpack_dxp_and_grp.description") }}
    </v-list-item>
    <v-list-item>
      <v-list-item-title>
        {{ t("wt_ext_cli.cmd_card.label.args") }}
      </v-list-item-title>
      <v-container>
        <v-row dense>
          <v-col cols="6">
            <v-text-field
              :label="
                t('wt_ext_cli.cmd_card.unpack_dxp_and_grp.input_dir_label')
              "
              append-inner-icon="mdi-folder"
              v-model="cmdArgs.inputDir"
              @click:append-inner="selectInputDir"
              clearable
            >
            </v-text-field>
          </v-col>
          <v-col cols="6">
            <v-text-field
              :label="
                t('wt_ext_cli.cmd_card.unpack_dxp_and_grp.output_dir_label')
              "
              append-inner-icon="mdi-folder"
              v-model="cmdArgs.outputDir"
              @click:append-inner="selectOutputDir"
              clearable
            >
            </v-text-field>
          </v-col>
          <v-col cols="6">
            <v-switch
              v-model="cmdArgs.keepSuffix"
              :label="
                t('wt_ext_cli.cmd_card.unpack_dxp_and_grp.keep_suffix_label')
              "
              color="primary"
            ></v-switch>
          </v-col>
          <v-col cols="6">
            <v-switch
              v-model="cmdArgs.help"
              :label="
                t('wt_ext_cli.cmd_card.unpack_dxp_and_grp.show_help_label')
              "
              color="primary"
            ></v-switch>
          </v-col>
        </v-row>
      </v-container>
    </v-list-item>
  </v-list>
  <div class="d-flex ga-2">
    <v-btn color="primary" @click="exec">
      {{ t("wt_ext_cli.cmd_card.button.execute") }}
    </v-btn>
    <v-btn color="warning" @click="cleanOutput">
      {{ t("wt_ext_cli.cmd_card.button.clean_output") }}
    </v-btn>
    <v-btn color="info" @click="showOutputDir" :disabled="!cmdArgs.outputDir">
      {{ t("wt_ext_cli.cmd_card.button.show_output_dir") }}
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
