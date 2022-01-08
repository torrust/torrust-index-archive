<template>
  <div
      class="flex overflow-auto fixed top-0 left-0 z-30 flex-col justify-center w-full h-screen bg-black bg-opacity-50"
      @click.self="closeModal"
  >
    <div class="mx-auto mt-8 w-11/12 h-full sm:w-full sm:max-w-5xl">

      <div
          class="relative px-4 py-12 mx-auto mb-4 max-w-3xl bg-white shadow xl:py-8 sm:px-6 lg:px-8 xl:max-w-2xl rounded-lg">
        <button
            @click="closeModal"
            class="absolute top-0 right-0">
          <XIcon class="m-2 xl:m-5"/>
        </button>

        <div>
          <h1 class="pb-2 mb-4 text-xl font-bold text-gray-900 break-all border-b md:text-2xl">New torrent</h1>

          <form class="space-y-4">
            <div>
              <label for="title" class="block font-medium text-gray-700">
                Title
              </label>
              <div class="mt-1">
                <input id="title" name="title" type="text" v-model="form.title"
                       class="block w-full rounded-md border-gray-300 shadow-sm  focus:border-primary-500 sm:text-sm">
              </div>
            </div>

            <div>
              <label for="description" class="block font-medium text-gray-700">
                Description
              </label>
              <div class="mt-1">
                <textarea id="description" name="description" rows="8" v-model="form.description"
                          class="block w-full rounded-md border border-gray-300 shadow-sm  focus:border-primary-500 sm:text-sm"></textarea>
              </div>
              <p class="mt-2 text-sm text-gray-500">Markdown is supported.</p>
            </div>

            <CategorySelect :category.sync="form.category"/>
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

            <div class="py-3 sm:flex sm:flex-row-reverse">
              <button :disabled="!formValid || uploading" @click="submitForm" type="button" class="button text-white"
                      :class="[{'bg-gray-500': !formValid || uploading}, {'hover:bg-gray-500': !formValid || uploading}, {'bg-green-600': formValid && !uploading}, {'hover:bg-green-700': formValid && !uploading}]"
              >
                <svg v-if="uploading" class="animate-spin h-5 w-5 mr-3" viewBox="0 0 24 24">
                  <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                  <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                Upload torrent
              </button>
              <button @click="closeModal" type="button" class="mt-3 button bg-white border border-gray-300 text-gray-700 hover:bg-gray-50">
                Cancel
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import {XIcon} from "@vue-hero-icons/solid";
import FileUpload from "@/components/torrent-upload/FileUpload";
import CategorySelect from "@/components/torrent-upload/CategorySelect";
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
          this.$router.push(`/categories/${this.form.category}/${res.data.data.torrent_id}`);
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
.button {
  @apply w-full inline-flex justify-center rounded-lg shadow-sm px-4 py-2 text-base font-medium sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm;
}
</style>
