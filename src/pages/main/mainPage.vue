<template>
    <div id="box">
        111


        <!--全屏组件-->
        <loading :visible="pageLoading" text="Check login"/>
    </div>
</template>

<script setup>
import {invoke} from "@tauri-apps/api/tauri";
import {onMounted, ref} from "vue";
import loading from "@/components/loading.vue";
import Notice from "@/components/js/notice.js";
import {useRouter} from "vue-router";

let router = useRouter();
const pageLoading = ref(false);
onMounted(async () => {
    pageLoading.value = true;
    await invoke("check_login").catch((err) => {
        Notice(err, "err");
        router.push("/login")
    })
    pageLoading.value = false;
})
</script>

<style scoped lang="scss">
#box {
    width: 100vw;
    height: 100vh;
    display: flex;
}
</style>