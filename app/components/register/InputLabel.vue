<script setup lang="ts">
	const props = defineProps<{
		styleLabel?: string;
		styleSpan?: string;
		styleInput?: string;
		labelName?: string;
		link?: string;
		place?: string;
		inputType?: string;
		modelValue?: string;
	}>();

	const emit = defineEmits<{
		(e: 'updateValue', value: string): void;
	}>();

	const updateValue = (event: Event) => {
		const input = event.target as HTMLInputElement;
		emit('updateValue', input.value);
	};

	const type = props.inputType || 'text';
</script>
<template>
	<label
		:for="link"
		:class="['w-[80%] h-[35px]', styleLabel]"
	>
		<input
			:type="type"
			:id="link"
			:class="['outline-none border-b-1 border-b-blue-700 w-full h-full', styleInput]"
			:value="modelValue"
			@input="updateValue"
			:placeholder="props.place || ''"
		/>
		<span :class="['', styleSpan]">{{ labelName }}</span>
	</label>
</template>
<style scoped>
label{
	position: relative;
}

label span{
	position: absolute;
	top: 50%;
	transform: translateY(-50%);
	left: 0;
	transition: 1s ease all;
	padding: 0 10px;
	color: rgb(71, 71, 199);
	text-transform: capitalize;
	font-size: 14px;
	letter-spacing: 1px;
}

label input:focus + span,
label input:not(:placeholder-shown) + span{
	top: -12px;
	left: 0px;
}

</style>
