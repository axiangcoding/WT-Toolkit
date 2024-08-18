<template>
    <div class="m-4">
        <h1>{{ t("wt_live.title") }}</h1>

        <div v-if="loading">{{ t("wt_live.loading") }}</div>
        <div v-else-if="error">{{ error }}</div>
        <div v-else class="cards-grid">
            <LivePostCard v-for="item in data.list" :key="item.id" :data="item" />
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import LivePostCard from '../components/card/LivePostCard.vue';
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const data = ref<{ list: any[] }>({ list: [] });
const loading = ref<boolean>(true);
const error = ref<string | null>(null);

const fetchData = async (): Promise<void> => {
    try {
        // TODO: check if api supports url params or a json body
        // Also just clean up
        const formdata = new FormData();
        formdata.append('content', 'all');
        formdata.append('sort', 'created');
        formdata.append('user', '');
        formdata.append('period', '7');
        formdata.append('searchString', '');
        formdata.append('page', '0');
        formdata.append('featured', '0');
        formdata.append('subtype', 'all');
        const response = await fetch('https://live.warthunder.com/api/feed/get_regular/',
            {
                method: 'POST',
                body: formdata
            }
        );
        const result = await response.json();
        data.value = result.data;
        loading.value = false;
        console.log(data.value);
    } catch (err) {
        error.value = 'An error occurred while fetching data';
        loading.value = false;
    }
};

onMounted(fetchData);
</script>

<style scoped>
.cards-grid {
    display: flex;
    flex-wrap: wrap;
    gap: 16px;
}

.cards-grid>* {
    flex: 1 1 300px;
}
</style>
