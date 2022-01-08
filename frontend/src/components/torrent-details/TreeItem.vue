<template>
  <li>
    <p
        class="inline-flex"
        :class="{'text-blue-500': isFolder, 'cursor-pointer': isFolder}"
        @click="toggle"
    >
      <FolderIcon v-if="isFolder" size="20" class="mr-0.5 text-gray-500"/>
      {{ file.name + ` (${fileSize(file.length)})` }}
    </p>
    <ul v-show="isOpen" v-if="isFolder">
      <TreeItem v-for="(child, index) in file.children" :key="index" :file="child" :level="level+1" />
    </ul>
  </li>
</template>

<script>
import { FolderIcon } from "@vue-hero-icons/solid"

export default {
  name: "TreeItem",
  components: {FolderIcon},
  props: {
    level: Number,
    file: Object
  },
  data: () => ({
    isOpen: true,
  }),
  mounted() {
    if (this.level >= 3) {
      this.isOpen = false;
    }
  },
  computed: {
    isFolder() {
      return this.file.children && this.file.path.children;
    }
  },
  methods: {
    toggle() {
      if (!this.isFolder) {
        return;
      }

      this.isOpen = !this.isOpen;
    }
  }
}
</script>

<style scoped>

</style>
