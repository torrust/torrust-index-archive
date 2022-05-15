<template>
  <div id="app">
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
    this.$store.dispatch('getPublicSettings');
    this.$store.dispatch('getSiteName');
  },
  watch:{
    $route (to, from){
      this.$store.dispatch('getPublicSettings');
      this.$store.dispatch('getCategories');
      this.$store.dispatch('getSiteName');
    }
  }
}
</script>

<style>
html {
  @apply bg-slate-900;
}
</style>
