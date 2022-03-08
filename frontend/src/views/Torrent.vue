<template>
    <div class="flex flex-col">
      <div class="flex flex-row">
<!--        <div class="basis-1 lg:basis-1/3 pr-6">-->
<!--          <svg xmlns="http://www.w3.org/2000/svg" class="bg-black shadow-lg w-auto h-auto text-white/5 rounded-3xl" fill="none" viewBox="0 0 24 24" stroke="currentColor">-->
<!--            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 4v16M17 4v16M3 8h4m10 0h4M3 12h18M3 16h4m10 0h4M4 20h16a1 1 0 001-1V5a1 1 0 00-1-1H4a1 1 0 00-1 1v14a1 1 0 001 1z" />-->
<!--          </svg>-->
<!--        </div>-->
        <div class="w-full">
          <h1 class="py-2 text-xl font-semibold text-white truncate">{{ torrent.title }}</h1>
          <h2 class="font-semibold text-sm text-slate-400 uppercase">{{ torrent.info_hash }}</h2>

          <div class="py-4 flex flex-col flex-col-reverse lg:flex-row">
            <div class="px-1 py-1 w-full lg:w-2/3 flex flex-col lg:flex-row flex-row border border-slate-800 rounded-md">
              <div class="px-3 w-full lg:w-1/2 flex flex-col justify-start">
                <div class="detail">Total size:<span class="value">{{ fileSize(torrent.file_size) }}</span></div>
                <div class="detail">Upload Date:<span class="value">{{ new Date(torrent.upload_date * 1000).toLocaleString() }}</span></div>
                <div class="detail lg:border-none">Uploader:<span class="value">{{ torrent.uploader }}</span></div>
              </div>
              <div class="px-3 w-full lg:w-1/2 hidden lg:flex flex-col justify-start">
                <div class="detail">Downloads:<span class="value italic">coming soon</span></div>
                <div class="detail">Comments:<span class="value italic">coming soon</span></div>
                <div class="detail border-none">Last Updated:<span class="value italic">coming soon</span></div>
              </div>
            </div>
            <div class="px-0 lg:pl-3 w-full mb-2 lg:mb-0 lg:w-1/3 flex flex-col items-center">
              <div class="w-full flex flex-row items-center">
                <div class="status">Seeders: <span class="ml-auto text-green-500">{{ torrent.seeders }}</span></div>
                <div class="ml-2 status">Leechers: <span class="ml-auto text-red-500">{{ torrent.leechers }}</span></div>
              </div>
              <button type="button" @click="downloadTorrent"
                      class="mt-2 px-3 py-1.5 w-full flex flex-row justify-center text-sm text-white text-center bg-green-600 border border-green-600 rounded-md transition duration-200 hover:shadow-lg hover:shadow-green-600/25">
                <DownloadIcon class="justify-self-start mr-2 w-5 h-5"/>
                <span>Torrent Download</span>
              </button>
              <a type="button" :href="torrent.magnet_link"
                 class="mt-2 px-3 py-1.5 w-full flex flex-row justify-center text-sm text-white text-center bg-red-600 border border-red-600 rounded-md transition duration-200 hover:shadow-lg hover:shadow-red-600/25">
                <svg xmlns="http://www.w3.org/2000/svg" class="mr-2 h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                <span>Magnet Download</span>
              </a>
            </div>
          </div>

          <div v-if="isAdmin">
            <h2 class="py-3 text-slate-400">Admin Tools</h2>
            <div class="flex flex-row items-center">
              <button type="button" @click="deleteTorrent"
                      class="px-3 py-2 text-sm text-white bg-red-600 rounded-md hover:shadow-lg hover:shadow-red-600/25 transition duration-200">
                Delete Torrent
              </button>
              <button type="button" @click="banUser(torrent.uploader)"
                      class="ml-2 px-3 py-2 text-sm text-white bg-red-600 rounded-md hover:shadow-lg hover:shadow-red-600/25 transition duration-200">
                Ban Uploader
              </button>
            </div>
          </div>
        </div>
      </div>

      <div>
        <div class="py-3 border-b border-slate-200/5"></div>
        <h2 class="py-3 text-slate-400">Torrent Description</h2>
        <!--        <button v-if="isAdmin || isOwner" type="button"-->
        <!--                class="mb-2 text-white bg-blue-600 border-transparent shadow-sm button hover:bg-blue-500">-->
        <!--          Edit description-->
        <!--        </button>-->
        <div class="flex flex-col w-full text-slate-400 overflow-auto">
          <div v-html="compiledMarkdown" class="max-w-none prose-sm prose-blue"></div>
        </div>
      </div>

      <div>
        <div class="py-3 border-b border-slate-200/5"></div>
        <h2 class="py-3 text-slate-400">Torrent Files</h2>
        <div class="text-sm flex flex-col w-full text-slate-400 overflow-auto">
          <div v-for="(file, i) in groupedFiles" :key="i">- {{ file.name }} <span class="font-bold">({{ fileSize(file.length) }})</span></div>
        </div>
      </div>

    </div>
</template>

<script>
import MarkdownIt from 'markdown-it';
import {XIcon, DownloadIcon, ChevronLeftIcon} from "@vue-hero-icons/outline";
import HttpService from "@/common/http-service";
import Vue from "vue";
import {mapState} from "vuex";
import Breadcrumb from "../components/Breadcrumb.vue";

export default {
  name: "TorrentDetail",
  components: {Breadcrumb, XIcon, DownloadIcon, ChevronLeftIcon},
  data: () => ({
    prevRoute: null,
    torrent: {
      torrent_id: 0,
      uploader: "",
      info_hash: "",
      title: "",
      description: "",
      category_id: 0,
      upload_date: Date.now(),
      file_size: 0,
      seeders: 0,
      leechers: 0,
      files: [],
      trackers: [],
      magnet_link: "",
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
      HttpService.delete(`/torrent/${this.torrent.torrent_id}`, {}, () => {
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
      HttpService.getBlob(`/torrent/download/${this.torrent.torrent_id}`, (res) => {
        const url = window.URL.createObjectURL(new Blob([res.data]));
        const link = document.createElement('a');
        link.href = url;
        link.setAttribute('download', `${this.torrent.title}.torrent`);
        document.body.appendChild(link);
        link.click();
      });
    },
    banUser(user) {
      const self = this;
      HttpService.delete(`/user/ban/${user}`, {}, () => {
        Vue.notify({
          title: 'Banned!',
          text: 'User banned successfully.',
          type: 'success',
        })
        self.closeModal();
      }).catch(() => {
        self.closeModal();
      })
    },
  },
  computed: {
    ...mapState({
      user: state => state.auth.user,
    }),
    isAdmin() {
      return this.$store.getters.isAdministrator;
    },
    isOwner() {
      return this.user.username === this.torrent.uploader;
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

.status {
  @apply px-2 py-1.5 w-1/2 flex flex-row bg-slate-800 text-slate-400 capitalize border border-slate-800 rounded-md text-sm uppercase;
}

.detail {
  @apply py-2 flex flex-row font-semibold text-sm text-slate-200 border-b border-slate-800;
}

.detail > .value {
  @apply ml-auto text-slate-400;
}
</style>
