<template>
  <div>
    <div v-if="torrent.info_hash" class="flex flex-col">
      <div class="flex flex-row">
<!--        <div class="basis-1 lg:basis-1/3 pr-6">-->
<!--          <svg xmlns="http://www.w3.org/2000/svg" class="bg-black shadow-lg w-auto h-auto text-white/5 rounded-3xl" fill="none" viewBox="0 0 24 24" stroke="currentColor">-->
<!--            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 4v16M17 4v16M3 8h4m10 0h4M3 12h18M3 16h4m10 0h4M4 20h16a1 1 0 001-1V5a1 1 0 00-1-1H4a1 1 0 00-1 1v14a1 1 0 001 1z" />-->
<!--          </svg>-->
<!--        </div>-->
        <div class="w-full overflow-hidden">
          <div class="flex flex-row items-center">
            <input v-if="editingTitle" v-model="newTitle" class="mr-2" type="text">
            <h1 v-else class="py-2 text-xl font-semibold text-slate-200 truncate">{{ torrent.title }}</h1>
            <button v-if="torrent.uploader === user.username && !editingTitle" @click="editTitle" class="ml-auto edit">Edit</button>
            <div v-if="torrent.uploader === user.username && editingTitle" class="ml-auto flex flex-row space-x-2">
              <button @click="() => (editingTitle = false)" class="ml-auto edit">Cancel</button>
              <button :disabled="newTitle === torrent.title" @click="saveTitle" class="ml-auto edit">Save</button>
            </div>
          </div>
          <h2 class="font-semibold text-xs lg:text-sm text-slate-400 uppercase">{{ torrent.info_hash }}</h2>

          <div class="py-4 flex flex-col flex-col-reverse lg:flex-row">
            <div class="px-1 py-1 w-full lg:w-2/3 flex flex-col lg:flex-row flex-row bg-slate-800/50 rounded-md">
              <div class="px-3 w-full lg:w-1/2 flex flex-col justify-start">
                <div class="detail">Total size:<span class="value">{{ fileSize(torrent.file_size) }}</span></div>
                <div class="detail">Upload Date:<span class="value">{{ new Date(torrent.upload_date * 1000).toLocaleString() }}</span></div>
                <div class="detail lg:border-none">Uploader:<span class="value">{{ torrent.uploader }}</span></div>
              </div>
              <div class="px-3 w-full lg:w-1/2 flex flex-col justify-start">
                <div class="detail">Downloads:<span class="value italic">coming soon</span></div>
                <div class="detail">Comments:<span class="value italic">coming soon</span></div>
                <div class="detail border-none">Last Updated:<span class="value italic">coming soon</span></div>
              </div>
            </div>
            <div class="px-0 lg:pl-3 w-full mb-4 lg:mb-0 lg:w-1/3 flex flex-col items-center">
              <div class="w-full flex flex-row items-center">
                <div class="status">Seeders: <span class="ml-auto text-green-500">{{ torrent.seeders }}</span></div>
                <div class="ml-2 status">Leechers: <span class="ml-auto text-red-500">{{ torrent.leechers }}</span></div>
              </div>
              <button v-if="showDownloadButtons" type="button" @click="downloadTorrent"
                      class="mt-2 px-3 py-1.5 w-full flex flex-row justify-center text-sm text-white text-center bg-green-600 border border-green-600 rounded-md transition duration-200 hover:shadow-lg hover:shadow-green-600/25">
                <DownloadIcon class="justify-self-start mr-2 w-5 h-5"/>
                <span>Torrent Download</span>
              </button>
              <button v-else type="button" @click="$store.dispatch('openAuthModal')"
                      class="mt-2 px-3 py-1.5 w-full flex flex-row justify-center text-sm text-white text-center bg-sky-500 border border-sky-500 rounded-md transition duration-200">
                <DownloadIcon class="justify-self-start mr-2 w-5 h-5"/>
                <span>Sign in to download</span>
              </button>
              <a v-if="showDownloadButtons" type="button" :href="torrent.magnet_link"
                 class="mt-2 px-3 py-1.5 w-full flex flex-row justify-center text-sm text-white text-center bg-red-600 border border-red-600 rounded-md transition duration-200 hover:shadow-lg hover:shadow-red-600/25">
                <svg xmlns="http://www.w3.org/2000/svg" class="mr-2 h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                <span>Magnet Download</span>
              </a>
              <button v-else type="button" @click="$store.dispatch('openAuthModal')"
                      class="mt-2 px-3 py-1.5 w-full flex flex-row justify-center text-sm text-white text-center bg-sky-500 border border-sky-500 rounded-md transition duration-200">
                <svg xmlns="http://www.w3.org/2000/svg" class="mr-2 h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                <span>Sign in to download</span>
              </button>
            </div>
          </div>

          <div v-if="isAdmin">
            <h2 class="section">Admin Tools</h2>
            <div class="py-3 border-t border-slate-200/5"></div>
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
        <div v-if="isAdmin" class="py-3 border-b border-slate-200/5"></div>
        <div class="flex flex-row items-center">
          <h2 class="section">Torrent Description</h2>
          <button v-if="torrent.uploader === user.username && !editingDescription" @click="editDescription" class="ml-auto edit">Edit</button>
          <div v-if="torrent.uploader === user.username && editingDescription" class="ml-auto flex flex-row space-x-2">
            <button @click="() => (editingDescription = false)" class="ml-auto edit">Cancel</button>
            <button :disabled="newDescription === torrent.description" @click="saveDescription" class="ml-auto edit">Save</button>
          </div>
        </div>
        <div class="py-3 border-t border-slate-200/5"></div>
        <textarea v-if="editingDescription" rows="8" v-model="newDescription" class="input"></textarea>
        <h2 v-if="editingDescription" class="section">Markdown Preview</h2>
        <MarkdownItVue v-if="editingDescription" :content="newDescription" class="torrust-md px-4 py-4 max-h-64 overflow-auto md-body max-w-none prose-sm prose-blue bg-slate-800/50 rounded-md" />
        <MarkdownItVue v-if="!editingDescription && torrent.description" :content="torrent.description" class="md-body max-w-none prose-sm prose-blue" />
        <span v-if="!editingDescription && !torrent.description" class="text-slate-400 italic">Empty</span>
      </div>

      <div>
        <div class="py-3 border-b border-slate-200/5"></div>
        <h2 class="section">Torrent Files</h2>
        <div class="py-3 border-t border-slate-200/5"></div>
        <div class="text-sm flex flex-col w-full text-slate-400 overflow-auto">
          <div v-for="(file, i) in groupedFiles" :key="i">- {{ file.name }} <span class="font-bold">({{ fileSize(file.length) }})</span></div>
        </div>
      </div>
    </div>
    <h1 v-else-if="!loading" class="py-6 text-slate-600 italic">Torrent not found.</h1>
    <div v-else class="flex flex-row text-slate-400 items-center" disabled>
      <svg class="animate-spin h-5 w-5 mr-3 text-sky-500" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
      </svg>
      Loading torrent information...
    </div>
  </div>
