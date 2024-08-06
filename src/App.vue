<template>
  <div class="container">
    <frame/>
    <router-view/>
  </div>
</template>

<script setup>
import frame from "@/components/frame.vue";
</script>

<style lang="scss">

body {
  -webkit-touch-callout: none;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
  cursor: default;
  background: url("assets/img/bg.jpg") no-repeat;
  background-size: 100%;
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
