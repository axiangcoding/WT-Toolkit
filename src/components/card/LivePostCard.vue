<template>
  <div class="card">
    <div class="card-header">
      <div class="author-info">
        <img
          :src="props.data?.author?.avatar || ''"
          alt="Author Avatar"
          class="author-avatar"
        />
        <span class="author-nickname">{{
          props.data?.author?.nickname || ""
        }}</span>
      </div>
      <div class="post-info">
        <span class="card-time">{{
          DateTime.fromSeconds(parseInt(props.data?.created)).toRelative()
        }}</span>
        <span class="separator">•</span>
        <span class="card-type">{{ props.data?.type || "" }}</span>
      </div>
    </div>
    <div class="card-body">
      <p class="card-description" v-html="props.data?.description || ''"></p>
    </div>
    <div class="card-image">
      <a
        :href="
          'https://live.warthunder.com/post/' + props.data?.lang_group || '#'
        "
        target="_blank"
      >
        <img
          :src="
            props.data?.images?.[0]?.src || props.data?.video_info?.image || ''
          "
          loading="lazy"
          alt="Card Image"
          class="w-100 rounded"
        />
      </a>
    </div>
    <div class="card-footer">
      <a
        :href="
          'https://live.warthunder.com/post/' + props.data?.lang_group || '#'
        "
        class="card-link"
        target="_blank"
        >{{ t("wt_live.open_browser") }}</a
      >
      <a
        :href="props.data?.file?.link || '#'"
        class="card-link"
        target="_blank"
        :title="props.data?.file?.link || '???'"
        >{{ t("wt_live.download") }}</a
      >
    </div>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { DateTime } from "luxon";
const { t } = useI18n();

const props = defineProps<{
  data: any;
}>();
</script>

<style scoped>
.card {
  width: 300px;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 10px;
  margin: 1px;
  background-color: #fff;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  height: 100%;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
  padding-bottom: 10px;
  border-bottom: 1px solid #e0e0e0;
}

.author-info {
  display: flex;
  align-items: center;
}

.author-avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  margin-right: 10px;
}

.author-nickname {
  font-weight: bold;
}

.post-info {
  display: flex;
  align-items: center;
}

.card-time {
  font-size: 12px;
  color: #757575;
}

.separator {
  margin: 0 8px;
  color: #757575;
}

.card-type {
  color: #757575;
  font-size: 12px;
}

.card-body {
  padding-bottom: 16px;
  flex-grow: 1;
  border-bottom: 1px solid #e0e0e0;
}

.card-description {
  margin: 0;
  color: #757575;
}

.card-image {
  padding-top: 16px;
  padding-bottom: 16px;
  border-bottom: 1px solid #e0e0e0;
}

.card-footer {
  padding-top: 16px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.card-link {
  text-decoration: none;
  color: #1976d2;
  font-weight: bold;
}

.card-link:hover {
  text-decoration: underline;
}
</style>
