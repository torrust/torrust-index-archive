<template>
  <div class="relative inline-block text-left" v-click-outside="() => (dropdownOpened = false)">
    <button class="filter relative" @click="dropdownOpened = !dropdownOpened">
      <AdjustmentsIcon size="16" class="mr-1 opacity-50" />
      Categories
    </button>
    <div class="origin-top-left absolute left-0 mt-2 z-10" :class="{hidden: !dropdownOpened}">
      <div class="py-2 px-2 w-48 flex flex-col bg-slate-800 text-sm rounded-md shadow-lg">
        <ul v-if="$route.name === 'Browse Torrents'" id="category-filters" class="">
          <li v-for="category in categories"
              @click="selectFilter(category.name)"
              class="cursor-pointer text-slate-400 hover:text-white"
              :key="category.name">
            <span class="">{{ titleCase(category.name) }} ({{ category.num_torrents }})</span>
            <input type="checkbox" class="" :checked="filterActive(category.name)">
          </li>
          <li v-if="categoryFilters.length > 0">
            <button @click="clearFilters" class="py-1.5 w-full rounded-md bg-red-500 bg-opacity-10 text-red-400 transition duration-200 hover:text-red-500">Clear filters</button>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script>
import {UserIcon} from "@vue-hero-icons/outline";
import { AdjustmentsIcon } from '@vue-hero-icons/solid'
import {mapState} from "vuex";

export default {
  name: "FilterCategory",
  components: {UserIcon, AdjustmentsIcon},
  data: () => ({
    dropdownOpened: false,
  }),
  computed: {
    ...mapState({
      user: state => state.auth.user,
      categories: state => state.categories,
      categoryFilters: state => state.categoryFilters
    })
  },
  methods: {
    filterActive(category) {
      return this.categoryFilters.indexOf(category) > -1
    },
    selectFilter(category) {
      let filters = this.categoryFilters;
      if (filters.indexOf(category) > -1) {
        filters.splice(filters.indexOf(category), 1);
      } else {
        filters.push(category);
      }
      this.$store.commit('setCategoryFilters', filters);
    },
    clearFilters() {
      this.$store.commit('setCategoryFilters', []);
    },
  }
}
</script>

<style scoped>
.filter {
  @apply px-3 py-1.5 text-slate-400 text-sm border border-slate-800 rounded-md flex items-center relative cursor-pointer transition duration-200 hover:text-slate-200 hover:border-slate-200;
}

.regular-checkbox {
  @apply text-sky-400;
  -webkit-appearance: none;
  background-color: rgba(0, 0, 0, .1);
  border: none;
  box-shadow: 0 1px 2px rgba(0,0,0,0.05), inset 0px -15px 10px -12px rgba(0,0,0,0.05);
  padding: 9px;
  border-radius: 3px;
  display: inline-block;
  position: relative;
}

#category-filters li {
  @apply px-2 py-1 flex flex-row justify-between;
}

#category-filters li input {
  @apply text-left
}
</style>
