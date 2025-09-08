<script setup lang="ts">
import AdminButtons from "~/components/common/AdminButtons.vue";

const position = ref({ x: 100, y: 100 });
const dragging = ref(false);

function onDragStart() {
  dragging.value = true
}

function onMouseMove(e: MouseEvent) {
  if (dragging.value) {
    position.value.x = e.clientX
    position.value.y = e.clientY
  }
}

function onMouseUp() {
  dragging.value = false
}
</script>

<template>
  <div class="w-screen h-screen bg-[url('~/assets/img/background.png')] bg-cover grid place-items-center"
    @mousemove="onMouseMove"
    @mouseup="onMouseUp">
    <AdminButtons :x="position.x" :y="position.y" @dragstart="onDragStart" />
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </div>
</template>
<style scoped>
*{
  margin: 0;
  padding: 0;
  list-style: none;
  font-family: sans-serif;
}
</style>
