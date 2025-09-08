<script setup lang="ts">
	import InputLabel from '../form/InputLabel.vue';
	import GenericButton from '../form/GenericButton.vue';
	import { message } from '@tauri-apps/plugin-dialog';
	import { ref } from 'vue';
	import { useHasAdminStore } from '~/store/has_admin';
	import { useRouter } from 'vue-router';

	const user = ref<string>('');
	const password = ref<string>('');
	const has_admin = useHasAdminStore();
	const router = useRouter()

	async function handleSubmit() {
		if (!user.value || !password.value) {
			await message('Preencha os campos corretatmente', {
				title: 'Campos em branco',
				kind: 'error',
			});
		}
	}

	const goToRegister = () =>{
		router.push("/register")
	}
</script>

<template>
	<form
		class="w-[400px] h-[250px] bg-[rgba(255,255,255,0.96)] shadow-2xl shadow-black rounded-2xl flex flex-col items-center justify-center gap-6"
	>
		<InputLabel
			labelName="Usuario"
			place="Digite seu usuario"
			v-model="user"
		/>
		<InputLabel
			labelName="Senha"
			place="*******"
			v-model="password"
			spanLabel="text-white"
		/>
		<section class="w-[80%] h-[20%] flex items-center justify-center gap-3">
			<GenericButton
				btnName="Entrar"
				@click="handleSubmit"
			/>
			<GenericButton
				v-if="has_admin.has_admin === false || has_admin.has_admin === undefined" 
                btnName="Cadastrar Administrador"
                btnStyle="text-xs"
				@click="goToRegister" 
            />
		</section>
	</form>
</template>
