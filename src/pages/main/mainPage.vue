<template>
    <div id="box" v-loading="data.page_is_loading">
        <!-- <loginBox></loginBox> -->
        <div class="forum" @wheel="marked_wheel">
            <div style="height: 30px"></div>
            <div class="marked" ref="forum">
                <div class="bar_head" v-for="bar in data.marked" :key="bar.forum_id" :title="bar.forum_name+'吧'">
                    <div class="bar_logo"><img :src="img_proxy(bar.avatar)" alt=""></div>
                    <div class="bar_name">{{ bar.forum_name }}吧</div>
                </div>
            </div>
        </div>
        <div class="cards" v-infinite-scroll="load_next_page" infinite-scroll-delay="1000" v-masonry gutter="20"
             transition-duration="0s">

            <div class="card_box" v-masonry-tile>
                <div class="hot_head">贴吧热议榜</div>
                <div class="hot_box">
                    <div v-for="(bang,index) in data.hot" :key="bang.topic_name">
                        <div>{{ index + 1 }}</div>
                        <div>{{ bang.topic_name }}</div>
                        <div>{{ bang.number }}</div>
                    </div>
                    <div class="hot_show_more" @click="toggle_more">{{
                            data.hot_show_more ? "收起" : "展开更多"
                        }}
                    </div>
                </div>
            </div>

            <div class="card_box" v-for="feed in data.feeds" :key="feed.id||feed.title" v-masonry-tile>
                <div class="feed_forum">
                    <div class="avatar_box"><img :src="img_proxy(feed.user_avatar)" alt=""></div>
                    <div class="user_box">
                        <div> {{ feed.username }}</div>
                        <div>@{{ feed.forum }}</div>
                    </div>
                </div>
                <div class="feed_title" @click="open_feed(feed)">{{ feed.title }}</div>
                <div class="feed_content" @click="open_feed(feed)">{{ feed.content }}</div>
                <div class="feed_img_list">
                    <div v-for="(url,index) in feed.img" :key="url">
                        <el-image tabindex="-1" fit="cover" :hide-on-click-modal="true" class="feed_img"
                                  :src="img_proxy(url)"
                                  :preview-src-list="img_proxy_list(feed.img)"
                                  :lazy="true"
                                  :initial-index="index"/>
                    </div>
                </div>
                <div class="feed_count">
                    <div><img src="@/assets/icon/time.png">
                        <p>{{ feed.time }}</p></div>
                    <div @click="open_feed(feed)"><img src="@/assets/icon/comment.png">
                        <p>{{ feed.post_num }}</p></div>
                </div>
            </div>
        </div>
        <div class="load_tile" v-if="page_loading" v-loading="true"></div>
        <div class="fixed_btn">
            <div title="刷新" @click="flush()"><img src="@/assets/icon/flash.png" alt=""></div>
            <div title="返回顶部" @click="to_top()"><img src="@/assets/icon/top.png" alt=""></div>
        </div>
    </div>
</template>

<script setup>
import {getCurrentInstance, inject, nextTick, onActivated, onMounted, ref} from "vue";
import {useRoute, useRouter} from "vue-router";
import {invoke} from "@tauri-apps/api/tauri";
import {ElNotification} from 'element-plus'

const redrawVueMasonry = inject("redrawVueMasonry")
const img_proxy = getCurrentInstance().proxy.img_proxy;
const img_proxy_list = getCurrentInstance().proxy.img_proxy_list;
const router = useRouter();
let data = ref({
    page: {
        limit: 20,
        offset: 0
    },
    marked: [
        {
            avatar: "加载中",
            forum_id: "12",
            forum_name: "加载中",
            member_count: 100,
            thread_count: 200,
        }
    ],
    hot_all: [{}],
    hot: [
        {
            topic_id: "加载中....",
            topic_name: "加载中...",
            topic_desc: "加载中...",
            topic_avatar: "加载中...",
            discuss_num: 999,
            is_more: true
        }
    ],
    feeds: [
        // {
        //   id: null,
        //   forum: "加载中...",
        //   title: "加载中...",
        //   content: "加载中...",
        //   username: "加载中...",
        //   user_avatar: "加载中",
        //   img: [
        //     "加载中"
        //   ]

        // }
    ],
    hot_show_more: false,
    page_is_loading: true,
    page_data_is_loading: false,
    show_content_mask: true
});

