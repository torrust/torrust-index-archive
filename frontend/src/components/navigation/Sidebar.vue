<template>
  <nav class="lg:text-sm lg:leading-6 relative">

    <div class="py-4">
      <router-link class="block text-2xl text-white" to="/">
        <span class="font-semibold">{{ $store.state.settings.website.name }}</span>
      </router-link>
    </div>

    <!-- search bar -->
    <div class="sticky top-0 py-4 -ml-0.5 pointer-events-none">
      <div class="relative pointer-events-auto">
        <div class="w-full flex items-center text-sm text-slate-400 bg-slate-800/50 rounded-md ring-1 ring-slate-900/10 py-1.5 pl-2 pr-3 transition duration-200 hover:ring-sky-400 focus:ring-sky-400">
          <svg width="22" height="22" fill="none" aria-hidden="true" class="mr-2 flex-none">
            <path d="m19 19-3.5-3.5" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path>
            <circle cx="11" cy="11" r="6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></circle>
          </svg>
          <input @keyup.enter="submitSearch" v-model="searchQuery" type="text" name="search" placeholder="Search torrents..."
                 class="bg-transparent text-white font-light w-full focus:outline-none">
        </div>
      </div>
      <div class="h-4 bg-gradient-to-b from-white dark:from-slate-900"></div>
    </div>

    <!-- items -->
    <ul>
      <li>
        <router-link :to="`/torrents/popular`">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M3.172 5.172a4 4 0 015.656 0L10 6.343l1.172-1.171a4 4 0 115.656 5.656L10 17.657l-6.828-6.829a4 4 0 010-5.656z" clip-rule="evenodd" />
          </svg>
          <span class="ml-2">Most Popular</span>
        </router-link>
      </li>
      <li>
        <router-link :to="`/torrents/recent`">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M11.3 1.046A1 1 0 0112 2v5h4a1 1 0 01.82 1.573l-7 10A1 1 0 018 18v-5H4a1 1 0 01-.82-1.573l7-10a1 1 0 011.12-.38z" clip-rule="evenodd" />
          </svg>
          <span class="ml-2">Most Recent</span>
        </router-link>
      </li>
      <li v-show="$router.currentRoute.fullPath.includes('torrents')" class="mt-6">
        <h3 class="py-2 text-slate-200 font-semibold">Categories</h3>
        <ul v-if="$route.name === 'Browse Torrents'" id="category-filters" class="">
          <li v-for="category in categories"
              @click="selectFilter(category.name)"
              class="cursor-pointer text-slate-400 hover:text-white"
              :key="category.name">
            <span class="">{{ titleCase(category.name) }} ({{ category.num_torrents }})</span>
            <input type="checkbox" class="" :checked="filterActive(category.name)">
          </li>
          <li v-if="filters.length > 0">
            <button @click="clearFilters" class="w-full rounded-lg bg-red-500 bg-opacity-10 text-red-400 hover:text-red-500 text-center cursor-pointer">Clear filters</button>
          </li>
        </ul>
      </li>
    </ul>
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
  },
}
</script>

<style scoped>
li a {
  @apply mb-4 flex text-slate-400 font-semibold hover:text-white items-center lg:text-sm lg:leading-6;
}

ul li .router-link-active {
  @apply text-sky-400;
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
  @apply px-4 py-1 flex flex-row justify-between;
}

#category-filters li input {
  @apply text-left
}
</style>
