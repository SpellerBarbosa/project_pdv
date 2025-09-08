<script setup lang="ts">
	import type { PropType } from 'vue';

	const props = defineProps({
		btnType: {
			type: String as PropType<'button' | 'submit' | 'reset'>,
			default: 'button',
		},
		btnName: {
			type: String,
			default: '',
		},
		btnStyle:{
			type: String
		}
	});

	const emit = defineEmits<{
		(e: 'click', event: MouseEvent): void;
	}>();

	const emitValue = (event: MouseEvent) => {
		return emit('click', event);
	};
</script>
<template>
	<button
		:type="props.btnType"
		class='w-[120px] h-[40px] rounded-lg font-semibold uppercase tracking-wider bg-green-700 text-white cursor-pointer'
		:class="props.btnStyle"
		@click="emitValue"
	>
		{{ props.btnName }}
	</button>
</template>
