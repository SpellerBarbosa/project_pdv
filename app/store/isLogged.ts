import { defineStore } from "pinia";

export const useIsLoggedStore = defineStore("isLogged", () => {
  const isLogged = ref<boolean>(false);
  const isLoading = ref<boolean>(false);

  return {
    isLogged,
    isLoading
  };
});
