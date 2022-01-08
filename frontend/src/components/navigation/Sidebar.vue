<template>
  <div class="mt-20 lg:mt-0 w-2/3 md:w-1/3 lg:w-64 fixed md:top-0 md:left-0 h-screen lg:block bg-white z-0" :class="{ 'hidden': !sideBarOpen }" id="main-nav">

    <div class="w-full h-20 flex px-4 items-center">
      <a class="text-white font-semibold text-xl mx-auto" href="/">Torrust</a>
    </div>

    <!-- search bar -->
    <div class="px-4 flex md:hidden flex-col">
      <input v-model="searchQuery" type="search" name="search" placeholder="Search Torrents.."
             class="bg-gray-100 text-black h-10 w-full xl:w-64 px-5 rounded-lg text-sm focus:outline-none">
      <button @click="submitSearch" class="mt-2 py-2 bg-primary-400 text-white rounded-lg" type="submit">
        Search
      </button>
    </div>

    <div class="mt-6 px-4 text-gray-300">
      <router-link :to="`/`" class="mt-2 w-full flex items-center h-10 pl-4 rounded-lg cursor-pointer" exact-active-class="text-white bg-secondary">
        <span>Home</span>
      </router-link>
      <router-link :to="`/torrents/popular`" class="mt-2 w-full flex items-center h-10 pl-4 rounded-lg cursor-pointer" exact-active-class="text-white bg-secondary">
        <span>Most Popular</span>
      </router-link>
      <router-link :to="`/torrents/recent`" class="mt-2 w-full flex items-center h-10 pl-4 rounded-lg cursor-pointer" exact-active-class="text-white bg-secondary">
        <span>Most Recent</span>
      </router-link>
      <router-link :to="`/torrents`" class="mt-2 w-full flex items-center h-10 pl-4 rounded-lg cursor-pointer" exact-active-class="text-white bg-secondary">
        <span>Browse Torrents</span>
      </router-link>
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
<!--      <router-link :to="'/categories'" class="mt-2 w-full flex items-center h-10 pl-4 rounded-lg cursor-pointer" exact-active-class="text-white bg-secondary">-->
<!--        <span>Browse Categories</span>-->
<!--      </router-link>-->
    </div>
  </div>
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
