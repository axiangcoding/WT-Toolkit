<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";
import { CmdResult } from "../../schema";
import { open } from "@tauri-apps/api/dialog";

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
      <v-list-item-title>命令</v-list-item-title>
      <v-chip color="primary">wt-ext-cli vromf_version</v-chip>
    </v-list-item>
    <v-list-item>
      <v-list-item-title>说明</v-list-item-title>
      打印 vromfs 文件或文件夹中的版本
    </v-list-item>
    <v-list-item>
      <v-list-item-title>参数</v-list-item-title>
      <v-container>
        <v-row dense>
          <v-col cols="6">
            <v-text-field
              label="输入目录或者文件。不会递归文件夹"
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
              label="展示帮助信息"
              color="primary"
            ></v-switch>
          </v-col>
        </v-row>
      </v-container>
    </v-list-item>
  </v-list>
  <div class="d-flex ga-2">
    <v-btn color="primary" @click="exec">执行命令</v-btn>
    <v-btn color="warning" @click="cleanOutput"> 清空输出</v-btn>
  </div>

  <v-divider class="my-3"></v-divider>

  <v-list>
    <v-list-item>
      <v-list-item-title>执行结果</v-list-item-title>
      <v-chip
        variant="elevated"
        color="success"
        v-if="cmdOutput.code != null && cmdOutput.code == 0"
      >
        执行成功
      </v-chip>
      <v-chip
        variant="elevated"
        color="error"
        v-if="cmdOutput.code != null && cmdOutput.code != 0"
        >执行失败
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
      <v-list-item-title>内容输出</v-list-item-title>
      <v-code class="console-box border-success">
        {{ cmdOutput.stdout ? cmdOutput.stdout : cmdOutput.stderr }}
      </v-code>
    </v-list-item>
  </v-list>
</template>

<style scoped></style>
