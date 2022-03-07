<template>
  <div class="flex flex-col">
    <div class="-my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
      <div class="py-2 align-middle inline-block min-w-full sm:px-6 lg:px-8">
        <div class="overflow-hidden border-b border-slate-800">
          <table class="min-w-full">
            <thead class="text-white rounded-md border-b border-t border-slate-800">
              <tr>
                <th v-for="(column, index) in table_columns" :key="column" @click="changeSort(column)"
                    scope="col" class="px-4 py-3" :class="{ 'rounded-l-md': column === 'name' }"
                >
                  <button
                      class="flex flex-row justify-start items-center text-left text-sm font-semibold capitalize"
                  >
                    <span>{{ column }}</span>
                    <SortAscendingIcon size="14" v-if="sorting.name===column&&sorting.direction==='ASC'" class="ml-1" />
                    <SortDescendingIcon size="14" v-if="sorting.name===column&&sorting.direction==='DESC'" class="ml-1" />
                  </button>
                </th>
                <th scope="col" class="px-4 py-3 text-left text-sm font-semibold capitalize rounded-r-md">
                  Uploader
                </th>
              </tr>
            </thead>
            <tbody class="divide-y divide-slate-800 text-slate-400">
              <tr
                  v-for="(torrent, index) in torrents" :key="index"
                  @click="$router.push(`/torrent/${torrent.torrent_id.toString()}`)"
                  class="duration-200"
              >
                <td>
                  <div class="flex flex-row items-center">
<!--                    <svg v-if="torrent.category_id === 2" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">-->
<!--                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 4v16M17 4v16M3 8h4m10 0h4M3 12h18M3 16h4m10 0h4M4 20h16a1 1 0 001-1V5a1 1 0 00-1-1H4a1 1 0 00-1 1v14a1 1 0 001 1z" />-->
<!--                    </svg>-->
<!--                    <svg v-if="torrent.category_id === 3" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">-->
<!--                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />-->
<!--                    </svg>-->
<!--                    <svg v-if="torrent.category_id === 1" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">-->
<!--                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3" />-->
<!--                    </svg>-->
                    <span>{{ torrent.title }}</span>
                  </div>
                </td>
                <td class="text-green-500 font-light">
                  {{ torrent.seeders }}
                </td>
                <td class="text-red-500 font-light">
                  {{ torrent.leechers }}
                </td>
                <td>
                  {{ timeSince(torrent.upload_date) }} ago
                </td>
                <td>
                  {{ fileSize(torrent.file_size) }}
                </td>
                <td>
                  {{ torrent.uploader }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { SortAscendingIcon, SortDescendingIcon } from "@vue-hero-icons/outline"

export default {
  name: "TorrentList",
  components: {SortAscendingIcon, SortDescendingIcon},
  props: {
    torrents: Array,
    updateSorting: Function,
    sorting: Object,
  },
  data: () => ({
    sort: {
      name: 'uploaded',
      direction: 'DESC'
    },
    table_columns: [
      'name',
      'seeders',
      'leechers',
      'uploaded',
      'size',
    ]
  }),
  methods: {
    changeSort(sort) {
      let direction = 'ASC';
      if (this.sorting.name === sort) {
        if (this.sorting.direction === 'ASC') {
          direction = 'DESC'
        } else {
          direction = 'ASC'
        }
      } else {
        this.sorting.name = sort;
        direction = 'DESC'
      }
      this.updateSorting({name: sort, direction});
      //this.$emit('update:sorting', sort);
    }
  }
}
</script>

<style scoped>
td {
  @apply px-4 py-4 whitespace-nowrap;
}

tbody tr {
  @apply cursor-pointer bg-transparent hover:bg-gradient-to-r hover:from-slate-800/10 hover:via-slate-800/30 hover:to-slate-800/10;
}
</style>
