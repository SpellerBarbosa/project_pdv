import { defineStore } from 'pinia';

export const useHasAdminStroe = defineStore('has_admin',()=>{
    const has_admin = ref<boolean>(false)


        async function get_admin(){
            try {
                const response = await fetch('http://localhost:8080/has-admin',{
                    method:"GET",
                    headers:{
                        "Content-type":"application/json"
                    }
                });
    
                const data: {message: boolean } = await response.json();
                has_admin.value = data.message
            } catch (error) {
                console.error("Erro ao buscar admin", error);
                has_admin.value = false
            }
        }

    return{
        has_admin,
        get_admin
    }
   
})