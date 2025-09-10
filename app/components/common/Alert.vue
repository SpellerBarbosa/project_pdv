<script setup lang="ts">
import { ref } from 'vue';
defineProps<{
    alt?:string,
    image?:string,
    title?:string,
    message?:string,
    btnName?:string,
    btnStyle?:string
}>()


</script>
<template>
    <section class="w-screen h-screen bg-[rgba(85,85,85,.5)] absolute top-0 grid place-items-center"> 
        <article class="w-[500px] h-[300px] bg-white rounded-2xl flex flex-col items-center justify-evenly">
            <figure class="w-[70px]">
                <img :src="image" :alt="alt">
            </figure>
            <h1 class="first-letter:text-3xl text-2xl tracking-wider text-shadow-2xs font-semibold font-mono">{{ title }}</h1>
            <p class="w-[70%] text-center text-lg font-semibold text-shadow-2xs">{{ message }}</p>

            <button :class="['w-[120px] h-[40px] shadow shadow-gray-300 rounded-xl', btnStyle]">{{ btnName }}</button>
        </article>
    </section>
</template>