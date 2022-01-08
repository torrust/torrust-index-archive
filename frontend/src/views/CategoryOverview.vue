<template>
  <div>
    <h1 class="view-title text-white">Categories</h1>

    <div v-if="categories.length === 0" class="relative w-full text-center p-12 font-medium text-sm text-gray-700">
      Loading...
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-3 mt-6">
      <router-link :to="`/torrents/?categories=${urlSafe(category.name)}`"
                   v-for="(category, index) in categories"
                   :key="index"
                   class="category-tile bg-primary duration-300 transform scale-100 hover:bg-light"
      >
        <p class="capitalize font-semibold text-white text-xl">{{ category.name }}</p>
        <div class="details">
          <p class="pr-2 inline-flex text-gray-200 text-sm">
            {{ category.num_torrents }}
            Torrents
          </p>
        </div>
      </router-link>
    </div>
  </div>
</template>

<script>
import {mapState} from "vuex";

export default {
  name: "CategoryOverview",
  computed: {
    ...mapState(['categories'])
  }
}
</script>

<style scoped>
.category-tile {
  @apply bg-cover !important;
  @apply w-full rounded-3xl shadow-lg text-center py-16 relative;
}

.details {
  @apply inline-flex;
}
</style>
