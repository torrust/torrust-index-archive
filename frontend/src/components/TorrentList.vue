<template>
  <div class="col-span-12">
    <div class="overflow-auto lg:overflow-visible ">
      <table class="table w-full mb-2 border-separate text-sm whitespace-nowrap">
        <thead class="text-gray-500">
        <tr>
          <th v-for="column in table_columns" :key="column" @click="changeSort(column)"
              class="cursor-pointer text-center hover:text-white"
              :class="{ 'text-white': sorting.name === column }"
          >
            <div class="flex flex-row justify-start items-center" :class="{ 'justify-center': column!=='name' }">
              <span>{{ titleCase(column) }}</span>
              <SortAscendingIcon size="16" v-if="sorting.name===column&&sorting.direction==='ASC'" class="ml-1" />
              <SortDescendingIcon size="16" v-if="sorting.name===column&&sorting.direction==='DESC'" class="ml-1" />
            </div>
          </th>
          <th class="">Uploader</th>
        </tr>
        </thead>
        <tbody class="text-gray-300">
        <tr
            v-for="(torrent, index) in torrents" :key="index"
            @click="$router.push(`/torrent/${torrent.torrent_id.toString()}`)"
            class="hover:bg-black hover:bg-opacity-10 text-center cursor-pointer duration-100 hover:text-white"
        >
          <td class="font-bold text-left" :class="{ 'text-red-400': torrent.seeders === 0 }">
            {{ torrent.title }}
          </td>
          <td class="text-green-400 font-bold">
            {{ torrent.seeders }}
          </td>
          <td class="text-red-400 font-bold">
            {{ torrent.leechers }}
          </td>
          <td class="">
            {{ timeSince(torrent.upload_date) }} ago
          </td>
          <td class="">
            {{ fileSize(torrent.file_size) }}
          </td>
          <td class="font-bold">
            {{ torrent.uploader }}
          </td>
        </tr>
        </tbody>
      </table>
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

<style lang="scss" scoped>
.table {
  border-spacing: 0 15px;
}

i {
  font-size: 1rem !important;
}

.table tr {
  border-radius: 20px;
}

tr {
  &:hover {
    td {
      @apply duration-100;
      @apply border-blue-400;
    }
  }
}

tr th,
tr td {
  @apply p-3;
}

th {
  @apply font-light;
}

//tr td {
//  @apply border-t-2 border-b-2 border-gray-200;
//}
//
//
tr td:nth-child(n+6) {
  border-radius: 0 .625rem .625rem 0;
  //@apply border-r-2;
}

tr td:nth-child(1) {
  border-radius: .625rem 0 0 .625rem;
  //@apply border-l-2;
}
</style>
