<template>
  <nav class="text-sm font-semibold mb-6" aria-label="Breadcrumb">
    <ol class="list-none p-0 inline-flex">
      <li class="flex items-center text-gray-500">
        <router-link to="/">Home</router-link>
      </li>
      <li v-for="(crumb, index) in crumbs" :key="index" class="flex items-center text-gray-500">
        <svg class="fill-current w-3 h-3 mx-3" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 512"><path d="M285.476 272.971L91.132 467.314c-9.373 9.373-24.569 9.373-33.941 0l-22.667-22.667c-9.357-9.357-9.375-24.522-.04-33.901L188.505 256 34.484 101.255c-9.335-9.379-9.317-24.544.04-33.901l22.667-22.667c9.373-9.373 24.569-9.373 33.941 0L285.475 239.03c9.373 9.372 9.373 24.568.001 33.941z"/></svg>
        <router-link
            :to="crumb.to" :class="{ 'text-white': index === crumbs.length - 1 }"
        >
          {{ titleCase(crumb.text) }}
        </router-link>
      </li>
    </ol>
  </nav>
</template>

<script>
export default {
  name: "Breadcrumb",
  computed: {
    crumbs: function() {
      let pathArray = this.$route.path.split("/")
      pathArray.shift()
      let breadcrumbs = pathArray.reduce((breadcrumbArray, path, idx) => {
        breadcrumbArray.push({
          path: path,
          to: breadcrumbArray[idx - 1]
              ? "/" + breadcrumbArray[idx - 1].path + "/" + path
              : "/" + path,
          text: this.$route.matched[idx].meta.breadCrumb || path,
        });
        return breadcrumbArray;
      }, [])
      return breadcrumbs;
    }
  }
}
</script>

<style scoped>

</style>
