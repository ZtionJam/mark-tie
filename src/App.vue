<template>
  <div class="container">
    <div class="head" v-if="showHead">
      <div class="title" data-tauri-drag-region>马克贴</div>
      <div class="btn_box">
        <div class="user">
          <el-dropdown trigger="click">
            <el-avatar
                title="用户"
                class="avatar_box"
                size="small"
                src="https://cube.elemecdn.com/0/88/03b0d39583f48206768a7534e55bcpng.png"
            />
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item @click="toggle_login">账号</el-dropdown-item>
                <el-dropdown-item>设置</el-dropdown-item>
                <el-dropdown-item>关于</el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>
        <div @click="appWindow.minimize()" class="min">
          <img src="./assets/icon/min.png" alt/>
        </div>
        <div @click="appWindow.close()" class="close">
          <img src="./assets/icon/close.png" alt/>
        </div>
      </div>
    </div>
    <router-view/>
    <div class="content_mask" v-if="showMask" @click="toggle_mask"></div>
    <loginBox v-if="showLogin" :closeFn="toggle_login"/>
  </div>
</template>

<script setup>
import {ref} from "vue";
import {appWindow} from "@tauri-apps/api/window";
import loginBox from "./components/loginBox.vue";
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
.btn_box {
  height: 100%;
  width: 18%;
  display: flex;
  align-items: center;
  border-top-right-radius: 10px;

  > div {
    height: 100%;
    width: 33.33%;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 200ms;
  }

  .avatar_box {
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
  width: 82%;
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
