<template>
  <nav class="lg:text-sm lg:leading-6 relative">

    <!-- search bar -->
    <div class="sticky top-0 -ml-0.5 pointer-events-none">
      <div class="h-10 bg-slate-900"></div>
      <div class="bg-slate-900 relative pointer-events-auto">
        <div class="hidden w-full lg:flex items-center text-sm leading-6 text-slate-400 rounded-md ring-slate-900/10 shadow-sm py-1.5 pl-2 pr-3 hover:ring-slate-300 bg-slate-800 highlight-5 hover:bg-slate-700">
          <svg width="24" height="24" fill="none" aria-hidden="true" class="mr-2 flex-none">
            <path d="m19 19-3.5-3.5" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path>
            <circle cx="11" cy="11" r="6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></circle>
          </svg>
          <input @keyup.enter="submitSearch" v-model="searchQuery" type="text" name="search" placeholder="Search torrents.."
                 class="bg-transparent text-white w-full focus:outline-none">
        </div>
      </div>
      <div class="h-8 bg-gradient-to-b from-white dark:from-slate-900"></div>
    </div>

    <!-- items -->
    <ul>
      <li>
        <router-link :to="`/torrents/popular`">
          <div class="mr-4 rounded-md ring-1 ring-slate-900/5 shadow-sm group-hover:shadow group-hover:ring-slate-900/10 dark:ring-0 dark:shadow-none dark:group-hover:shadow-none dark:group-hover:highlight-white/10 group-hover:shadow-sky-200 dark:group-hover:bg-sky-500 dark:bg-sky-500 dark:highlight-white/10">
            <svg class="h-6 w-6" viewBox="0 0 24 24" fill="none">
              <path fill-rule="evenodd" clip-rule="evenodd" d="M8.5 7c1.093 0 2.117.27 3 .743V17a6.345 6.345 0 0 0-3-.743c-1.093 0-2.617.27-3.5.743V7.743C5.883 7.27 7.407 7 8.5 7Z" class="fill-sky-200 group-hover:fill-sky-500 dark:fill-sky-300 dark:group-hover:fill-sky-300"></path>
              <path fill-rule="evenodd" clip-rule="evenodd" d="M15.5 7c1.093 0 2.617.27 3.5.743V17c-.883-.473-2.407-.743-3.5-.743s-2.117.27-3 .743V7.743a6.344 6.344 0 0 1 3-.743Z" class="fill-sky-400 group-hover:fill-sky-500 dark:fill-sky-200 dark:group-hover:fill-sky-200"></path>
            </svg>
          </div>
          <span>Most Popular</span>
        </router-link>
      </li>
      <li>
        <router-link :to="`/torrents/recent`">
          <div class="mr-4 rounded-md ring-1 ring-slate-900/5 shadow-sm group-hover:shadow group-hover:ring-slate-900/10 dark:ring-0 dark:shadow-none dark:group-hover:shadow-none dark:group-hover:highlight-white/10 group-hover:shadow-sky-200 dark:group-hover:bg-sky-500 dark:bg-sky-500 dark:highlight-white/10">
            <svg class="h-6 w-6" viewBox="0 0 24 24" fill="none">
              <path fill-rule="evenodd" clip-rule="evenodd" d="M8.5 7c1.093 0 2.117.27 3 .743V17a6.345 6.345 0 0 0-3-.743c-1.093 0-2.617.27-3.5.743V7.743C5.883 7.27 7.407 7 8.5 7Z" class="fill-sky-200 group-hover:fill-sky-500 dark:fill-sky-300 dark:group-hover:fill-sky-300"></path>
              <path fill-rule="evenodd" clip-rule="evenodd" d="M15.5 7c1.093 0 2.617.27 3.5.743V17c-.883-.473-2.407-.743-3.5-.743s-2.117.27-3 .743V7.743a6.344 6.344 0 0 1 3-.743Z" class="fill-sky-400 group-hover:fill-sky-500 dark:fill-sky-200 dark:group-hover:fill-sky-200"></path>
            </svg>
          </div>
          <span>Most Recent</span>
        </router-link>
      </li>
    </ul>

    <div class="mt-6 px-4 text-gray-300">
      <ul v-if="$route.name === 'TorrentList'" id="category-filters" class="p-4 mt-2 bg-secondary rounded-xl">
        <h3 class="py-2">Categories</h3>
        <li v-for="category in categories"
            @click="selectFilter(category.name)"
            class="cursor-pointer text-gray-400 hover:text-white"
            :key="category.name">
          <span class="">{{ titleCase(category.name) }} ({{ category.num_torrents }})</span>
          <input type="checkbox" class="regular-checkbox" :checked="filterActive(category.name)">
        </li>
        <li v-if="filters.length > 0">
          <button @click="clearFilters" class="w-full rounded-lg bg-red-500 bg-opacity-10 text-red-400 hover:text-red-500 text-center cursor-pointer">Clear filters</button>
        </li>
      </ul>
    </div>
  </nav>
</template>

<script>
import { mapState } from 'vuex'
import HttpService from '@/common/http-service';

export default {
  name: 'Sidebar',
  created() {
    HttpService.get('/category', (res) => {
      const categories = res.data.data;
      this.$store.commit('setCategories', categories);
    }).catch(() => {
    });
  },
  computed: {
    ...mapState(['sideBarOpen', 'categories', 'categoryFilters'])
  },
  data: () => ({
    searchQuery: '',
    filters: []
  }),
  methods: {
    submitSearch() {
      if (this.searchQuery) {
        this.$router.push(`/torrents?search=${this.searchQuery}`)
      }
    },
    filterActive(category) {
      return this.filters.indexOf(category) > -1
    },
    selectFilter(category) {
      if (this.filters.indexOf(category) > -1) {
        this.filters.splice(this.filters.indexOf(category), 1);
      } else {
        this.filters.push(category);
      }
      this.$store.commit('setCategoryFilters', this.filters);
    },
    clearFilters() {
      this.filters = [];
      this.$store.commit('setCategoryFilters', this.filters);
    },
  }
}
</script>

<style scoped>
li a {
  @apply mb-4 flex text-slate-400 hover:text-slate-300 items-center lg:text-sm lg:leading-6;
}

.router-link-active {
  @apply text-sky-400;
}

.regular-checkbox {
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
  @apply px-4 py-1 flex flex-row justify-between;
}

#category-filters li input {
  @apply text-left
}
</style>