let forum = ref();
const route = useRoute()
onActivated(() => {
    redrawVueMasonry();
    nextTick(() => {
        document.getElementById(route.meta.scrollBoxId).scrollTop = route.meta.savePosition
    });
})
const to_top = () => {
    document.getElementById(route.meta.scrollBoxId).scrollTop = 0
}
const flush = () => {
    data.value.page_is_loading = true
    data.value.feeds = [];
    to_top();
    load_next_page();
    data.value.page_is_loading = false
}
onMounted(async () => {
    //加载热榜
    data.value.hot_all = await invoke("get_topic");
    data.value.feeds = [];
    await load_next_page();
    data.value.hot_show_more = true
    toggle_more();
    await load_hot_forum();
    data.value.page_is_loading = false
})
/**
 * 切换展示更多热榜，默认只展示10条
 */
const toggle_more = () => {
    if (data.value.hot_show_more) {
        data.value.hot = data.value.hot_all.slice(0, 10);
    } else {
        data.value.hot = data.value.hot_all;
    }
    data.value.hot_show_more = !data.value.hot_show_more;
    //重绘布局
    nextTick(() => {
        redrawVueMasonry();
    });
}
/**
 * 加载顶部贴吧
 */
const load_hot_forum = async () => {
    data.value.marked = await invoke("get_hot_forum");
}
let page_loading = ref(false);
/**
 * 加载下一页数据
 * @returns {Promise<void>}
 */
const load_next_page = async () => {
    if (page_loading.value) {
        return;
    }
    page_loading.value = true;
    let last_tid = null;
    if (data.value.feeds.length > 0) {
        last_tid = data.value.feeds[data.value.feeds.length - 1].id
    }
    let new_page = await invoke("get_index_page", {
        limit: data.value.page.limit + '',
        offset: data.value.page.offset + '',
        lastTid: last_tid + ''
    }).catch(e => {
        ElNotification({
            title: '失败',
            message: e,
            type: 'error'
        })
    });
    if (new_page) {
        data.value.feeds.push(...new_page);
        //这里每次固定+20，不管实际返回了多少条，跟网页版一样，
        data.value.page.offset += 20;
    }

    page_loading.value = false;

}
/**
 * 顶部的滚动
 * @param event e
 */
const marked_wheel = (event) => {
    forum.value.scrollLeft += event.deltaY;
    event.preventDefault();
}
/**
 * 打开帖子
 */
const open_feed = (feed) => {
    router.push(
        {
            name: "feedPage",
            query: {pid: feed.id}
            // query: {pid: "9046705650"}
        }
    )
}
</script>

<style scoped lang="scss">

