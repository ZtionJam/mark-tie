<template>
  <div class="container">
    <div class="head" v-if="showHead">
      <div class="title" data-tauri-drag-region>马克贴</div>
      <div class="user">
        <el-dropdown trigger="click">
          <div>
            <el-text class="username">{{ userinfo.user_name_show }}</el-text>
            <el-avatar
                :title="userinfo.user_name_show"
                class="avatar_box"
                size="small"
                :src="'https://gss0.bdstatic.com/6LZ1dD3d1sgCo2Kml5_Y_D3/sys/portrait/item/'+userinfo.user_portrait"
            />
          </div>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item @click="toggle_login">账号</el-dropdown-item>
              <el-dropdown-item>设置</el-dropdown-item>
              <el-dropdown-item>关于</el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </div>
      <div class="btn_box">
        <div @click="appWindow.minimize()" class="min">
          <img src="./assets/icon/min.png" alt/>
        </div>
        <div @click="appWindow.close()" class="close">
          <img src="./assets/icon/close.png" alt/>
        </div>
      </div>
    </div>
    <router-view v-slot="{ Component,route }">
      <keep-alive :include="getKeepPage()">
        <component :is="Component"/>
      </keep-alive>
    </router-view>
    <div class="content_mask" v-if="showMask" @click="toggle_mask"></div>
    <loginBox v-if="showLogin" :closeFn="toggle_login"/>
  </div>
</template>

<script setup>
import {onMounted, ref} from "vue";
import {appWindow} from "@tauri-apps/api/window";
import loginBox from "./components/loginBox.vue";
import {invoke} from "@tauri-apps/api/tauri";
import {ElNotification} from "element-plus";
import {useRouter} from "vue-router";

const router = useRouter();
const getKeepPage = () => {
  return router.getRoutes().filter(route => route.meta.keepAlive).map(route => route.name);
}

let userinfo = ref({
  user_name_show: "未登录",
  user_portrait: "",
  open_uid: ""
});
onMounted(() => {
  invoke("get_user_info").then(u => {
    userinfo.value = u;
    ElNotification({
      type: "success",
      title: "登录成功",
      message: "你好！" + u.user_name_show + "，欢迎回来~",
      position: "bottom-right"
    });
  });
});
//切换登录框状态
const toggle_login = () => {
  showMask.value = !showMask.value;
  showLogin.value = !showLogin.value;
};
//切换遮罩状态
const toggle_mask = () => {
  showMask.value = !showMask.value;
  showLogin.value = false;
};

let showHead = ref(true);
let showMask = ref(false);
let showLogin = ref(false);
</script>

<style lang="scss">
.user {
  width: 10%;
  height: 100%;
  transition: all 200ms;

  > div > div {
    display: flex !important;
    align-items: center;
    justify-content: space-around !important;
    width: 100%;
  }

  &:hover {
    cursor: pointer;
    // background: #ccc;
    .username {
      color: black;
    }
  }

  .username {
    line-height: 30px;
    font-size: 12px;
    color: white;
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
    width: 50px;
  }

  .avatar_55x {
    width: 24px;
  }
}

.btn_box {
  height: 100%;
  width: 12%;
  display: flex;
  align-items: center;
  border-top-right-radius: 10px;

  > div {
    height: 100%;
    width: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 200ms;
  }

  .close:hover {
    background: red;
    transition: all 200ms;
  }

  > div:hover {
    background: #ccc;
    transition: all 200ms;
  }

  img {
    width: 45%;
  }
}

.title {
  height: 100%;
  width: 78%;
  text-indent: 54%;
  line-height: 30px;
  font-size: 14px;
  color: white;
  font-weight: bold;
}

.container {
  overflow: hidden;
  background-color: #dceffe;
  width: 98vw;
  height: 98vh;
  box-shadow: 0 0 8px rgba(0, 0, 0, 0.6);
  margin: 1vh auto 0;
  border-radius: 10px;

  .content_mask {
    position: fixed;
    top: 0;
    left: 1%;
    overflow: hidden;
    width: 98%;
    height: 95vh;
    margin: 4vh auto 0;
    border-radius: 10px;
    background: rgba($color: #cccccc, $alpha: 0.5);
    transition: all 200ms;
  }
}

.head {
  height: 30px;
  width: 98vw;
  position: fixed;
  background-color: #3388ff;
  z-index: 999999;
  border-top-left-radius: 10px;
  border-top-right-radius: 10px;
  display: flex;
  align-items: center;
  overflow: hidden;
}

body {
  -webkit-touch-callout: none;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
  cursor: default;
}

#app {
  width: 100vw;
  height: 100vh;
  margin: 0 auto;
  position: fixed;
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  overflow-x: hidden;
  overflow-y: auto;
  user-select: none;
}

#app::-webkit-scrollbar {
  display: none;
}

img {
  -webkit-user-drag: none;
}
</style>
