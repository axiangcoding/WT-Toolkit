<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";
import { CmdResult } from "../../schema";
import { open } from "@tauri-apps/api/dialog";

const cmdOutput = ref<CmdResult>({} as any);

const cmdArgs = ref<{
  inputDir: string;
  outputDir: string;
  keepSuffix: boolean;
  help: boolean;
}>({} as any);

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
      <v-list-item-title>命令</v-list-item-title>
      <v-chip color="primary">wt-ext-cli unpack_dxp_and_grp</v-chip>
    </v-list-item>
    <v-list-item>
      <v-list-item-title>说明</v-list-item-title>
      将文件夹和子文件夹中的 DXP 和 GRP 文件解压为文本格式文件
    </v-list-item>
    <v-list-item>
      <v-list-item-title>参数</v-list-item-title>
      <v-container>
        <v-row dense>
          <v-col cols="6">
            <v-text-field
              label="输入目录。内含 DXP/GRP 文件的文件夹"
              append-inner-icon="mdi-folder"
              v-model="cmdArgs.inputDir"
              @click:append-inner="selectInputDir"
              clearable
            >
            </v-text-field>
          </v-col>
          <v-col cols="6">
            <v-text-field
              label="输出目录。创建的目标文件夹将包含新文件，并保留文件结构"
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
              label="最终 DXP/GRP 中的路径和名称后是否保留后缀"
              color="primary"
            ></v-switch>
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
    <v-btn color="info" @click="showOutputDir" :disabled="!cmdArgs.outputDir">
      打开输出目录
    </v-btn>
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
