<script setup lang="ts">
import { ref } from "vue";
import { useIsLoggedStore } from "~/store/isLogged";
defineProps<{
  selectName?: string;
}>();

const emit = defineEmits<{
  (e: "update:value", value: string): void;
}>();

const updateValue = (event: Event) => {
  const select = event.target as HTMLSelectElement;
  emit("update:value", select.value);
};

const store = useIsLoggedStore();

</script>
<template>
  <label for="role">
    <select name="role" id="role" @change="updateValue" required>
      <option value="" disabled selected></option>
      <option value="admin">administrador</option>
      <option value="operator" :disabled="store.isLogged === false">operador</option>
      <option value="stock" :disabled="store.isLogged === false">Estoquista</option>
    </select>
    <span>{{ selectName }}</span>
  </label>
</template>
<style scoped>
label {
  position: relative;
  width: 50%;
  border: solid 1px rgb(30, 30, 185);
}

label select {
  width: 100%;
  padding: 12px 10px;
  appearance: none;
  outline: none;
  font-weight: 700;
  text-transform: capitalize;
  letter-spacing: 1px;
}

label span {
  position: absolute;
  top: 12px;
  left: 10px;
  pointer-events: none;
  transition: all 0.2s ease;
  font-size: 18px;
  text-align: center;
  font-weight: 500;
  letter-spacing: 1px;
  color: blue;
}

label select:focus + span,
label select:valid + span {
  top: -15px;
  left: 8px;
  padding: 0 4px;
  background: #fff;
}
</style>
