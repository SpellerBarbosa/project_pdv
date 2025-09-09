<script setup lang="ts">
	import InputLabel from './InputLabel.vue';
	import Select from './Select.vue';
	import { ref } from 'vue';
	import img_user_add from '~/assets/img/user-add.png';
	import img_success from '~/assets/img/sucess.png';
	import Loading from '../common/Loading.vue';
	import { useIsLoggedStore } from '~/store/isLogged';

	const image = ref(img_user_add);
	const store = useIsLoggedStore();
    const name = ref<string>("");
    const user = ref<string>("");
    const password = ref<string>("");
    const role = ref<string>("");

    const submitRegister = async() =>{
        if(!name.value || !user.value || !password.value || !role.value){
            
        }
        
        try {
            
        } catch (error) {
            
        }
    }
</script>
<template>
	<section
		class="w-[500px] h-[650px] bg-[rgba(255,255,255,.9)] rounded-2xl flex flex-col items-center"
	>
		<h1
			class="w-full h-[20%] grid place-items-center text-2xl uppercase font-semibold font-mono"
		>
			Cadastrar acesso
		</h1>
		<form class="w-full h-[80%] flex flex-col items-center justify-evenly">
			<InputLabel
				labelName="nome"
				link="name"
                v-model="name"
			/>
			<InputLabel
				labelName="usuario"
				link="user"
                v-model="user"
			/>
			<InputLabel
				labelName="Senha"
				link="password"
                v-model="password"
			/>
			<Select selectName="Escolha um acesso" v-model="role"/>
			<button
				type="button"
				class="w-[180px] h-[40px] flex bg-blue-500 items-center rounded-lg shadow-xs shadow-gray-500 cursor-pointer"
                :disabled="store.isLoading"
			>
				<figure
					class="w-1/4 h-full border-r-1 border-white grid place-items-center"
				>
					<img
						:src="image"
						alt=""
						class="w-[40px] h-[40px]"
                        v-show="store.isLoading === false"
					/>
					<Loading v-show="store.isLoading" />
				</figure>
				<span
					class="w-3/4 h-full text-center uppercase text-xs tracking-[1px] font-semibold font-mono text-white grid place-items-center"
					>{{ store.isLoading === true ? 'Cadastrando':'Cadastrar' }}</span
				>
			</button>
		</form>
	</section>
</template>
