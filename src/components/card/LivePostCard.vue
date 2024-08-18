<template>
    <div class="card">

        <div class="card-body">
            <p class="card-description" v-html="props.data?.description || ''"></p>
        </div>

        <div class="card-image">
            <img :src="props.data?.images?.[0]?.src || props.data?.video_info?.image || ''" loading="lazy" alt="Card Image" class="w-100 rounded" />
        </div>

        <div class="card-footer">
            <a :href="props.data?.file?.link || '#'" class="card-link" target="_blank" :title="props.data?.file?.link || '???'">{{ t("wt_live.download") }}</a>
        </div>
    </div>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n";

const { t } = useI18n();
// take props
const props = defineProps<{
    data: any
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

.card-title {
    margin: 0;
    font-weight: bold;
}

.card-body {
    padding-bottom: 16px;
    flex-grow: 1; 
}

.card-description {
    margin: 0;
    color: #757575;
}

.card-footer {
    border-top: 1px solid #e0e0e0;
    padding-top: 8px;
    margin-top: 8px;
    text-align: right;
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
