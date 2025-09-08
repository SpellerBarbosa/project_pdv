<script setup lang="ts">
import Message from '~/components/register/Message.vue';
import Form from '~/components/register/Form.vue';
import { ref } from 'vue';

const confirmed =ref<boolean>(false);
const showMessage = ref<boolean>(true);

const handleConfirm =()=>{
    confirmed.value = true;
    showMessage.value = false;

}
const handleCancel = () =>{
    confirmed.value = false;
    showMessage.value = true;
}

</script>
<template>
    <main>
        <Message
        v-show="showMessage" 
        @confirm="handleConfirm"
        @close="handleCancel"
        />
        <Form  v-if="confirmed" />
    </main>
</template>