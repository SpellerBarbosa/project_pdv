<script setup lang="ts">
	defineProps<{
		selectName?: string;
	}>();

	const emit = defineEmits<{
		(e: 'update:value', value: string): void;
	}>();

	const updateValue = (event: Event) => {
		const select = event.target as HTMLSelectElement;
		emit('update:value', select.value);
	};
</script>
<template>
	<label for="role">
		<select
			name="role"
			id="role"
			@change="updateValue"
            required
		>
			<option value="admin">administrador</option>
			<option value="operator">operador</option>
			<option value="stock">Estoquista</option>
		</select>
		<span>{{ selectName }}</span>
	</label>
</template>
<style scoped>
label select{
    position: relative;
}

label span{
    position: absolute;
}
</style>