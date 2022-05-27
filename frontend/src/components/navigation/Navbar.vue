<template>
  <div class="top-0 z-40 w-full flex-none lg:z-50">
    <div class="max-w-8xl mx-auto">
      <div class="py-4 border-b lg:border-0 border-slate-300/10">
        <div class="relative flex items-center">
          <div class="text-2xl font-bold text-slate-100">
            {{ $route.name }}
          </div>
          <div class="relative flex items-center ml-auto">
            <div v-if="$store.getters.isLoggedIn" class="flex justify-between items-center space-x-4">
              <router-link to="/upload" class="px-4 py-1.5 bg-sky-500 text-sm text-white border border-sky-500 rounded-md transition duration-200 hover:shadow-lg hover:shadow-sky-500/25">
                <span class="hidden md:block">Upload torrent</span>
                <span class="block md:hidden">+</span>
              </router-link>
            </div>

            <div v-if="this.$store.getters.isAdministrator&&this.$store.state.settings.invite_only" class="ml-4 flex items-center">
              <Invite />
            </div>

            <div class="ml-4 flex items-center">
              <Profile />
            </div>
 
          </div>
        </div>
      </div>
      <div class="flex items-center p-4 border-b lg:hidden text-slate-400 border-slate-50/[0.06]">
        <button
            @click="toggleSidebar"
            type="button"
        >
          <span class="sr-only">Navigation</span>
          <svg width="24" height="24">
            <path
                d="M5 6h14M5 12h14M5 18h14"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
            />
          </svg>
        </button>
        <Breadcrumb class="ml-6" />
      </div>
      <Sidebar v-show="$store.state.sideBarOpen" class="block lg:hidden px-4" />
    </div>
  </div>
</template>

<script>
import {mapState} from 'vuex'
import Profile from "./Profile.vue";
import Logo from "../Logo.vue";
import Breadcrumb from "../Breadcrumb.vue";
import Sidebar from "./Sidebar.vue";
import Invite from "./Invite.vue";

export default {
  name: 'Navbar',
  components: {Sidebar, Breadcrumb, Profile, Logo, Invite},
  computed: {
    ...mapState({})
  },
  data: () => ({
    searchQuery: ''
  }),
  methods: {
    submitSearch() {
      if (this.searchQuery) {
        this.$router.push(`/torrents?search=${this.searchQuery}`)
      }
    },
    toggleSidebar() {
      this.$store.dispatch('toggleSidebar')
    }
  }
}
</script>

<style scoped>
img {
  image-rendering: crisp-edges;
}

.button {
  @apply h-10 px-4 bg-red-500 text-white rounded-lg;
}
</style>
