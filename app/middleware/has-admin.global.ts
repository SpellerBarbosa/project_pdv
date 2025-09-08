import { useHasAdminStore } from "~/store/has_admin"

export default defineNuxtRouteMiddleware(async ()=>{
    const store = useHasAdminStore()

    if(!store.has_admin){
        await store.get_admin()
    }
})