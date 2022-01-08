<template>
  <div class="relative inline-block text-left" v-click-outside="() => (dropdownOpened = false)">
    <div class="py-2 px-4 rounded-lg bg-primary text-white flex items-center relative cursor-pointer" @click="dropdownOpened = !dropdownOpened">
      {{ user }}
      <UserIcon size="16" class="ml-2"/>
    </div>

    <div :class="{hidden: !dropdownOpened}"
         class="origin-top-right absolute right-0 mt-2 w-56 rounded-md shadow-lg bg-white z-10 ring-1 ring-black ring-opacity-5"
         role="menu" aria-orientation="vertical" aria-labelledby="menu-button" tabindex="-1"
    >
      <div class="py-1" role="none">
        <router-link to="/settings" replace class="block w-full text-left px-4 py-2 hover:bg-gray-100">
          <span>Settings</span>
        </router-link>
        <button
            @click="$store.dispatch('logout')"
            class="block w-full text-left px-4 py-2 text-red-500 hover:bg-gray-100"
        >
          Sign out
        </button>
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
