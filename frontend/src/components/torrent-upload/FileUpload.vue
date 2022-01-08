<template>
  <div class="flex justify-center px-6 pt-5 pb-6 mt-1 rounded-md border-2 border-gray-300 border-dashed"
       :class="{'border-primary-200': filedrag, 'bg-primary-200': filedrag, 'border-red-500': error}"
       @dragover="dragover"
       @dragleave="dragleave"
       @drop="dropFile"
  >

    <input hidden id="torrent-upload" name="torrent-upload" type="file" class="sr-only" :accept="accept"
           ref="file" @change="onChange">

    <div class="space-y-1 text-center" v-if="filelist.length === 0">
      <DocumentAddIcon v-if="type === 'file'" class="mx-auto w-12 h-12 text-gray-400"/>
      <PhotographIcon v-else-if="type === 'image'" class="mx-auto w-12 h-12 text-gray-400"/>
      <div class="flex text-sm text-gray-600">
        <a @click="$refs.file.click()"
           class="relative font-medium text-blue-600 rounded-md cursor-pointer hover:text-primary-500 focus-within:outline-none focus-within:ring-2 focus-within:ring-offset-2 focus-within:ring-primary-500">
          <span>Upload a file</span>
        </a>
        <p class="pl-1">or drag and drop</p>
      </div>
      <p class="text-xs text-gray-500">
        {{ subTitle }}
      </p>
    </div>

    <ul v-if="this.filelist.length">
      <li class="flex flex-row w-full justify-between items-center gap-2 text-sm" v-for="file in filelist" :key="file.name">
        <img v-if="type === 'image'" :src="fileUrl(file)" :alt="file.name"
             class="h-20 w-auto"
        />
        <p class="truncate">{{ file.name }}</p>
        <a @click="$refs.file.click()"
           class="relative font-medium text-blue-600 rounded-md cursor-pointer hover:text-primary-500 focus-within:outline-none focus-within:ring-2 focus-within:ring-offset-2 focus-within:ring-primary-500">
          <span>Upload a file</span>
        </a>
      </li>
    </ul>
  </div>
</template>

<script>
import {DocumentAddIcon, PhotographIcon} from "@vue-hero-icons/outline";

export default {
  name: "FileUpload",
  components: {DocumentAddIcon, PhotographIcon},
  props: {
    type: {
      type: String,
      default: () => 'file',
      validator: (value) => {
        return ['file', 'image'].indexOf(value) !== -1;
      }
    },
    accept: String,
    subTitle: String,
    error: Boolean,
  },
  data: () => ({
    filedrag: false,
    filelist: [],
  }),
  methods: {
    dragover(event) {
      event.preventDefault();
      this.filedrag = true;
    },
    dragleave() {
      this.filedrag = false;
    },
    dropFile(event) {
      event.preventDefault();
      this.$refs.file.files = event.dataTransfer.files;
      this.onChange();

      this.filedrag = false;
    },
    onChange() {
      this.filelist = [...this.$refs.file.files];
      this.$emit('onChange', [...this.$refs.file.files]);
    },
    fileUrl(file) {
      return URL.createObjectURL(file);
    }
  },
}
</script>

<style scoped>
.button {
  @apply w-full inline-flex justify-center rounded-md shadow-sm px-4 py-2 text-base font-medium sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm;
}
</style>
