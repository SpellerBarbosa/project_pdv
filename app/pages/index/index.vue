<script setup lang="ts">
import Login from "~/components/login/Login.vue";
import AdminButtons from "~/components/common/AdminButtons.vue";

const position = ref({ x: 100, y: 100 });
const dragging = ref(false);

function onDragStart(){
    dragging.value = true
}

function onMouseMove(e: MouseEvent) {
    if(dragging.value){
        position.value.x = e.clientX
        position.value.y = e.clientY
    }
}

function onMouseUp(){
    dragging.value = false
}
</script>

<template>
  <main
    class="w-screen h-screen bg-[url('~/assets/img/background.png')] bg-cover grid place-items-center"
    @mousemove="onMouseMove"
    @mouseup="onMouseUp"
  >
    <Login />
  </main>
</template>
