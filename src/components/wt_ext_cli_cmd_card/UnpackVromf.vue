<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";
import { CmdResult } from "../../schema";
import { open } from "@tauri-apps/api/dialog";

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
      <v-list-item-title>命令</v-list-item-title>
      <v-chip color="primary">wt-ext-cli unpack_vromf</v-chip>
    </v-list-item>
    <v-list-item>
      <v-list-item-title>说明</v-list-item-title>
      将 vromf 解压为原始格式或人类可读格式，如 Json 或 Blk
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
            <v-text-field
              label="输出目录。将创建的包含新文件的目标文件夹"
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
              label="输出格式。可以是[Json、BlkText、Raw] 默认：Json"
              :items="['Json', 'BlkText', 'Raw']"
              v-model="cmdArgs.format"
            ></v-select>
          </v-col>
          <v-col cols="6">
            <v-select
              clearable
              label="将所有 avif 图像转换为 png 格式。可以是 [imagemagick, ffmpeg] 默认：imagemagick"
              :items="['imagemagick', 'ffmpeg']"
              v-model="cmdArgs.avif2png"
            ></v-select>
          </v-col>
          <v-col cols="6">
            <v-text-field
              label="blk扩展名。如果提供，则将所有 blk 文件的扩展名替换为该扩展名，否则保持不变"
              v-model="cmdArgs.blkExtension"
              clearable
            >
            </v-text-field>
          </v-col>
          <v-col cols="6">
            <v-switch
              v-model="cmdArgs.override"
              label="在每个 json 中应用 `override:` 字段"
              color="primary"
            ></v-switch>
          </v-col>
          <v-col cols="6">
            <v-switch
              v-model="cmdArgs.crlf"
              label="返回带有 \r\n 而不是 \n 换行符的文件"
              color="primary"
            ></v-switch>
          </v-col>
          <v-col cols="6">
            <v-switch
              v-model="cmdArgs.zip"
              label="将输出文件夹压缩为 zip"
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
