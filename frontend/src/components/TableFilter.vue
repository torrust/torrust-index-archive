<template>
  <div class="relative inline-block text-left">
    <div class="">
      <button type="button"
              class="inline-flex justify-center w-full rounded-lg bg-primary border border-secondary shadow-sm px-4 py-2 text-sm font-medium text-white"
              id="menu-button" aria-expanded="true" aria-haspopup="true"
              @click="dropdownOpened = !dropdownOpened"
              v-click-outside="() => (dropdownOpened = false)"
      >
        Categories
        <!-- Heroicon name: solid/chevron-down -->
        <svg class="-mr-1 ml-2 h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor"
             aria-hidden="true">
          <path fill-rule="evenodd"
                d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
                clip-rule="evenodd"/>
        </svg>
      </button>
    </div>

    <div
        :class="{hidden: !dropdownOpened}"
        class="origin-top-right absolute right-0 mt-2 w-56 rounded-lg shadow-lg bg-white ring-1 ring-black ring-opacity-5 focus:outline-none z-10"
        role="menu" aria-orientation="vertical" aria-labelledby="menu-button" tabindex="-1"
    >
      <div class="py-1" role="none">
        <a
            v-for="(label, category) in categoryFilters"
            :key="category"
            class="item"
            :class="{active: selectedItems.indexOf(category) > -1}"
            @click="selectItem(category)"
        >
          {{ label }}
        </a>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: "TableFilter",
  data: () => ({
    dropdownOpened: false,
    selectedItems: [],
    categoryFilters: {
      movies: "Movies",
      other: "Other",
    }
  }),
  methods: {
    selectItem(category) {
      if (this.selectedItems.indexOf(category) > -1) {
        this.selectedItems.splice(this.selectedItems.indexOf(category), 1)
      } else {
        this.selectedItems.push(category);
      }
      this.$emit('update:filters', this.selectedItems);
    }
  }
}
</script>

<style scoped>
.item {
  @apply text-gray-800 block px-4 py-2 text-sm;
}

.active {
  @apply bg-blue-300 text-gray-700;
}
</style>
