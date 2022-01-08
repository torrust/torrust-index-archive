<template>
  <notifications position="top right">
    <template slot="body" slot-scope="{ item, close }">
      <div
          class="notification text-white m-2 rounded-lg py-4 px-6"
          :class="item.type"
      >
        <div class="flex justify-between items-center mb-2 text">
          <p class="font-semibold">
            {{ item.title }}
          </p>
          <button class="close" @click="close">
            <XIcon/>
          </button>
        </div>
        <div v-html="item.text"/>
      </div>
    </template>
  </notifications>
</template>

<script>
import {XIcon} from "@vue-hero-icons/solid";

export default {
  name: "Toast",
  components: {XIcon}
}
</script>

<style lang="scss">
.notification {
  &.success {
    @apply bg-green-500;
  }

  &.warn {
    @apply bg-yellow-500;
  }

  &.info {
    @apply bg-blue-400;
  }

  &.error {
    @apply bg-red-500;
  }
}
</style>
