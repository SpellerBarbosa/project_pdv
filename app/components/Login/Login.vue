<script setup lang="ts">
import InputLabel from '../form/InputLabel.vue';
import GenericButton from '../form/GenericButton.vue';
import { message } from '@tauri-apps/plugin-dialog';
import { ref } from 'vue';

const user = ref<string>("");
const password = ref<string>("");

async function handleSubmit(){
    if(!user.value || !password.value){
        await message('Preencha os campos corretatmente',{title:"Campos em branco", kind: "error"})
    }
}
</script>   

<template>
    <form class="w-[400px] h-[250px] bg-[rgba(255,255,255,0.96)] shadow-2xl shadow-black rounded-2xl flex flex-col items-center justify-center gap-6">
        <InputLabel 
        labelName="Usuario"
        place="Digite seu usuario"
        v-model="user"
        />
        <InputLabel
        labelName="Senha"
        place="*******"
        v-model="password"
        />
        <GenericButton 
        btnName="Entrar"
        @click="handleSubmit"
        />
    </form>
</template>