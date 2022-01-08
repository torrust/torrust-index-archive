<template>
  <div class="bg-primary p-6 rounded-3xl">
    <div v-if="search" class="flex flex-row">
      <h2 class="text-gray-400">Search results for '{{ this.search }}'</h2>
      <button @click="clearSearch" class="px-2 ml-2 text-sm rounded-xl bg-red-500 bg-opacity-10 text-red-400 hover:text-red-500">Clear search</button>
    </div>
    <div class="flex justify-between">
      <h1 class="view-title text-white">Browse Torrents</h1>
    </div>
    <router-view/>
    <TorrentList v-if="torrents.results.length > 0" :torrents="torrents.results" :sorting="sorting" :update-sorting="updateSorting"/>
    <Pagination v-if="torrents.results.length > 0" :current-page.sync="currentPage" :total-pages="totalPages" :total-results="torrents.total" :page-size="pageSize" />
    <div v-else class="py-6 text-gray-300">This category has no results.</div>
  </div>
</template>

<script>
import TorrentList from "../components/TorrentList";
import Pagination from "../components/Pagination";
import HttpService from "@/common/http-service";
import {mapState} from "vuex";

export default {
  name: "Torrents",
  components: {Pagination, TorrentList},
  data: () => ({
    sorting: {
      name: 'uploaded',
      direction: 'DESC',
    },
    search: '',
    torrents: {
      total: 0,
      results: []
    },
    currentPage: 1,
    pageSize: 20,
  }),
  methods: {
    loadTorrents(page) {
      HttpService.get(`/torrents?page_size=${this.pageSize}&page=${page-1}&sort=${this.sorting.name}_${this.sorting.direction}&categories=${this.categoryFilters.join(',')}&search=${this.search}`, (res) => {
        this.torrents = res.data.data;
      }).catch(() => {
      });
    },
    updateSortFromRoute() {
      if (this.$route.params.sorting) {
        let sort = this.$route.params.sorting;
        switch (sort) {
          case 'popular':
            this.sorting.name = 'seeders';
            this.sorting.direction = 'DESC';
            break;
          case 'recent':
            this.sorting.name = 'uploaded';
            this.sorting.direction = 'DESC';
            break;
          default:
            this.sorting.name = sort;
        }
      }
    },
    clearSearch() {
      this.$router.replace({ query: {...this.$route.query, search: ''}})
    },
    updateSorting(sorting) {
      console.log(sorting);
      this.sorting = sorting;
      this.loadTorrents(this.currentPage);
    }
  },
  computed: {
    ...mapState(['categoryFilters']),
    totalPages() {
      return Math.ceil(this.torrents.total / this.pageSize);
    },
  },
  watch: {
    '$route.query.search': function (search) {
      search ? this.search = search : this.search = '';
      this.loadTorrents(this.currentPage, this.sorting);
    },
    '$route.params.sorting': function () {
      this.updateSortFromRoute();
      this.loadTorrents(this.currentPage, this.sorting);
    },
    filters() {
      this.loadTorrents(this.currentPage, this.sorting);
    },
    currentPage(newPage) {
      this.loadTorrents(newPage, this.sorting);
    },
    categoryFilters() {
      this.loadTorrents(this.currentPage, this.sorting);
    }
  },
  mounted() {
    this.$route.query.search ? this.search = this.$route.query.search : this.search = '';
    this.updateSortFromRoute();
    this.loadTorrents(this.currentPage, this.sorting);
  }
}
</script>

<style scoped>

</style>
