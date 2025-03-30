<script lang="ts" setup>
import {openUrl} from "@tauri-apps/plugin-opener"
import {Icon} from "@iconify/vue"
import {ref} from "vue";
import { useRouter } from "vue-router";
import { Button } from "@/components/ui/button";

import StarshotIconSVG from "../static/icons/starshot/Starshot.svg"

const currentPage = ref(0);
const incrementPage = () => currentPage.value += 1;
const decrementPage = () => currentPage.value -= 1;
const router = useRouter();

const skipNonsense = () => {
  router.replace("/configure")
}

</script>
<template>
    <Transition name="slide-fade">
        <div class="w-screen h-screen pt-9"> 
            <div class="flex flex-col items-center justify-center h-full" v-if="currentPage === 0">
              <img :src="StarshotIconSVG" class="w-auto h-48 aspect-square" draggable="false" />
                <h1 class="text-3xl font-bold">Welcome to StarshotOCR</h1>
                <p class="mb-2">An open-source, powerful, and intelligent OCR application powered by Tesseract.</p>
                <span class="inline-flex mb-8 space-x-1 text-2xl">
                    <button class="cursor-pointer" @click="async () => await openUrl('https://github.com/rainwashed/StarshotOCR.git')"><Icon icon="proicons:github" /></button> 
                    <button class="cursor-pointer" @click="async () => await openUrl('https://github.com/tesseract-ocr/tesseract.git')"><Icon icon="proicons:google-2" /></button>
                </span>

                <Button class="mb-2 cursor-pointer" @click="incrementPage">Get Started</Button>  
                <p class="text-xs">— or —</p>
                <Button class="text-xs font-light" variant="ghost" @click="skipNonsense">skip this nonsense</Button> 
                <!-- <button class="py-1 px-2 rounded-sm text-xs cursor-pointer hover:border-[rgba(255,255,255,1)] transition-[border-color]" @click="skipNonsense">skip this nonsense</button>   -->
            </div>
            <div v-if="currentPage === 1">
                <h1>Select your page</h1>
            </div>
        </div>
    </Transition> 
</template>
<style lang="css">
.slide-fade-enter-active {
  transition: all 0.3s ease-out;
}

.slide-fade-leave-active {
  transition: all 0.8s cubic-bezier(1, 0.5, 0.8, 1);
}

.slide-fade-enter-from,
.slide-fade-leave-to {
  transform: translateX(20px);
  opacity: 0;
}
</style>