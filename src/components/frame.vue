<template>
    <div class="window_frame">
        <div class="title_bar" data-tauri-drag-region></div>
        <div class="btns">
            <div @click="appWindow.minimize()" title="最小化">
                <img src="@/assets/icon/min.png" alt/>
            </div>
            <div @click="appWindow.close()" title="关闭">
                <img src="@/assets/icon/close.png" alt/>
            </div>
        </div>
    </div>
</template>

<script setup>
import {appWindow} from "@tauri-apps/api/window";
</script>

<style lang="scss">
.window_frame {
    width: 100%;
    height: 35px;
    position: fixed;
    top: 0;
    left: 0;
    display: flex;
    justify-content: space-between;


    .title_bar {
        width: 90%;
    }

    .btns {
        width: 60px;
        height: 100%;
        display: flex;
        justify-content: space-between;
        margin-right: 20px;

        > div {
            width: 25px;
            height: 80%;
            background: rgba(255, 255, 255, 0.5);
            border-radius: 5px;
            margin-top: 15px;
            display: flex;
            justify-content: center;
            align-items: center;

            &:nth-child(2):hover {
                background: rgba(255, 0, 0, 0.5);
            }

            &:hover {
                background: rgba(255, 255, 255, 0.8);
                transition: all 300ms;
                cursor: pointer;
            }

            > img {
                width: 15px;
                opacity: 0.5;
            }

        }
    }
}
</style>