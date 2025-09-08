<script setup lang="ts">
import GenericButton from '../form/GenericButton.vue';
import { useRouter } from 'vue-router';

const router = useRouter();

const goToLogin = () =>{
    router.push("/")
}

</script>

<template>
    <section class="w-[450px] h-[300px] flex flex-col items-center justify-evenly bg-gray-100 rounded-2xl shadow-xs shadow-black relative">
        <h1 class="w-full h-[30px] flex items-center justify-center uppercase font-semibold tracking-wider rounded-tl-2xl rounded-tr-2xl bg-gray-200 absolute top-0">
            Atenção
            <div class="w-[20px] h-[20px] bg-red-500 flex flex-col items-center justify-center absolute right-4 cursor-pointer" @click="goToLogin">
                <span class="w-[90%] block h-[2px] bg-white origin-center rotate-45 translate-y-[1px]"></span>
                <span class="w-[90%] block h-[2px] bg-white origin-center -rotate-45 -translate-y-[1px]"></span>
            </div>
        </h1>
        <p class="w-[80%] mt-[30px] text-justify font-semibold">Essa tela so aprecererá uma vez para o cadastrdo do primeiro administrador, 
           coloque um id e senha  facil de lembrar, assim que  o administrador for cadastrado,
           a tela de registro so ficara disponivel dentro do sistema.
        </p>
        <div>
            <GenericButton 
            btn-name="ok"
            btn-style="text-xs hover:bg-blue-600 cursor-pointer"
            />
        </div>
    </section>
</template>