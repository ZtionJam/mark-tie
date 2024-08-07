<template>
  <div class="input_box">
    <span class="text">{{ text }}：</span>
    <input @input="$emit('update:modelValue', $event.target.value)" :value="modelValue" class="input"
           :placeholder="placeholder">
  </div>
</template>

<script setup>

import {defineEmits, defineProps} from 'vue';

defineProps({
  modelValue: String,
  text: String,
  placeholder: String,
});
defineEmits(['update:modelValue']);
</script>

<style scoped lang="scss">
.input_box {
  width: 100%;
  height: 35px;
  display: flex;
  justify-content: center;
  align-items: center;

  .text {
    font-size: 16px;
  }

  .input {
    height: 35px;
    width: 250px;
    border: 1px solid #539eec;
    font-size: 16px;
    border-radius: 5px;
    opacity: 0.5;
    padding: 0 10px;

    &:focus {
      outline: none;
      border: 1px solid #2289f1;
      box-shadow: 2px 10px 10px rgba(0, 0, 0, 0.25);
    }

    &:active {
      border: none;
    }
  }

}

</style>