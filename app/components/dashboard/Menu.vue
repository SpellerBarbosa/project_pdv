<script setup lang="ts">
import { ref } from 'vue';

const openMenu = ref<boolean>(false)

function viewMenu(){
    openMenu.value = !openMenu.value
}
</script>

<template>
    <nav class="w-screen h-[5vh] bg-gray-100 fixed top-0 flex gap-1 items-center">
        <li class="item-menu">Inicio</li>
        <li class="item-menu flex flex-col relative" @click="viewMenu">
            <span>Cadastro</span>
            <ul v-if="openMenu" class="w-full h-max absolute top-[5vh] text-xs bg-gray-50">
                <li class="item-submenu">Funcionarios</li>
                <li class="item-submenu">Produtos</li>
            </ul>
        </li>
    </nav>
</template>