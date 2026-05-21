<script setup>
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core"

async function selectFolder(params) {
    const folder = await open({
        directory: true,
        multiple: false,
    });

    if (!folder) return;

    const assets = await invoke("scan_and_build", {
        path: folder,
    });
    console.log(assets)
    
}
</script>
<template>
    <form class="row" @submit.prevent="selectFolder()">
      <button type="submit">select folder</button>
    </form>
</template>