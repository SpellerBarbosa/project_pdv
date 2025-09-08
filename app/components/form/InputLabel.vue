<script setup lang="ts">
	const props = defineProps<{
		link?: string;
		type?: string;
		place?: string;
		labelName?: string;
		styleLabel?: string;
		styleInput?: string;
		spanStyle?:string;
		modelValue?:string;
	}>();

	const type = props.type || 'text';

	const emit = defineEmits<{
		(e: 'update:modelValue',  value:string): void
	}>()

	const updateValue = (event: Event) =>{
		const input = event.target as HTMLInputElement;
		emit("update:modelValue", input.value);
	};
</script>
<template>
	<label
		:for="link"
		:class="['w-[90%] flex', styleLabel]"
	>
		<span
			:class="['w-[30%] grid place-items-center p-2 bg-green-500 text-white text-shadow-2xs text-shadow-black rounded-l-2xl uppercase font-semibold tracking-wider', spanStyle]"
			>{{ labelName }}</span
		>
		<input
			:type="type"
			:placeholder="place"
			:id="link"
			:class="[
				'w-[70%] outline-none border border-green-500 rounded-r-2xl placeholder:text-center placeholder:font-semibold placeholder:uppercase text-center tracking-wider',
				styleInput,
			]"
			:value="modelValue"
			@input="updateValue"
		/>
	</label>
</template>
