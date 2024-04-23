<template>
  <div class="img_box">
    <img :src="img_base" alt="" :style="{transform:'scale('+img_trans+')'}" ref="img" @wheel="img_wheel">
  </div>
</template>
<script setup>
import {ref} from "vue";

defineProps({
  img_base: String
})
let img = ref();
let img_trans = ref(1);
const img_wheel = (event) => {
  if (event.deltaY < 0) {
    img_trans.value += 0.1
  } else {
    img_trans.value -= 0.1
  }
  console.log(img_trans)
  event.preventDefault();
}

</script>
<style scoped>
.img_box {
  width: 98%;
  height: 98%;
  position: fixed;
  z-index: 99990;
  top: 50%;
  left: 50%;
  background-color: rgba(0, 0, 0, 0.5);
  transform: translate(-50%, -50%);
  border-radius: 10px;
  overflow: hidden;
  display: flex;
  justify-content: center;
  align-items: center;

  img {
    position: fixed;
    z-index: 99990;
    height: 80%;
    border: 2px solid #ccc;
    transform-origin: center center;

    &:hover {
      cursor: pointer;
    }
  }

}
</style>