<script setup>
import { convertFileSrc } from '@tauri-apps/api/core'
import { invoke } from '@tauri-apps/api/core'
const props = defineProps({
    asset: String,
    name: String,
    path: String,
    thumbnail: String
})
import { ref } from 'vue'

const rating = ref(0)
const ready = ref(false)
async function setRating(star) {
  rating.value = rating.value === star ? 0 : star
  console.log(rating.value)

  await invoke('update_asset_rating', {
    id: props.asset,
    rating: rating.value
  })
}

async function toggleReady() {
  ready.value = !ready.value

  await invoke('update_asset_ready', {
    id: props.asset,
    ready: ready.value
  })
}


</script>
<style scoped>
    .mp_media_card {
        display: flex;
        flex-direction: column;
        width: 100px;
        height: 150px;
        border: solid 1px #b07a43;
        border-radius: 6px;
        align-items: center;
        justify-content: space-between;
        margin: 5px;
    }
    .mp_media_title {
        font-size: xx-small;
        width: 100%;
        height: 38px;
        margin: 0px;
        border-top: 1px solid #b07a43;
        text-align: center;
    }
    .mp_media_image {
        height: 90px;
        width: 90px;
        font-size: xx-small;
        color: #a8adb3;
    }
    span {
        background-color: #b07a43;
        width: 20px;
        height: 45px;
        border-radius: 0px 6px 0px 3px;
        display: flex;
        flex-direction: column;
        align-items: center;
    }
    .mp_info {
        margin: 0px;
        width: 100%;
        display: flex;
        flex-direction: row;
        align-items: end;
        font-size: xx-small;
    }
    .mp_info_column {
        display: flex;
        flex-direction: column;
        width: 100%;
    }
    #assetReady:not(:disabled):checked {
    border: solid 1px #1a1b1e !important;
    background-color: #222428 !important;
    }
    .rating {
        display: flex;
        flex-direction: row;
        gap: 3px;
        border-left: #b07a43 solid 1px;
        border-top: #b07a43 solid 1px;
        border-radius: 7px 0px;
    }
    .star_button {
        background-color: transparent !important;
        appearance: none !important;
        border: none;
        width: 5px;
        color: #a8adb3;
    }
    .star_button.active {
        color: #e4e4e4;
    }
</style>
<template>
<div class="mp_media_card">
    <img :src="convertFileSrc(thumbnail)" :alt="name" class="mp_media_image">
    <div class="mp_info">
        <span>
            <label for="assetReady">R</label>
            <input type="checkbox" name="assetReady" id="assetReady" @change="toggleReady">
        </span>
        <div class="mp_info_column">
            <div class="rating">
                <button v-for="star in 5"
                 :key="star"
                 class="star_button"
                 :class="{ active: star <= rating }"
                 @click="setRating(star)">★</button>
            </div>
            <p class="mp_media_title">{{ name }}</p> 
        </div>
       
    </div>
</div>
</template>