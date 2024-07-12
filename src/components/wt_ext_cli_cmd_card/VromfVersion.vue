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
  help: boolean;
}>({} as any);

async function exec() {
  let args = ["vromf_version"];
  if (cmdArgs.value.inputDirOrFile) {
    args.push("--input_dir_or_file", cmdArgs.value.inputDirOrFile);
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

const loading = ref(false);
</script>

<template>
  <v-list>
    <v-list-item>
      <v-list-item-title>
        {{ t("wt_ext_cli.cmd_card.label.command") }}
      </v-list-item-title>
      <v-chip color="primary">wt-ext-cli vromf_version</v-chip>
    </v-list-item>
    <v-list-item>
      <v-list-item-title>
        {{ t("wt_ext_cli.cmd_card.label.description") }}
      </v-list-item-title>
      {{ t("wt_ext_cli.cmd_card.vromf_version.description") }}
    </v-list-item>
    <v-list-item>
      <v-list-item-title>
        {{ t("wt_ext_cli.cmd_card.label.args") }}
      </v-list-item-title>
      <v-container>
        <v-row dense>
          <v-col cols="6">
            <v-text-field
              :label="t('wt_ext_cli.cmd_card.vromf_version.input_file_label')"
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
            <v-switch
              v-model="cmdArgs.help"
              :label="t('wt_ext_cli.cmd_card.vromf_version.show_help_label')"
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
