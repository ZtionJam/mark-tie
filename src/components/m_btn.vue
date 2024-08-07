<template>
    <div class="btn_box">
        <button class="btn">{{ text }}</button>
    </div>
</template>

<script setup>

import {defineProps} from 'vue';

defineProps({
    text: String,
});
</script>

<style scoped lang="scss">
.btn_box {
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;

    .btn {
        width: 100px;
        height: 35px;
        border: 1px solid #6672ed;
        border-radius: 5px;
        background: linear-gradient(to right, lightblue, aqua, lightgreen);
        opacity: 0.8;

        &:hover {
            cursor: pointer;
            box-shadow: 0 0 5px rgba(0, 0, 0, 0.25);
        }

        &:active {
            box-shadow: 0 0 10px rgba(0, 0, 0, 0.25);
        }
    }

}
</style>