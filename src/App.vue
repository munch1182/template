<script lang="ts" setup> 
import { Command } from "@shared/bridge";
import { ref } from "vue";

const msg = ref("");

async function call() {
   msg.value = await Command.call_hello("world");
}
</script>

<template>
  <div class="p-page">
    <button @click="call" type="button">调用</button>
    <span>{{ msg }}</span>
  </div>
</template>

<style scoped></style>
