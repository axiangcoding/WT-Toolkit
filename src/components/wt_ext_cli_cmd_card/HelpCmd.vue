<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";

const cmdOutput = ref("");

const args = ref(["--help"]);

async function exec() {
  cmdOutput.value = await invoke("exec_wt_ext_cli", { args: args.value });
  console.log(cmdOutput.value);
}
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
    <v-btn color="warning" @click="cmdOutput = ''"> 清空输出</v-btn>
  </div>

  <v-divider class="my-3"></v-divider>
  <v-code>{{ cmdOutput }}</v-code>
</template>

<style scoped>
.v-code {
  white-space: pre-line;
}
</style>
