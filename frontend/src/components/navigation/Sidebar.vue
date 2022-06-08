<template>
  <nav class="lg:text-sm lg:leading-6 relative">

    <div class="py-4">
      <router-link class="block text-2xl text-white" to="/">
        <span class="font-semibold text-slate-400 hover:text-white transition duration-200">{{ $store.state.settings.website.name }}</span>
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
      <li class="nav">
        <button @click="goTo('/torrents/popular')" :class="{ 'active': $route.params.sorting === 'popular' }">
          <FireIcon class="h-5 w-5 opacity-50" />
          <span class="ml-2">Most Popular</span>
        </button>
      </li>
      <li class="nav">
        <button @click="goTo('/torrents/recent')" :class="{ 'active': $route.params.sorting === 'recent' }">
          <ClockIcon class="h-5 w-5 opacity-50" />
          <span class="ml-2">Most Recent</span>
        </button>
      </li>
      <li class="mt-6">
        <h3 class="py-2 text-slate-400 font-semibold">Categories</h3>
        <ul>
          <li v-for="category in categories">
            <button @click="selectCategory(category.name)" :class="{ 'active': $route.name === 'Browse Torrents' && categoryFilters.indexOf(category.name) > -1 }">
              <span>{{ titleCase(category.name.toString()) }}</span>
              <span class="ml-auto">{{ category.num_torrents }}</span>
            </button>
          </li>
        </ul>
        <h3 class="py-2 text-slate-400 font-semibold">Pages</h3>
        <ul>
          <li v-for="page in pages">
            <button @click="selectPage(page.route)" :class="{ 'active': $route.name === page.title }">
              <span>{{ titleCase(page.title.toString()) }}</span>
            </button>
          </li>
        </ul>
      </li>
    </ul>
  </nav>
</template>

<script>
import { mapState } from 'vuex'
import HttpService from '@/common/http-service';
import { ClockIcon, FireIcon } from "@vue-hero-icons/outline"

export default {
  name: 'Sidebar',
  components: {ClockIcon, FireIcon},
  created() {
    HttpService.get('/category', (res) => {
      const categories = res.data.data;
      this.$store.commit('setCategories', categories);
    }).catch(() => {
    });
    HttpService.get('/pages', (res) => {
      const pages = res.data.data;
      this.$store.commit('setPages', pages);
    }).catch(() => {
    }); 
  },
  computed: {
    ...mapState(['sideBarOpen', 'categories', 'categoryFilters', 'pages'])
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
    selectCategory(category) {
      this.$store.commit('setCategoryFilters', [category]);
      if (this.$route.name !== 'Browse Torrents') {
        this.$router.push(`/torrents`)
      } else if (this.$route.params.sorting) {
        this.$router.replace(`/torrents`)
      }
    },
    selectPage(page) {
       this.$router.push(`/${page}`)
    },
    goTo(url) {
      this.$store.commit('setCategoryFilters', []);
      if (this.$route.path !== url) {
        this.$router.push(url)
      }
    }
  },
}
</script>

<style scoped>
button {
  @apply px-3 py-1.5 mb-2 w-full flex text-slate-400 border border-slate-800 rounded-md hover:text-white items-center lg:text-sm lg:leading-6;
}

button.active {
  @apply bg-sky-500 text-white;
}
</style>
