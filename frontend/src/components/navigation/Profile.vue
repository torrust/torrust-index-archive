<template>
  <div class="relative inline-block text-left" v-click-outside="() => (dropdownOpened = false)">

    <button v-if="$store.getters.isLoggedIn" class="px-4 py-1.5 rounded-md border border-slate-800 text-sm text-slate-400 flex items-center relative cursor-pointer transition duration-200 hover:text-slate-200 hover:border-slate-200" @click="dropdownOpened = !dropdownOpened">
      <UserCircleIcon size="16" class="mr-1 opacity-50" />
      {{ user.username }}
      <ChevronDownIcon
          class="w-5 h-5 ml-2 -mr-1"
          aria-hidden="true"
      />
    </button>

    <button v-else class="px-4 py-1.5 bg-sky-500 text-sm text-white border border-sky-500 rounded-md transition duration-200 hover:shadow-lg hover:shadow-sky-500/25" @click="$store.dispatch('openAuthModal')">
      Sign in
    </button>

    <div class="origin-top-right absolute right-0 mt-2 z-10" :class="{hidden: !dropdownOpened}">
      <div @click.prevent="() => (dropdownOpened = false)" class="py-2 px-2 w-48 flex flex-col bg-slate-800 text-sm rounded-md shadow-lg">
        <router-link to="/settings" replace class="py-1.5 text-center text-slate-100 border border-transparent rounded-md transition duration-200 hover:bg-slate-700 hover:border-slate-700">
          <span>Settings</span>
        </router-link>
        <hr class="my-2 border-slate-700" />
        <button @click="$store.dispatch('logout')" class="py-1.5 bg-red-500 bg-opacity-10 text-red-400 border border-transparent rounded-md transition duration-200 hover:text-red-500">Sign out</button>
      </div>
    </div>

  </div>
</template>

<script>
import {UserIcon} from "@vue-hero-icons/outline";
import { ChevronDownIcon, UserCircleIcon } from '@vue-hero-icons/solid'
import {mapState} from "vuex";

export default {
  name: "Profile",
  components: {UserIcon, ChevronDownIcon, UserCircleIcon},
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
