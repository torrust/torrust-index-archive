<template>
  <div class="text-slate-500">
    <h1 class="view-title text-white">Welcome to {{ $store.state.settings.website.name }}</h1>

    <p class="py-2 text-white">Welcome to my torrent index.</p>

  </div>
</template>

<script>

export default {
  name: "Welcome"
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
