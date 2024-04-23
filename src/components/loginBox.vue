<template>
  <div id="login_box">
    <div class="head">
      <div class="close_b" title="关闭" @click="closeFn()"><img src="../assets/icon/close.png" alt/></div>
      <div class="login_title">Cookie登录</div>
    </div>
    <div class="login_content">
      <el-text class="cookie_des">请从网页版贴吧网络面板中获取</el-text>
      <el-input
          v-model="cookie"
          style="max-width: 400px"
          placeholder="输入贴吧Cookie"
      >
        <template #prepend>Cookie</template>
      </el-input>
      <el-button class="save_btn" type="primary" @click="get_or_set_cookie">保存</el-button>

    </div>
  </div>
</template>

<script setup>

import {ref,onMounted} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {ElNotification} from 'element-plus'

let props=defineProps({
  title: {
    type: String,
    default: "123"
  },
  closeFn: {
    type: Function
  }
})
let cookie = ref("")
const get_or_set_cookie = () => {
  invoke("get_or_set_cookie", {cookie: cookie.value}).then(res=>{
    if ("ok"===res){
      props.closeFn()
      window.location.reload()
    }else{
      cookie.value=res
    }
  }).catch(e=>{
    ElNotification({
      title: '失败',
      message: e,
      type: 'error'
    })
  })
}
onMounted(()=>{
  get_or_set_cookie()
});
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

  &:hover {
    box-shadow: 0 0 15px rgba(0, 0, 0, 0.6);
    transition: all 200ms;
  }

  .head {
    width: 100%;
    display: block;

    .login_title {
      margin: 0 auto;
      line-height: 30px;
      font-size: 15px;
      width: 100px;
      color: white;
    }

    .close_b {
      width: 40px;
      height: 100%;
      float: right;
      display: flex;

      align-items: center;
      justify-content: center;

      &:hover {
        background: red;
      }

      img {
        width: 25px;
        height: 25px;
        object-fit: cover;
      }
    }
  }

  .login_content {
    height: 100%;
    width: 100%;
    padding-top: 30px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-top: -30px;
    flex-direction: column;
    .cookie_des{
      font-size: 12px;
      height: 25px;
    }
  }

  .save_btn {
    margin-top: 20px;
    height: 25px;
    font-size: 14px;
  }

}
</style>