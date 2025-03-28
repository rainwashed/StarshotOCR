<script setup lang="ts">
import { onMounted } from "vue";
import {RouterView} from "vue-router";
import TitlebarComp from "./components/Titlebar.comp.vue";

onMounted(() => {
  document.addEventListener("contextmenu", e => e.preventDefault());
})

</script>
<template>
  <TitlebarComp />  
  <RouterView />
</template>