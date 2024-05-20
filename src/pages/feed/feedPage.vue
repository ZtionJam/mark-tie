<template>
  <div id="feed_box" v-loading="data.page_loading">
    <div class="content_box">
      <div class="feed_box"></div>
      <div class="comment_box"></div>
    </div>


    <div class="fixed_btn">
      <div @click="back"><img src="@/assets/icon/back.png" alt=""></div>
      <div><img src="@/assets/icon/top.png" alt=""></div>
    </div>
  </div>
</template>

<script setup>
import {useRouter} from "vue-router";
import {onMounted, ref} from "vue";

const router = useRouter();
let data = ref({
  page_loading: true
});

onMounted(() => {
  data.value.page_loading = false;
});

const back = () => {
  router.push("/main");
};
</script>

<style scoped lang="scss">
#feed_box {
  padding-top: 30px;
  width: 100%;
  height: 100%;

  .content_box {
    width: 100%;
    height: 100%;
    border: 1px solid red;

    .feed_box{
      width: 100%;
      min-height: 200px;
      border: 1px solid green;


    }
  }

  .fixed_btn {
    width: 40px;
    height: 80px;
    position: fixed;
    top: 75%;
    left: 90%;

    > div {
      width: 100%;
      height: 50%;
      display: flex;
      align-items: center;
      justify-content: center;
      border-radius: 5px;
      background: white;
      margin-top: 10px;
      opacity: 0.6;
      transition: all 200ms;
      box-shadow: 0 0px 5px rgba(23, 82, 30, 0.25);

      &:hover {
        opacity: 1;
        transition: all 200ms;
        transform: scale(1.1);
        box-shadow: 0 0px 10px rgba(23, 82, 30, 0.25);
        cursor: pointer;
      }

      > img {
        width: 80%;
        width: 80%;
      }
    }
  }
}
</style>