<template>
  <div class="text-slate-500">
    <h1 class="view-title text-white">Welcome to {{ $store.state.settings.website.name }}</h1>
  </div>
</template>

<script>
import {mapState} from "vuex";
import Vue from "vue";
export default {
  name: "Register",
  computed: {
	...mapState({
	  user: state => state.auth.user
	})
  },
  mounted () {
	console.log(this.$store.state);
	this.$store.state.isSignUp=true;
	this.$store.dispatch('openAuthModal');
  }
}
</script>

<style scoped>
.category-tile {
  @apply bg-cover !important;
  @apply w-full rounded-3xl shadow-lg text-center py-16 relative;
}

.details {
  @apply inline-flex;
}
</style>
