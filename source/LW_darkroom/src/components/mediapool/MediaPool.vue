<script setup>
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core"
import MediaCard from "./MediaCard.vue";
import FolderIcon from "../../assets/mp_folder.svg";
import { ref } from "vue";
const isLoading = ref(false)
const loadedCount = ref(0)
const assets = ref([]);

async function selectFolder(params) {
    const folder = await open({
        directory: true,
        multiple: false,
    });

    if (!folder) return;
    isLoading.value = true;
    loadedCount.value = 0;

    await invoke("scan_and_build", {
        path: folder,
    });

    const loadedAssets = await invoke("get_assets")
    assets.value = loadedAssets;
    loadedCount.value = loadedAssets.length;
    isLoading.value = false;
    console.log(loadedAssets)
}
</script>
<style scoped>
.mp_tray {
    width: 360px;
    height: 100%;
    display: flex;
    flex-direction: column;
}
.mp_pool {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
    height: calc(100vh - 100px); /* adjust for toolbar/header */
    gap: 12px;
    overflow-y: scroll;
    align-content: start;
}
.folder_icon {
    width: 24px;
    height: 24px;
    fill: #838383;
}
.mp_control {
    background-color: #111827;
}

.mp_control > button {
    background-color: #11182700;
    border: none;
    align-items: center;
}
span {
    font-size: xx-small;
}
.spinner {
    width: 12px;
    height: 12px;
    margin: 5px;
    border: 2px solid #83838333;
    border-top-color: #838383;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
}

@keyframes spin {
    to { transform: rotate(360deg); }
}
</style>
<template>
    <div class="mp_tray">
        <form class="row mp_control" @submit.prevent="selectFolder()">
            <button type="submit"><FolderIcon class="folder_icon"/></button>
            <div class="status_bar" v-if="isLoading || loadedCount > 0">
    <div v-if="isLoading" class="spinner"></div>

    <span v-else>
        Loaded {{ loadedCount }} images
    </span>
</div>
        </form>
        <div class="mp_pool" >
            <MediaCard v-for="asset in assets" :key="asset.id" :asset="asset.id" :name="asset.filename" :path="asset.path" :thumbnail="asset.thumbnail_path" />
        </div>
    </div>

</template>