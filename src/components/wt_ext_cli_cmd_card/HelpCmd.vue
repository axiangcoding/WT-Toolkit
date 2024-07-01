<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";
import { CmdResult } from "../../schema";

const cmdOutput = ref<CmdResult>({} as any);

const args = ref(["help"]);

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
      <v-list-item-title>命令</v-list-item-title>
      <v-chip color="primary">wt-ext-cli --help</v-chip>
    </v-list-item>
    <v-list-item>
      <v-list-item-title>说明</v-list-item-title>
      显示 wt-ext-cli 的帮助信息
    </v-list-item>
    <v-list-item>
      <v-list-item-title> 参数 </v-list-item-title>
      无
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
