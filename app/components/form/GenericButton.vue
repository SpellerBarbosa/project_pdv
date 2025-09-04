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
		btnStyle: String,
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
		:class="['w-[120px] h-[40px] bg-green-700 rounded-lg font-semibold uppercase tracking-wider text-white cursor-pointer', props.btnStyle]"
		@click="emitValue"
	>
		{{ props.btnName }}
	</button>
</template>
