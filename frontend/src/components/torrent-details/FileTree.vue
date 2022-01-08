<template>
  <div class="mt-8 xl:mt-10">
    <div class="divide-y divide-gray-200">
      <div class="pb-4">
        <h2 id="activity-title" class="text-lg font-medium text-gray-900">Files</h2>
      </div>
      <div class="pt-6">
        <ul class="file-tree pb-4 overflow-y-hidden overflow-x-auto h-auto">
          <li v-for="(file, index) in files" :key="index">
            {{ file.name + ` (${fileSize(file.length)})` }}
          </li>
<!--          <TreeItem v-for="(file, index) in files" :key="index" :file="file" :level="1" />-->
        </ul>
      </div>
    </div>
  </div>
</template>

<script>
//import TreeItem from "./TreeItem";
export default {
  name: "FileTree",
  components: {},
  props: {
    files: Array,
  }
}
</script>

<style>
ul.file-tree, ul.file-tree ul {
  list-style: none;
  margin: 0;
  padding: 0;
}

ul.file-tree ul {
  margin-left: 10px;
}

ul.file-tree li {
  margin: 0 7px;
  padding: 0;
  line-height: 20px;
  @apply text-gray-900;
}

ul.file-tree ul li {
  border-left: 1px solid rgb(100, 100, 100);
}

ul.file-tree li:before {
  position: relative;
  top: -0.3em;
  height: 1em;
  width: 10px;
  color: white;
  content: "";
  display: inline-block;
}
</style>
