<template>
  <div id="app">
    <Navbar />
    <Toast/>
    <div>
      <router-view />
    </div>
  </div>
</template>

<script>
import Toast from "./components/Toast.vue";
import Navbar from "./components/navigation/Navbar.vue";
import './main.css';

export default {
  name: 'App',
  components: {Toast, Navbar},
  beforeMount() {
    this.$store.dispatch('getSiteName')
  }
}
</script>

<style>
html {
  @apply bg-slate-900;
}
</style>
