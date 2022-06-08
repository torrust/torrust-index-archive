<template>
  <div>
    <div v-if="page.route" class="flex flex-col">
        
	<div v-if="isAdmin">
	  <h2 class="section">Admin Tools</h2>
	 <!-- <div class="section border-t border-slate-200/5"></div> -->
	  <div class="py-3 flex-row items-center">
	   <!-- <div class="flex flex-row items-center">
	     <button type="button" @click="deletePage"
		class="px-3 py-2 text-sm text-white bg-red-600 rounded-md hover:shadow-lg hover: shadow-ed-600/25 transition duration-200">
		Delete Page
	     </button>
	   </div>
-->
	   <div class="flex flex-row ml-auto space-x-2">
	     <button type="button" @click="deletePage"
		class="ml-auto px-3 py-2 text-sm text-white bg-red-600 rounded-md hover:shadow-lg hover: shadow-ed-600/25 transition duration-200">
		Delete Page
	     </button>
	     <button v-if="!editingDescription" class="editDescription ml-auto edit" @click="editDescription">Edit</button>
	     <button v-if="editingDescription" @click="() => (editingDescription = false)" class="ml-auto edit">Cancel</button>
	     <button v-if="editingDescription" :disabled="(newDescription === page.descriptioni)" @click="saveDescription" class="ml-auto edit">Save</button>
	   </div>
	</div>
	</div>

	<!-- <div v-if="isAdmin" class="py-3 border-b border-slate-200/5"></div>
        <div class="py-3 border-t border-slate-200/5"></div> --> 
        <textarea v-if="editingDescription" rows="8" v-model="newDescription" class="input"></textarea>
        <h2 v-if="editingDescription" class="section">Markdown Preview</h2>
        <MarkdownItVue v-if="editingDescription" :content="newDescription" class="px-4 py-4 max-h-64 overflow-auto md-body max-w-none prose-sm prose-blue bg-slate-800/50 rounded-md" />
        <MarkdownItVue v-if="!editingDescription && page.description" :content="page.description" class="md-body max-w-none prose-sm prose-blue" />
        <span v-if="!editingDescription && !page.description" class="text-slate-400 italic">Empty</span>
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
  name: "Page",
  components: {Breadcrumb, XIcon, DownloadIcon, ChevronLeftIcon, MarkdownItVue},
  data: () => ({
    page: {title: "cats", route: "cats"},
    loading: true,
    editingDescription: false,
    newDescription: '',
    prevRoute: null,
  }),
  beforeRouteEnter(to, from, next) {
    next(vm => {
      // save previous route to go back to
      vm.prevRoute = from
    })
  },
  mounted() {
    document.body.classList.add("modal-open");
    this.getPage(this.$route.params.page);
  },
  beforeDestroy() {
    document.body.classList.remove("modal-open");
  },
  methods: {
    getPage(route) {
      this.loading = true;
      HttpService.get(`/page/${route}`, (res) => {
        this.page = res.data.data;
        this.loading = false;
        this.updateUrlWithTitle();
      }).catch(() => {
        this.loading = false;
      })
    },
    updateUrlWithTitle() {
      if (this.$route.params.title !== this.page.title) {
        // Retrieve current params
        const currentParams = this.$router.currentRoute.params;
        // Create new params object
        const mergedParams = { ...currentParams, title: this.page.title };
        // When router is not supplied path or name,
        // it simply tries to update current route with new params or query
        // Almost everything is optional.
        this.$router.push({ params: mergedParams });
      }
    },
    deletePage() {
      HttpService.delete(`/page`, { route: this.page.route }, () => {
        Vue.notify({
          title: 'Deleted',
          text: 'Page deleted successfully.',
          type: 'success',
        })
        this.page = {};
      }).catch((e) => {
        console.log(e);
      })
    },
    editDescription() {
      this.newDescription = this.page.description;
      this.editingDescription = true;
    },
    saveDescription() {
      HttpService.put(`/page`, { route: this.page.route, description: this.newDescription }, (res) => {
        this.editingDescription = false;
        Vue.notify({
          title: 'Updated',
          text: 'Torrent updated successfully.',
          type: 'success',
        })
        this.page.description = res.data.data.description;
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
  }
}
</script>

<style>
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

textarea.input {
  @apply py-2 px-4 bg-slate-800/50 appearance-none w-full text-slate-200 rounded-md leading-tight focus:outline-none;
}

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
