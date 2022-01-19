<template>
  <div class="flex flex-col w-full">
    <div class="max-w-5xl">
      <h1 class="view-title text-white">Upload Torrent</h1>
        <form class="space-y-4">
          <div>
            <label for="title" class="block font-medium text-gray-700">
              Title
            </label>
            <div class="mt-1">
              <input id="title" name="title" type="text" v-model="form.title"
                     class="py-2 px-4 appearance-none w-full text-gray-700 rounded-md leading-tight focus:outline-none">
            </div>
          </div>

          <div>
            <label for="description" class="block font-medium text-gray-700">
              Description
            </label>
            <div class="mt-1">
              <textarea id="description" name="description" rows="8" v-model="form.description"
                        class="py-2 px-4 appearance-none w-full text-gray-700 rounded-md leading-tight focus:outline-none"></textarea>
            </div>
            <p class="mt-2 text-sm text-gray-500">Markdown is supported.</p>
          </div>

          <label class="block font-medium text-gray-700">
            Category
          </label>
          <CategorySelect class="mt-0" :category.sync="form.category"/>
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
            <button @click="closeModal" type="button" class="px-3 py-2 flex flex-row text-sm text-red-200 bg-red-600 hover:text-white hover:bg-red-500 rounded-md shadow-lg shadow-red-600/50 duration-200">
              Cancel
            </button>
            <button :disabled="!formValid || uploading" @click="submitForm" type="button" class="ml-2 px-3 py-2 flex flex-row text-sm text-green-200 bg-green-800 hover:bg-green-700 rounded-md shadow-lg shadow-green-700/50 disabled:shadow-none disabled:text-gray-100 disabled:bg-gray-400 disabled:hover:bg-gray-400">
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

export default {
  name: "TorrentDetail",
  components: {CategorySelect, FileUpload, XIcon},
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
</style>
