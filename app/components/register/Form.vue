<script setup lang="ts">
	import GenericButton from '../form/GenericButton.vue';
	import InputLabel from './InputLabel.vue';
    import Select from './Select.vue';
</script>
<template>
	<section class="w-[500px] h-[650px] bg-[rgba(255,255,255,.9)] rounded-2xl flex flex-col items-center">
		<h1 class="text-2xl uppercase font-semibold font-mono">Cadastrar acesso</h1>
		<form class="w-full h-[80%] flex flex-col items-center justify-evenly">
            <InputLabel
            labelName="nome"
            link="name"
            />
            <InputLabel
            labelName="usuario"
            link="user"
            />
            <InputLabel 
            labelName="Senha"
            link="password"
            />
            <Select 
            selectName="Escolha um acesso"
            />
        </form>
	</section>
</template>
