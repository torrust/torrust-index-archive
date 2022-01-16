<template>
  <div class="relative inline-block text-left" v-click-outside="() => (dropdownOpened = false)">

    <a class="text-slate-200 flex items-center relative cursor-pointer hover:text-white" @click="dropdownOpened = !dropdownOpened">
      {{ user.username }}
      <UserIcon size="14" class="ml-2"/>
    </a>

    <div class="origin-top-right absolute right-0 mt-2" :class="{hidden: !dropdownOpened}">
      <div @click.prevent="() => (dropdownOpened = false)" class="py-2 bg-slate-800 w-60 highlight-10 rounded-lg flex flex-col text-sm px-2 text-gray-500 shadow-lg">
        <router-link to="/settings" replace class="py-1.5 text-slate-100 text-center hover:bg-slate-600 hover:bg-opacity-10 rounded-lg">
          <span>Settings</span>
        </router-link>
        <hr class="my-2 border-slate-700" />
        <button @click="$store.dispatch('logout')" class="py-1.5 text-red-500 hover:bg-red-500 hover:bg-opacity-10 rounded-lg">Sign out</button>
      </div>
    </div>

  </div>
</template>

<script>
import {UserIcon} from "@vue-hero-icons/outline";
import {mapState} from "vuex";

export default {
  name: "Profile",
  components: {UserIcon},
  data: () => ({
    dropdownOpened: false,
  }),
  computed: {
    ...mapState({
      user: state => state.auth.user,
    })
  }
}
</script>

<style scoped>

</style>
