<template>
  <button @click="click" :disabled="disabled">
    <CheckIcon class="h-5 w-5 opacity-50" />
    <span class="ml-1">save changes</span>
  </button>
</template>

<script>
import { CheckIcon } from "@vue-hero-icons/outline";

export default {
  name: "SaveButton",
  components: { CheckIcon },
  props: ['click', 'disabled']
}
</script>

<style scoped>
button {
  @apply py-1.5 px-3 flex items-center text-xs text-white font-semibold bg-sky-600 rounded-md transition duration-200 hover:bg-sky-500 disabled:opacity-50;
}
</style>
