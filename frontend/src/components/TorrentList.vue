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
                    <img class="h-8 w-8 mr-3 p-1.5 bg-slate-800 text-white rounded-md" :src="indexedCategories[torrent.category_id] && indexedCategories[torrent.category_id].icon ? `/icons/${indexedCategories[torrent.category_id].icon}` : '/icons/computer.svg'">
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
import {mapState} from "vuex";

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
  computed: {
    ...mapState(['categories']),
    indexedCategories() {
      let obj = {};
      for (let category of this.categories) {
        obj[category.category_id] = category;
      }
      return obj;
    }
  },
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
  @apply cursor-pointer bg-transparent hover:bg-gradient-to-r hover:from-slate-800/10 hover:via-slate-800/30 hover:to-slate-800/10 transition duration-200;
}
</style>
