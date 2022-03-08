<template>
  <div class="mt-10 flex flex-col w-full">
    <div class="w-full">
      <form class="space-y-4">
        <div>
          <label for="title" class="block font-medium text-gray-700">
            Title
          </label>
          <div class="mt-1">
            <input id="title" name="title" type="text" v-model="form.title" class="input">
          </div>
        </div>

        <div>
          <label for="description" class="block font-medium text-gray-700">
            Description (Markdown supported)
          </label>
          <div class="mt-1">
            <textarea id="description" name="description" rows="8" v-model="form.description"
                      class="input"></textarea>
          </div>
        </div>

        <div v-if="form.description">
          <label>Markdown Preview</label>
          <MarkdownItVue :content="form.description" class="mt-1 px-4 py-4 max-h-64 overflow-auto md-body max-w-none prose-sm prose-blue bg-slate-800/50 rounded-md" />
        </div>

        <div>
          <label class="block font-medium text-gray-700">
            Category
          </label>
          <CategorySelect class="py-1" :category.sync="form.category"/>
        </div>
<!--            <div>-->
<!--              <label class="block font-medium text-gray-700">-->
<!--                Cover picture-->
<!--              </label>-->
<!--              <FileUpload type="image" accept="image/*" />-->
<!--            </div>-->

        <div>
          <label class="block font-medium text-gray-700">
            Torrent
          </label>
          <FileUpload @onChange="setFile" sub-title="Only .torrent files allowed" accept=".torrent" />
        </div>

        <div class="py-3 flex flex-row justify-end">
          <button @click="closeModal" type="button" class="px-3 py-2 flex flex-row text-sm text-red-200 bg-red-600 rounded-md hover:text-white hover:bg-red-500 hover:shadow-lg shadow-red-600/50 transition duration-200">
            Cancel
          </button>
          <button :disabled="!formValid || uploading" @click="submitForm" type="button" class="ml-2 px-3 py-2 flex flex-row bg-sky-500 text-sm text-white rounded-md transition duration-200 hover:shadow-lg hover:shadow-sky-500/25 disabled:shadow-none disabled:text-gray-100 disabled:bg-gray-400 disabled:hover:bg-gray-400">
            <svg v-if="uploading" class="animate-spin h-5 w-5 mr-3" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            Upload torrent
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script>
import {XIcon} from "@vue-hero-icons/solid";
import FileUpload from "@/components/torrent-upload/FileUpload.vue";
import CategorySelect from "@/components/torrent-upload/CategorySelect.vue";
import HttpService from "@/common/http-service";
import MarkdownItVue from "markdown-it-vue";
import 'markdown-it-vue/dist/markdown-it-vue.css';

export default {
  name: "TorrentDetail",
  components: {CategorySelect, FileUpload, XIcon, MarkdownItVue},
  data: () => ({
    cover: null,
    torrent: null,
    uploading: false,
    form: {
      title: "",
      category: "",
      description: "",
      torrentFile: ""
    }
  }),
  mounted() {
    document.body.classList.add("modal-open");
  },
  beforeDestroy() {
    document.body.classList.remove("modal-open");
  },
  methods: {
    closeModal() {
      this.$router.go(-1);
    },
    setFile(file) {
      [this.form.torrentFile] = file;
    },
    submitForm() {
      if (this.formValid && !this.uploading) {
        this.uploading = true;
        let formData = new FormData();
        formData.append('title', this.form.title);
        formData.append('description', this.form.description);
        formData.append('category', this.form.category);
        formData.append('torrent', this.form.torrentFile);
        HttpService.post(`/torrent/upload`, formData, (res) => {
          this.$router.push(`/torrent/${res.data.data.torrent_id}`);
        }).finally(() => {
          this.uploading = false;
        })
      }
    },
  },
  computed: {
    formValid() {
      return !!this.form.title && !!this.form.category && !!this.form.torrentFile;
    }
  },
}
</script>

<style scoped>
label {
  @apply text-left text-xs font-medium uppercase tracking-wider text-slate-400 hover:text-slate-200;
}

.input {
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
