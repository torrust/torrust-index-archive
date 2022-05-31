<template>
  <button @click="click" :disabled="disabled">
    <TrashIcon class="h-5 w-5 opacity-50" />
    <span class="ml-1">discard changes</span>
  </button>
</template>

<script>
import { TrashIcon } from "@vue-hero-icons/outline";

export default {
  name: "CancelButton",
  components: { TrashIcon },
  props: ['click', 'disabled']
}
</script>

<style scoped>
button {
  @apply py-1.5 px-3 flex items-center bg-sky-200 text-xs text-sky-600 font-semibold rounded-md transition duration-200 hover:text-sky-500 disabled:opacity-50;
}
</style>
