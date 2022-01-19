<template>
  <div class="relative inline-block text-left" v-click-outside="() => (dropdownOpened = false)">

    <button v-if="$store.getters.isLoggedIn" class="px-2 text-slate-200 flex items-center relative cursor-pointer hover:text-white" @click="dropdownOpened = !dropdownOpened">
      {{ user.username }}
      <ChevronDownIcon
          class="w-5 h-5 ml-2 -mr-1 text-slate-400"
          aria-hidden="true"
      />
    </button>

    <button v-else class="px-4 py-1 bg-sky-800 text-sm text-sky-100 rounded-xl hover:shadow-lg hover:bg-sky-700 hover:shadow-sky-700/20 duration-200" @click="$store.dispatch('openAuthModal')">
      Sign in
    </button>

    <div class="origin-top-right absolute right-0 mt-5" :class="{hidden: !dropdownOpened}">
      <div @click.prevent="() => (dropdownOpened = false)" class="py-2 px-2 w-48 flex flex-col backdrop-blur bg-slate-900/75 text-sm text-gray-500 rounded-xl shadow-lg ring-1 ring-slate-700 ring-inset">
        <router-link to="/settings" replace class="py-2 text-slate-100 text-center hover:bg-slate-600 hover:bg-opacity-10 rounded-lg">
          <span>Settings</span>
        </router-link>
        <hr class="my-2 border-slate-700" />
        <button @click="$store.dispatch('logout')" class="px-4 py-2 bg-red-800 hover:bg-red-700 text-sm text-white rounded-md hover:shadow-lg hover:shadow-red-500/20 duration-200">Sign out</button>
      </div>
    </div>

  </div>
</template>

<script>
import {UserIcon} from "@vue-hero-icons/outline";
import { ChevronDownIcon } from '@vue-hero-icons/solid'
import {mapState} from "vuex";

export default {
  name: "Profile",
  components: {UserIcon, ChevronDownIcon},
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
