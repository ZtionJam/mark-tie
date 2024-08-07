<template>
    <div class="login_window">
        <div class="login_box">
            <div class="login_title">Login With Cookie</div>
            <m_input class="cookie_input" v-model="cookie" text="Cookie" placeholder="Please enter a valid cookie"/>
            <div class="cookie_des"><span>What is a cookie? </span>:Cookie is the login credential of Baidu Tieba, which
                can
                be obtained from the developer tool of the browser after login on the web
            </div>
            <m_btn @click="go_login" class="login_btn" text="Login"/>
        </div>
        <!--全屏组件-->
        <loading :visible="pageLoading" text="Authenticating"/>
    </div>
</template>

<script setup>
import m_input from "@/components/m_input.vue";
import m_btn from "@/components/m_btn.vue";
import loading from "@/components/loading.vue";
import {ref} from "vue";
import {useRouter} from "vue-router";

let router = useRouter();

const cookie = ref("");
const pageLoading = ref(false);

const go_login = () => {
    pageLoading.value = true;
    setTimeout(() => {
        pageLoading.value = false;
    }, 10000)
    // router.push("/main")
}
</script>

<style lang="scss">
.login_window {
    width: 100vw;
    height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;

    .login_box {
        width: 50%;
        height: 40%;
        background: rgba(255, 255, 255, 0.5);
        box-shadow: 5px 30px 25px rgba(0, 0, 0, 0.25);
        border-radius: 10px;
        transition: all 300ms;

        .login_title {
            width: 100%;
            text-align: center;
            font-weight: bold;
            line-height: 50px;
        }

        .cookie_input {
            margin-top: 20px;
        }

        .cookie_des {
            font-size: 12px;
            width: 85%;
            text-indent: 5px;
            margin: 10px auto 0;
            color: #292929;

            > span {
                font-weight: bold;
            }
        }

        .login_btn {
            margin-top: 30px;
        }


        &:hover {
            //transition: all 300ms;
            //width: 51%;
            //height: 41%;
        }
    }
}

</style>