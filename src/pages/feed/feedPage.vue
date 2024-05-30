<template>
  <div id="feed_box" v-loading="data.page_loading">
    <div class="water">
      <div class="content_box">
        <div class="master_box">
          <div class="avatar_box"><img :src="data.feed.master.avatar" alt=""></div>
          <div class="username_box">{{ data.feed.master.name }}</div>
          <div class="level_box">{{ data.feed.master.level }}</div>
        </div>
        <div class="feed_content_box">
          <div class="feed_title_box">{{ data.feed.feed_title }}</div>
          <div class="feed_info_box">
            {{ data.feed.feed_content }}
          </div>
          <div class="feed_img_list">
            <div v-for="(url,index) in data.feed.feed_img_list">
              <el-image tabindex="-1" fit="cover" :hide-on-click-modal="true" class="feed_img" :src="img_proxy(url)"
                        :preview-src-list="img_proxy_list(data.feed.feed_img_list)"
                        :lazy="true"
                        :initial-index="index"/>
            </div>
          </div>
        </div>
      </div>
      <div class="comment_box" v-masonry gutter="20" transition-duration="0s">
        <div style="width:  47.5%;height: 400px;" v-masonry-tile></div>
        <div style="width:  47.5%;height: 132px; " v-masonry-tile></div>
        <div style="width:  47.5%;height: 344px; " v-masonry-tile></div>
        <div style="width:  47.5%;height: 200px; " v-masonry-tile></div>
        <div style="width:  47.5%;height: 122px; " v-masonry-tile></div>
        <div style="width:  47.5%;height: 567px; " v-masonry-tile></div>
      </div>
    </div>

    <div class="fixed_btn">
      <div @click="back"><img src="@/assets/icon/back.png" alt=""></div>
      <div><img src="@/assets/icon/top.png" alt=""></div>
    </div>
  </div>
</template>

<script setup>
import {useRouter,useRoute} from "vue-router";
import {invoke} from "@tauri-apps/api/tauri";
import {onMounted, ref,getCurrentInstance} from "vue";
const img_proxy = getCurrentInstance().proxy.img_proxy;
const img_proxy_list = getCurrentInstance().proxy.img_proxy_list;

const router = useRouter();
const route=useRoute();
let data = ref({
  page_loading: true,
  feed:{
    master: {
    name: "加载中...",
    avatar: 'https://gss0.bdstatic.com/6LZ1dD3d1sgCo2Kml5_Y_D3/sys/portrait/item/tb.1.e8d8c93d.SIzKRGfq7QSmtHvyCeC1ZQ?t=1698846135',
    level: '加载中...'
  },
  feed_title: "加载中...",
  feed_content: "加载中...",
  feed_img_list: [],
  }
});

onMounted(async () => {
  let ret = await invoke("get_feed_info", {pid: route.query.pid})
  data.value.feed=ret
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
  overflow-y: scroll;

  .comment_box {
    width: 100%;
    padding: 5px;

    > div {
      background: white;
      border-radius: 10px;
      overflow: hidden;
      box-shadow: 0 0 8px rgba(0, 0, 0, 0.3);
      width: 47.5%;
      margin-top: 20px;
    }
  }

  .content_box {
    width: 100%;
    display: flex;
    border-bottom: 1px solid #ccc;

    .master_box {
      width: 25%;
      display: flex;
      flex-direction: column;
      align-items: center;
      padding-top: 30px;
      background-color: #f5f5f4;
      padding-bottom: 10px;

      .avatar_box {
        width: 80px;
        height: 80px;
        overflow: hidden;
        border-radius: 40px;
        box-shadow: 0 0 5px rgba(0, 0, 0, 0.3);

        img {
          width: 100%;
          object-fit: cover;
        }
      }

      .username_box {
        font-weight: bold;
        margin-top: 10px;
      }

      .level_box {
        border: 1px solid #ccc;
        padding: 2px 10px;
        border-radius: 5px;
        font-size: 13px;
        margin-top: 10px;
      }
    }

    .feed_content_box {
      width: 75%;
      display: flex;
      flex-direction: column;
      padding: 10px;

      .feed_title_box {
        font-weight: bold;
      }

      .feed_info_box {
        margin-top: 10px;
        text-indent: 20px;
        font-size: 15px;
        line-height: 16px
      }

      .feed_img_list {
        margin-top: 10px;
        width: 100%;

        :deep(.el-image-viewer__wrapper) {
          overflow: hidden;
          width: 98vw;
          height: 95vh;
          margin: 5vh auto 0;
          border-radius: 10px;
        }
        > div {
          width: 30%;
          display: inline-block;
          margin: 2px 5px;

          .feed_img {
            width: 100%;
            height: 150px;
            border-radius: 5px;
            overflow: hidden;
            object-fit: cover;
          }
        }
      }
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

::-webkit-scrollbar {
  width: 0;
}
</style>