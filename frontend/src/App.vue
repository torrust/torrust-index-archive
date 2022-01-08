<template>
  <div id="app">
    <Toast/>
    <router-view />
  </div>
</template>

<script>
import Toast from "@/components/Toast";

export default {
  name: 'App',
  components: {Toast},
}
</script>

<style>
body.modal-open {
  overflow: hidden;
  /*padding-right: 15px;*/
}

html {
  background-color: #22252e;
}
</style>