</template>

<script>
import {XIcon, DownloadIcon, ChevronLeftIcon} from "@vue-hero-icons/outline";
import HttpService from "@/common/http-service";
import Vue from "vue";
import {mapState} from "vuex";
import Breadcrumb from "../components/Breadcrumb.vue";
import MarkdownItVue from "markdown-it-vue";
import 'markdown-it-vue/dist/markdown-it-vue.css';

export default {
  name: "TorrentDetail",
  components: {Breadcrumb, XIcon, DownloadIcon, ChevronLeftIcon, MarkdownItVue},
  data: () => ({
    loading: true,
    editingTitle: false,
    newTitle: "",
    editingDescription: false,
    newDescription: "",
    prevRoute: null,
    torrent: {},
  }),
  beforeRouteEnter(to, from, next) {
    next(vm => {
      // save previous route to go back to
      vm.prevRoute = from
    })
  },
  mounted() {
    document.body.classList.add("modal-open");
    this.getTorrent(this.$route.params.torrentId, this.$route.params.download === "download" || this.$route.params.title === "download");
  },
  beforeDestroy() {
    document.body.classList.remove("modal-open");
  },
  methods: {
    getTorrent(torrentId, download) {
      this.loading = true;
      HttpService.get(`/torrent/${torrentId}`, (res) => {
        this.torrent = res.data.data;
        this.loading = false;
        if(download){
          this.downloadTorrent();
        }
        this.updateUrlWithTitle();
      }).catch(() => {
        this.loading = false;
      })
    },
    updateUrlWithTitle() {
      if (this.$route.params.title !== this.torrent.title) {
        // Retrieve current params
        const currentParams = this.$router.currentRoute.params;
        // Create new params object
        const mergedParams = { ...currentParams, title: this.torrent.title };
        // When router is not supplied path or name,
        // it simply tries to update current route with new params or query
        // Almost everything is optional.
        this.$router.push({ params: mergedParams });
      }
    },
    deleteTorrent() {
      HttpService.delete(`/torrent/${this.torrent.torrent_id}`, {}, () => {
        Vue.notify({
          title: 'Deleted',
          text: 'Torrent deleted successfully.',
          type: 'success',
        })
        this.torrent = {};
      }).catch((e) => {
        console.log(e);
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
    editTitle() {
      this.newTitle = this.torrent.title;
      this.editingTitle = true;
    },
    saveTitle() {
      HttpService.put(`/torrent/${this.torrent.torrent_id}`, { title: this.newTitle }, (res) => {
        this.editingTitle = false;
        Vue.notify({
          title: 'Updated',
          text: 'Torrent updated successfully.',
          type: 'success',
        })
        this.torrent.title = res.data.data.title;
      }).catch(() => {})
    },
    editDescription() {
      this.newDescription = this.torrent.description;
      this.editingDescription = true;
    },
    saveDescription() {
      HttpService.put(`/torrent/${this.torrent.torrent_id}`, { description: this.newDescription }, (res) => {
        this.editingDescription = false;
        Vue.notify({
          title: 'Updated',
          text: 'Torrent updated successfully.',
          type: 'success',
        })
        this.torrent.description = res.data.data.description;
      }).catch(() => {})
    }
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
    showDownloadButtons() {
      return this.$store.getters.isLoggedIn || this.$store.state.publicSettings.tracker_mode === "Public" || this.$store.state.publicSettings.tracker_mode === "Whitelisted"
    }
  }
}
</script>

<style scoped>
.status {
  @apply px-2 py-1.5 w-1/2 flex flex-row bg-slate-800/50 text-slate-200 capitalize border border-transparent rounded-md text-sm uppercase;
}

.detail {
  @apply py-2 flex flex-row text-sm text-slate-200 border-b border-slate-700;
}

.detail > .value {
  @apply ml-auto text-slate-400;
}

h2.section {
  @apply py-3 font-semibold text-xl text-slate-400;
}

button.edit {
  @apply px-4 py-1.5 rounded-md border border-slate-800 text-sm text-slate-400 flex items-center relative cursor-pointer transition duration-200 hover:text-slate-200 hover:border-slate-200 disabled:bg-slate-700 disabled:border-slate-700 disabled:text-slate-400;
}

textarea.input, input {
  @apply py-2 px-4 bg-slate-800/50 appearance-none w-full text-slate-200 rounded-md leading-tight focus:outline-none;
}
</style>

<style>
.markdown-body {
  @apply text-slate-400;
}

.markdown-body a {
  @apply text-sky-400;
}

.markdown-body blockquote {
  @apply text-slate-400 border-slate-600;
}

.markdown-body hr {
  @apply bg-slate-200/50;
}

.markdown-body h1, .markdown-body h2, .markdown-body h3, .markdown-body h4, .markdown-body h5, .markdown-body h6 {
  @apply border-slate-200/50;
}

.markdown-body .highlight pre, .markdown-body pre {
  @apply bg-slate-800 text-slate-400 rounded-md;
}

.markdown-body table tr, .markdown-body table td, .markdown-body table th {
  @apply bg-slate-800 border-slate-700;
}
</style>
