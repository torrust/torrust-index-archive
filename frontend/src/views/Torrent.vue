<template>
  <div class="py-6 overflow-auto fixed top-0 left-0 z-20 flex-col justify-center w-full h-screen bg-black bg-opacity-50"
      @click.self="closeModal"
  >
    <div class="mt-16 mx-auto w-11/12 max-w-5xl">
      <div class="px-5 pb-5 rounded-3xl bg-secondary flex flex-col">
        <div class="py-5 flex flex-row items-center w-full">
          <h1 class="text-gray-100 truncate">{{ torrent.title }}</h1>
          <a @click="closeModal" class="ml-auto cursor-pointer text-gray-300 hover:text-white">
            <XIcon />
          </a>
        </div>

        <div v-if="isAdmin" class="pb-5">
          <button type="button" @click="deleteTorrent"
                  class="text-white bg-red-600 border-transparent shadow-sm button hover:bg-red-500">
            Delete torrent
          </button>
          <button type="button"
                  class="ml-2 text-white bg-blue-600 border-transparent shadow-sm button hover:bg-blue-500">
            Edit description
          </button>
        </div>

        <div class="p-4 bg-primary text-gray-300 rounded-3xl flex flex-col lg:flex-row justify-center items-center w-full overflow-auto">
          <div class="mb-4 lg:mb-0 flex flex-row">
            <div class="px-2 text-gray-300 text-sm">Seeders: <span class="text-green-500">{{ torrent.seeders }}</span></div>
            <div class="px-2 text-gray-300 text-sm">Leechers: <span class="text-red-500">{{ torrent.leechers }}</span></div>
          </div>
          <button type="button" @click="downloadTorrent"
                  class="ml-0 lg:ml-auto text-white bg-green-600 border-transparent shadow-sm button hover:bg-green-500">
            <DownloadIcon class="mr-2 -ml-1 w-5 h-5"/>
            Torrent file
          </button>
        </div>

        <h2 class="p-4 text-gray-300">Torrent information</h2>
        <div class="p-4 bg-primary text-sm text-gray-300 rounded-3xl flex flex-col w-full overflow-auto">
          <div><span class="font-bold">Infohash:</span> {{ torrent.info_hash }}</div>
          <div><span class="font-bold mt-1">Total size:</span> {{ fileSize(torrent.file_size) }}</div>
          <div><span class="font-bold mt-1">Upload date:</span> {{ new Date(torrent.upload_date * 1000).toLocaleString() }}</div>
          <div><span class="font-bold mt-1">Uploader:</span> {{ torrent.uploader }}</div>
        </div>

        <h2 class="p-4 text-gray-300">Torrent files</h2>
        <div class="p-4 bg-primary text-sm text-gray-300 rounded-3xl flex flex-col w-full overflow-auto">
          <div v-for="(file, i) in groupedFiles" :key="i">{{ file.name }} <span class="font-bold">({{ fileSize(file.length) }})</span></div>
        </div>

        <h2 class="p-4 text-gray-300">Torrent Description</h2>
        <div class="p-4 bg-primary text-gray-300 rounded-3xl flex flex-col w-full overflow-auto">
          <div v-html="compiledMarkdown" class="max-w-none prose-sm prose-blue"></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import MarkdownIt from 'markdown-it';
import {XIcon, DownloadIcon} from "@vue-hero-icons/outline";
import HttpService from "@/common/http-service";
import Vue from "vue";

export default {
  name: "TorrentDetail",
  components: {XIcon, DownloadIcon},
  data: () => ({
    prevRoute: null,
    torrent: {
      name: "",
      seeders: 0,
      leechers: 0,
      date: Date.now(),
      size: 0,
      uploader: "null",
      image: "",
      categories: [],
      files: [],
    },
    md: new MarkdownIt(),
  }),
  beforeRouteEnter(to, from, next) {
    next(vm => {
      // save previous route to go back to
      vm.prevRoute = from
    })
  },
  mounted() {
    document.body.classList.add("modal-open");
    this.getTorrent(this.$route.params.torrentId);
  },
  beforeDestroy() {
    document.body.classList.remove("modal-open");
  },
  methods: {
    closeModal() {
      // check if user was browsing or torrents or used a direct link
      if (this.prevRoute.name === 'TorrentList') {
        this.$router.go(-1);
      } else {
        this.$router.push('/torrents/');
      }
    },
    getTorrent(torrentId) {
      const self = this;
      HttpService.get(`/torrent/${torrentId}`, (res) => {
        this.torrent = res.data.data;
      }).catch(() => {
        self.closeModal();
      })
    },
    deleteTorrent() {
      const self = this;
      HttpService.delete(`/torrent/${this.torrent.torrent_id}`, () => {
        Vue.notify({
          title: 'Deleted',
          text: 'Torrent deleted successfully.',
          type: 'success',
        })
        self.closeModal();
      }).catch(() => {
        self.closeModal();
      })
    },
    downloadTorrent() {
      HttpService.getBlob(`/torrents/download/${this.torrent.torrent_id}`, (res) => {
        const url = window.URL.createObjectURL(new Blob([res.data]));
        const link = document.createElement('a');
        link.href = url;
        link.setAttribute('download', `${this.torrent.title}.torrent`);
        document.body.appendChild(link);
        link.click();
      });
    }
  },
  computed: {
    isAdmin() {
      return this.$store.getters.isAdministrator;
    },
    torrentId() {
      return this.$route.params.torrentId;
    },
    compiledMarkdown() {
      return this.md.render(this.torrent.description || "This torrent has no description.");
    },
    groupedFiles() {
      let files = [];

      if (this.torrent.files) {
        for (const file of this.torrent.files) {
          let filename = "";
          for (const [i, path] of file.path.entries()) {
            filename += path;
            if (i !== file.path.length - 1) {
              filename += "/"
            }
          }
          files.push({
            name: filename,
            length: file.length
          });
        }
      }

      return files;
    },
  }
}
</script>

<style scoped>
.button {
  @apply inline-flex justify-center px-4 py-2 text-sm font-medium rounded-md border shadow-sm;
}
</style>
