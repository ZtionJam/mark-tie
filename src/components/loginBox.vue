<template>
  <div id="login_box" :key="key">
    <div class="head"><div class="close_b" title="关闭" @click="closeFn()"><img src="../assets/icon/close.png" alt/></div> <div class="login_title">Cookie登录</div></div>
    <div></div>
  </div>
</template>

<script>
import {createVNode, ref, render} from "vue";
import loginBox from "./loginBox.vue";
import { type } from "@tauri-apps/api/os";

export default {
  props: {
    title: {
      type: String,
      default: ""
    },
    closeFn:{
      type:Function
    }
  },
  setup(props) {
    let key = ref(Math.floor(Math.random() * 10000));
    return {key}
  }
};

export function showLoginBox(title) {
  const vnode = createVNode(loginBox, {title});
  render(vnode, document.body);
}
</script>

<style scoped lang="scss">
#login_box {
  width: 60vw;
  height: 30vw;
  background-color: #dceffe;
  box-shadow: 0 0 8px rgba(0, 0, 0, 0.6);
  border-radius: 10px;
  z-index: 999;
  position: fixed;
  left: 50%;
  top: 50%;
  overflow: hidden;
  transform: translate(-50%, -50%);

  &:hover{
    box-shadow: 0 0 15px rgba(0, 0, 0, 0.6);
    transition: all 200ms;
  }

  .head{
    width: 100%;
    display: block;

    .login_title{
      margin: 0 auto;
      line-height: 30px;
      font-size: 15px;
      width: 100px;
      color: white;
    }

    .close_b{
      width: 40px;
      height: 100%;
      float: right;
      display: flex;
      
      align-items: center;
      justify-content: center;

      &:hover{
        background: red;
      }

      img{
        width: 25px;
        height: 25px;
        object-fit: cover;
      }
    }
  }

}
</style>