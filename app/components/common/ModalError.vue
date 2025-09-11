<script setup lang="ts">
defineProps<{
  message?: string;
}>();

const emit = defineEmits<{
  (e: "click", event: MouseEvent): void;
}>();
</script>
<template>
  <section
    class="w-screen h-screen bg-[rgba(85,85,85,.6)] absolute top-0 grid place-items-center"
  >
    <article class="w-[350px] h-[250px] bg-white rounded-2xl border-2">
      <div
        class="w-full h-[40px] bg-red-500 rounded-tl-xl rounded-tr-xl relative border-b-2"
      >
        <h1
          class="text-2xl absolute left-7 top-[50%] -translate-y-[50%] font-semibold tracking-[1px] text-shadow-2xs"
        >
          Error
        </h1>
        <div
          class="w-[35px] h-[35px] absolute right-3 top-[50%] -translate-y-[50%] flex flex-col items-center justify-evenly cursor-pointer"
          @click="emit('click', $event)"
        >
          <span
            class="w-[95%] h-[2px] bg-black block origin-center rotate-45 translate-y-[8px]"
          ></span>
          <span
            class="w-[95%] h-[2px] bg-black block origin-center -rotate-45 -translate-y-[5px]"
          ></span>
        </div>
      </div>
      <section class="w-full h-[calc(250px-40px)] flex flex-col items-center justify-center gap-5">
        <p class="w-[80%] text-center font-semibold tracking-wider">{{ message }}</p>
        <button
          type="button"
          class="w-[120px] h-[40px] border-2 border-black text-shadow-sm tracking-[1px] text-shadow-black rounded-md bg-red-400 font-semibold text-white cursor-pointer"
          @click="emit('click', $event)"
        >
          OK
        </button>
      </section>
    </article>
  </section>
</template>
