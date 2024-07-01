<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";
import { CmdResult } from "../../schema";
import { open } from "@tauri-apps/api/dialog";

const cmdOutput = ref<CmdResult>({} as any);

const cmdArgs = ref<{
  inputDir: string;
  outputDir: string;
  format: string;
  help: boolean;
}>({} as any);

async function exec() {
  let args = ["unpack_raw_blk"];
  if (cmdArgs.value.inputDir) {
    args.push("--input_dir", cmdArgs.value.inputDir);
  }
  if (cmdArgs.value.outputDir) {
    args.push("--output_dir", cmdArgs.value.outputDir);
  }
  if (cmdArgs.value.format) {
    args.push("--format", cmdArgs.value.format);
  }
  if (cmdArgs.value.help) {
    args.push("--help");
  }

  cmdOutput.value = await invoke("exec_wt_ext_cli", { args: args });
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

async function selectFile() {
  let selectedPath = await open({
    directory: false,
    multiple: false,
  });
  return selectedPath;
}

async function selectInputDir() {
  let selectedPath = await selectFile();
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
</script>

<template>
  <v-list>
    <v-list-item>
      <v-list-item-title>命令</v-list-item-title>
      <v-chip color="primary">wt-ext-cli unpack_raw_blk</v-chip>
    </v-list-item>
    <v-list-item>
      <v-list-item-title>说明</v-list-item-title>
      解压一个文件夹中的原始/二进制 blk 文件为解包格式
    </v-list-item>
    <v-list-item>
      <v-list-item-title> 参数 </v-list-item-title>
      <v-container>
        <v-row dense>
          <v-col cols="6">
            <v-text-field
              label="需要解包的二进制 blk 文件"
              append-inner-icon="mdi-file"
              v-model="cmdArgs.inputDir"
              @click:append-inner="selectInputDir"
              readonly
              clearable
            >
            </v-text-field>
          </v-col>
          <v-col cols="6">
            <v-text-field
              label="输出目录。将创建的包含新文件的目标文件夹"
              append-inner-icon="mdi-folder"
              v-model="cmdArgs.outputDir"
              @click:append-inner="selectOutputDir"
              readonly
              clearable
            >
            </v-text-field>
          </v-col>
          <v-col cols="6">
            <v-select
              clearable
              label="输出格式，可以是[Json、BlkText] 默认是Json"
              :items="['Json', 'BlkText']"
              v-model="cmdArgs.format"
            ></v-select>
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