#box {
    width: 100%;
    height: 100%;
    overflow-y: auto;
    padding-top: 10px;


    .load_tile {
        width: 100%;
        text-align: center;
    }

    .forum {
        height: 15%;
        width: 98%;

        .marked {
            padding: 5px;
            display: flex;
            width: 100%;
            overflow-x: scroll;
            white-space: nowrap;

            .bar_head {
                width: 50px;
                height: 50px;
                float: left;
                cursor: pointer;

                .bar_logo {
                    width: 40px;
                    height: 40px;
                    border-radius: 20px;
                    overflow: hidden;
                    margin: 0 auto;
                    box-shadow: 0 0 8px rgba(0, 0, 0, 0.3);

                    img {
                        width: 100%;
                    }
                }

                .bar_name {
                    width: 50px;
                    font-size: 10px;
                    scale: 0.8;
                    text-align: center;
                    white-space: nowrap;
                    text-overflow: ellipsis;
                    overflow: hidden;

                    &:hover {
                        text-decoration: underline;
                        background: #3388ff;
                        color: white;
                        border-radius: 5px;
                        box-shadow: 0 0 8px rgba(0, 0, 0, 0.3);
                    }
                }
            }

            .bar_head:not(:first-child) {
                margin-left: 15px;
            }

            .bar_head:first-child {
                margin-left: 10px;
            }
        }
    }

    .cards {
        width: 100%;
        padding-left: 2%;
        box-sizing: border-box;

        .card_box {
            background: white;
            border-radius: 10px;
            box-shadow: 0 0 8px rgba(0, 0, 0, 0.3);
            padding: 10px;
            width: 450px;
            margin-top: 20px;
            height: 200px;
            overflow: hidden;

            .feed_count {
                height: 25px;
                display: flex;
                border: 1px solid #ccc;
                border-radius: 5px;

                > div {
                    width: 50%;
                    height: 100%;
                    line-height: 25px;
                    font-size: 13px;
                    display: flex;
                    align-items: center;
                    justify-content: center;

                    &:hover {
                        cursor: pointer;
                        background: #ccc;
                    }

                    > img {
                        object-fit: cover;
                        height: 100%;
                        transform: scale(0.7);
                    }
                }
            }

            .hot_head {
                height: 35px;
                line-height: 35px;
                text-indent: 5px;
                border-radius: 5px;
                width: 100%;
                border: 1px solid #ccc;
                color: #ff4757;
                font-weight: bold;
            }

            .hot_box {
                margin-top: 5px;
                transition: all 200ms;

                .hot_show_more {
                    font-size: 14px;

                    &:hover {
                        text-decoration: underline;
                        background: #3388ff;
                        color: white;
                        border-radius: 5px;
                        box-shadow: 0 0 8px rgba(0, 0, 0, 0.3);
                        text-indent: 10px;
                    }
                }

                > div:nth-child(1) > div:nth-child(1) {
                    color: red;
                    font-weight: bold;
                }

                > div:nth-child(2) > div:nth-child(1) {
                    color: red;
                    font-weight: bold;
                }

                > div:nth-child(3) > div:nth-child(1) {
                    color: red;
                    font-weight: bold;
                }

                > div {
                    display: flex;
                    color: #666;
                    line-height: 25px;
                    cursor: pointer;

                    > div:nth-child(1) {
                        width: 8%;
                        height: 15px;
                    }

                    > div:nth-child(2) {
                        white-space: nowrap;
                        text-overflow: ellipsis;
                        overflow: hidden;
                        font-size: 15px;
                        width: 72%;

                        &:hover {
                            text-decoration: underline;
                            background: #3388ff;
                            color: white;
                            border-radius: 5px;
                            box-shadow: 0 0 8px rgba(0, 0, 0, 0.3);
                            text-indent: 10px;
                        }
                    }

                    > div:nth-child(3) {
                        width: 20%;
                        text-align: right;
                        white-space: nowrap;
                        text-overflow: ellipsis;
                        overflow: hidden;
                        font-size: 13px;
                    }
                }
            }

            .feed_forum {
                width: 100%;
                min-height: 25%;
                font-size: 16px;
                background: #f6f7fb;
                border-radius: 10px;

                > div {
                    display: inline-block;
                }

                .avatar_box {
                    width: 50px;
                    height: 50px;
                    overflow: hidden;
                    border-radius: 25px;
                    box-shadow: 0 0 5px rgba(0, 0, 0, 0.3);
                }

                .user_box {
                    > div:nth-child(1) {
                        line-height: 50px;
                        margin-left: 10px;

                        &:hover {
                            text-decoration: underline;
                        }
                    }

                    > div:nth-child(2) {
                        font-size: 10px;
                        line-height: 50px;
                        margin-left: 5px;
                        color: blue;

                        &:hover {
                            text-decoration: underline;
                        }
                    }

                    > div {
                        float: left;
                        cursor: pointer;
                    }
                }

                img {
                    width: 100%;
                }
            }

            .feed_title {
                margin-top: 5px;
                cursor: pointer;

                &:hover {
                    text-decoration: underline;
                }
            }

            .feed_content {
                font-size: 14px;
                text-indent: 10px;
                margin-top: 5px;
                cursor: pointer;
                color: #666;

                &:hover {
                    text-decoration: underline;
                }
            }

            .feed_img_list {
                width: 100%;
                padding: 2px;
                background: #f6f7fb;
                border-radius: 5px;
                margin-top: 10px;

                > div {
                    width: 48%;
                    display: inline-block;
                    margin: 2px;
                }

                .feed_img {
                    width: 100%;
                    height: 150px;
                    object-fit: cover;
                    cursor: zoom-in;
                    border-radius: 5px;
                    transition: all 200ms;

                }
            }

            .feed_title {
                width: 100%;
                min-height: 25%;
                font-size: 16px;
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