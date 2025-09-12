<script setup lang="ts">
defineProps<{
    message?:string
    view:boolean
}>()

</script>
<template>
    <section :class="['w-0 h-[70px] opacity-0  rounded-2xl absolute top-3 right-2 bg-green-800 border-2 border-white', view === true ? 'transform transition-all duration-700 ease-in':'flex w-[400px] opacity-100 transform transition-all duration-700 ease-in']">
        <figure :class="['w-1/4 h-full grid place-items-center rounded-l-2xl border-r-2 border-r-white', view === true ? 'transform transition-all delay-700 duration-500 opacity-0':'opacity-100 transform transition-all duration-500 delay-700']">
            <img src="~/assets/img/sucess.png" alt="imagem de check" class="w-[50px]">
        </figure>
        <span :class="['w-3/4 h-full text-white grid place-items-center font-semibold tracking-wider text-xs uppercase' , view === true ? 'transform transition-all delay-700 duration-500 opacity-0':'opacity-100 transform transition-all duration-500 delay-700']">{{ message }}</span>
    </section>
</template>