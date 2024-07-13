<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";
import { CmdResult } from "../../schema";
import { open } from "@tauri-apps/api/dialog";
import { useI18n } from "vue-i18n";

const { t } = useI18n();
const cmdOutput = ref<CmdResult>({} as any);

const cmdArgs = ref<{
  inputDirOrFile: string;
  outputDir: string;
  format: string;
  override: boolean;
  avif2png: string;
  crlf: boolean;
  zip: boolean;
  blkExtension: string;
  help: boolean;
}>({} as any);

async function exec() {
  let args = ["unpack_vromf"];
  if (cmdArgs.value.inputDirOrFile) {
    args.push("--input_dir_or_file", cmdArgs.value.inputDirOrFile);
  }
  if (cmdArgs.value.outputDir) {
    args.push("--output_dir", cmdArgs.value.outputDir);
  }
  if (cmdArgs.value.format) {
    args.push("--format", cmdArgs.value.format);
  }
  if (cmdArgs.value.override) {
    args.push("--override");
  }
  if (cmdArgs.value.avif2png) {
    args.push("--avif2png", cmdArgs.value.avif2png);
  }
  if (cmdArgs.value.crlf) {
    args.push("--crlf");
  }
  if (cmdArgs.value.zip) {
    args.push("--zip");
  }
  if (cmdArgs.value.blkExtension) {
    args.push("--blk_extension", cmdArgs.value.blkExtension);
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

async function selectInputDirOrFile(directory: boolean) {
  let selectedPath = await open({
    directory: directory,
    multiple: false,
  });
  if (selectedPath != null) {
    if (typeof selectedPath == "string") {
      cmdArgs.value.inputDirOrFile = selectedPath;
    } else {
      cmdArgs.value.inputDirOrFile = selectedPath[0];
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
      <v-chip color="primary">wt-ext-cli unpack_vromf</v-chip>
    </v-list-item>
    <v-list-item>
      <v-list-item-title>
        {{ t("wt_ext_cli.cmd_card.label.description") }}
      </v-list-item-title>
      {{ t("wt_ext_cli.cmd_card.unpack_vromf.description") }}
    </v-list-item>
    <v-list-item>
      <v-list-item-title>
        {{ t("wt_ext_cli.cmd_card.label.args") }}
      </v-list-item-title>
      <v-container>
        <v-row dense>
          <v-col cols="6">
            <v-text-field
              :label="t('wt_ext_cli.cmd_card.unpack_vromf.input_file_label')"
              v-model="cmdArgs.inputDirOrFile"
              clearable
            >
              <template #append-inner>
                <v-icon @click="selectInputDirOrFile(true)">mdi-folder</v-icon>
                <v-icon @click="selectInputDirOrFile(false)">mdi-file</v-icon>
              </template>
            </v-text-field>
          </v-col>
          <v-col cols="6">
            <v-text-field
              :label="t('wt_ext_cli.cmd_card.unpack_vromf.output_dir_label')"
              append-inner-icon="mdi-folder"
              v-model="cmdArgs.outputDir"
              @click:append-inner="selectOutputDir"
              clearable
            >
            </v-text-field>
          </v-col>

          <v-col cols="6">
            <v-select
              clearable
              :label="t('wt_ext_cli.cmd_card.unpack_vromf.format_label')"
              :items="['Json', 'BlkText', 'Raw']"
              v-model="cmdArgs.format"
            ></v-select>
          </v-col>
          <v-col cols="6">
            <v-select
              clearable
              :label="t('wt_ext_cli.cmd_card.unpack_vromf.avif2png_label')"
              :items="['imagemagick', 'ffmpeg']"
              v-model="cmdArgs.avif2png"
            ></v-select>
          </v-col>
          <v-col cols="6">
            <v-text-field
              :label="t('wt_ext_cli.cmd_card.unpack_vromf.blk_extension_label')"
              v-model="cmdArgs.blkExtension"
              clearable
            >
            </v-text-field>
          </v-col>
          <v-col cols="6">
            <v-switch
              v-model="cmdArgs.override"
              :label="t('wt_ext_cli.cmd_card.unpack_vromf.override_label')"
              color="primary"
            ></v-switch>
          </v-col>
          <v-col cols="6">
            <v-switch
              v-model="cmdArgs.crlf"
              :label="t('wt_ext_cli.cmd_card.unpack_vromf.crlf_label')"
              color="primary"
            ></v-switch>
          </v-col>
          <v-col cols="6">
            <v-switch
              v-model="cmdArgs.zip"
              :label="t('wt_ext_cli.cmd_card.unpack_vromf.zip_label')"
              color="primary"
            ></v-switch>
          </v-col>

          <v-col cols="6">
            <v-switch
              v-model="cmdArgs.help"
              :label="t('wt_ext_cli.cmd_card.unpack_vromf.show_help_label')"
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
