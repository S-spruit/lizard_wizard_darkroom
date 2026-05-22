<script setup>
const props = defineProps({
    asset: String,
    name: String,
    path: String,
    thumbnail: String
})
</script>
<style scoped>
    .mp_media_card {
        width: 100px;
        height: 100px;
        border: dotted 1px #838383;
        align-items: center;
    }
    .mp_media_title {
        font-size: xx-small;
        width: 90px;
        text-align: center;
    }
    .mp_media_image {
        height: 90px;
        width: 90px;
        font-size: xx-small;
    }
</style>
<template>
<div class="mp_media_card">
    <img :src="path" :alt="name" class="mp_media_image">
    <p class="mp_media_title">{{ name }}</p>
</div>
</template>