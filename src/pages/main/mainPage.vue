<template>
  <div class="box">
    <div class="top_bar">
      <div class="title">{{ title }}</div>
    </div>
    <div class="tie_card">
      <swiper
          :mousewheel="true"
          :initial-slide="1"
          :autoplay="true"
          :slides-per-view="'auto'"
          :grab-cursor="true"
          :centeredSlides="true"
          :effect="'creative'"
          :creative-effect="creativeEffect"
          :modules="[EffectCreative,Mousewheel]"
          :max-backface-hidden-slides="1"
          @slideChange="updateActiveIndex"
      >
        <swiper-slide v-for="(item,index) in cards" class="card"
                      @click="scrollToSlide(index)">
          <div :class="{card_content:true,card_shadow:activeIndex===index}">
            <button>{{ item.name }}</button>
          </div>
        </swiper-slide>
      </swiper>
    </div>

    <!--操作栏-->
    <div class="user_bar">
      <div class="avatar_box">
        <div class="avatar"><img src="@/assets/img/avatar.jpg" alt=""/></div>
        <div class="name">ZtionJam</div>
      </div>
      <div v-for="(item,index) in bar_btn" @click="toggle_bar(index)" :class="{underLine:now_bar===index,bar_btn}">
        {{ item.name }}
      </div>
      <div class="search_box">搜索</div>
    </div>
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
import {Swiper, SwiperSlide} from 'swiper/vue';
import {EffectCreative, Mousewheel} from 'swiper/modules';

let router = useRouter();
const card_wrap = ref(null);
let creativeEffect = {
  prev: {
    translate: [-90, 0, 0],
    scale: 0.85,
    shadow: false,
  },
  next: {
    translate: [90, 0, 0],
    scale: 0.85,
    shadow: false,
  },
  limitProgress: Math.floor(3 / 2),
  shadowPerProgress: true,
}
let now_bar = ref(0)
let title = ref("Recommended post");
let bar_btn = [
  {name: "推荐", title: "Recommended post"},
  {name: "收藏", title: "Favorite"},
  {name: "进吧", title: "Enter TieBa"},
  {name: "关注的", title: "My Followee"},
  {name: "设置", title: "Mark-tie setting"}];
let cards = [{name: "000"}, {name: "111"}, {name: "222"}, {name: "333"}, {name: "444"}, {name: "555"}];
const pageLoading = ref(false);
onMounted(async () => {
  pageLoading.value = true;
  await invoke("check_login").catch((err) => {
    Notice(err, "err");
    router.push("/login")
  })
  pageLoading.value = false;
  card_wrap.value = document.querySelector('.swiper').swiper;
})
const toggle_bar = (index) => {
  now_bar.value = index
  title.value=bar_btn[index].title
}
const scrollToSlide = (index) => {
  card_wrap.value.slideTo(index);
}
const activeIndex = ref(1);
const updateActiveIndex = () => {
  if (card_wrap.value) {
    activeIndex.value = card_wrap.value.activeIndex;
    console.log(activeIndex.value)
  }
};
</script>

<style scoped lang="scss">
.box {
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: center;

  .top_bar {
    height: 60px;
    width: 100%;
    margin-top: 20px;

    .title {
      width: 250px;
      color: white;
      font-weight: bold;
      line-height: 40px;
      border-bottom: 2px solid #dd32f8;
      font-size: 25px;
      margin-left: 40px;
    }
  }

  .tie_card {
    height: 80vh;
    width: 800px;
    cursor: default;

    .card {
      cursor: default;
      width: 700px;
      height: 430px;
      display: flex;
      justify-content: center;
      align-items: center;

      .card_content {
        width: 680px;
        height: 410px;
        border-radius: 10px;
        //background: url("../../assets/img/tieBg.jpg") no-repeat;
        background: white;
        background-size: 100%;
        display: flex;
        justify-content: center;
        align-items: center;
      }
    }

  }

  .user_bar {
    height: 70px;
    width: 95%;
    margin-bottom: 10px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    z-index: 999;
    border-radius: 10px;
    background: url("../../assets/img/bg.jpg") no-repeat;
    background-size: 100%;
    box-shadow: 0 10px 10px rgba(0, 0, 0, 0.25);

    .bar_btn {
      line-height: 30px;
      transition: all 200ms;

      &:hover {
        cursor: pointer;
        font-weight: bold;
        transition: all 200ms;
      }
    }

    .underLine {
      border-bottom: 3px solid #dd32f8;
      transition: all 200ms;

    }

    .avatar_box {
      width: 100px;
      height: 110px;
      margin-left: 20px;
      margin-top: -40px;

      .avatar {
        width: 80px;
        height: 80px;
        border-radius: 40px;
        margin: 0 auto;
        overflow: hidden;
        box-shadow: 0 0 8px rgba(0, 0, 0, 0.25);

        > img {
          width: 100%;
          object-fit: cover;
        }
      }

      .name {
        width: 100px;
        height: 20px;
        text-align: center;
        margin-top: 5px;
      }
    }

    .search_box {
      width: 200px;
      height: 35px;
      border: 1px solid #ccc;
      line-height: 35px;
      text-indent: 10px;
      margin-right: 10px;
      border-radius: 10px;
    }
  }
}

.card_shadow {
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.25);
  box-sizing: border-box;
}

</style>