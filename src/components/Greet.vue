<script setup>
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  let data = await invoke("get_index_page", {limit: "0", offset: "20"});

  console.log(data)

  greetMsg.value = data
}
</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..."/>
    <button type="submit">Greet</button>
  </form>

  <p>{{ greetMsg }}</p>
</template>
