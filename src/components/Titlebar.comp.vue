<script setup lang="ts">
import { ref } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { Icon } from "@iconify/vue";
import Button from './ui/button/Button.vue';

const appWindow = getCurrentWindow();
const handleWindowDrag = (e: MouseEvent) => {
    if (e.buttons === 1) {
        e.detail === 2 ? appWindow.toggleMaximize() : appWindow.startDragging();
    }
}
const minimizeWindow = () => appWindow.minimize();
const toggleFloatingWindow = () => {
    if (isFloating.value) {
        appWindow.maximize();
        isFloating.value = false;
    } else {
        appWindow.unmaximize();
        isFloating.value = true;
    }
}
const closeWindow = () => appWindow.close();
const isFloating = ref<boolean>(true);

</script>
<template>
    <nav class="fixed top-0 left-0 flex items-center justify-end w-full h-9 space-x-0.5">
        <div class="w-full h-full" @mousedown="handleWindowDrag"></div> 
        <Button variant="ghost" size="icon" class="cursor-pointer" @click="minimizeWindow">
            <Icon icon="proicons:subtract" /> 
        </Button>
        <Button variant="ghost" size="icon" class="cursor-pointer" @click="toggleFloatingWindow">
            <Icon icon="proicons:full-screen-maximize" v-if="isFloating" />
            <Icon icon="proicons:full-screen-minimize" v-else />
        </Button>
        <Button variant="ghost" size="icon" class="cursor-pointer" @click="closeWindow">
            <Icon icon="proicons:cancel" />
        </Button>
    </nav>
</template>
<style scoped>


</style>