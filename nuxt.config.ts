// https://nuxt.com/docs/api/configuration/nuxt-config
import tailwindcss from "@tailwindcss/vite";

export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },
  css:['~/assets/css/main.css'],
  modules:['@pinia/nuxt'],
  runtimeConfig:{
    public:{
      nuxtEnv: process.env.NUXT_ENV || "production"
    }
  },
  vite:{
    plugins:[
      tailwindcss(),
    ],
  },
});