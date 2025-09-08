<script setup lang="ts">
import GenericButton from '../form/GenericButton.vue';
import { useRouter } from 'vue-router';

defineProps<{ x: number, y: number }>();
defineEmits(['dragstart'])
const config = useRuntimeConfig()
const router = useRouter();


const redirectDashboard = () =>{
    router.push("/dashboard")
}

const redirectLogin = () =>{
    router.push("/")
}

</script>
<template>
    <div v-if="config.public.nuxtEnv"
        class="w-[400px] h-[100px] bg-white flex flex-col items-center justify-evenly rounded-xl absolute cursor-move select-none shadow-2xl shadow-black"
        :style="{ top: y + 'px', left: x + 'px' }" @mousedown="$emit('dragstart')">
        <h1 class="p-1 uppercase font-semibold tracking-wider text-shadow-2xs text-black font-sans">Botões de navegação</h1>
        <section class="w-full flex items-center justify-center gap-2">
            <generic-button btn-name="Login" @click="redirectLogin" />
            <generic-button btn-name="dashboard" @click="redirectDashboard"/>
            <generic-button btn-name="cadastro administrador" btn-style="text-xs" />
        </section>
    </div>
</template>