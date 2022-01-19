<template>
  <div class="sticky top-0 z-40 w-full backdrop-blur flex-none transition-colors duration-500 lg:z-50 lg:border-b lg:border-slate-50/[0.06] bg-slate-900/75">

    <div class="max-w-8xl mx-auto">

      <div class="py-4 lg:px-8 border-b lg:border-0 border-slate-300/10 mx-4 lg:mx-0">

        <div class="relative flex items-center">

          <router-link class="flex-none w-auto overflow-hidden text-xl text-slate-200 hover:text-white" to="/">
            <span class="sr-only">{{ $store.state.settings.website.name }}</span>
            <span class="font-semibold">{{ $store.state.settings.website.name }}</span>
          </router-link>

          <div class="relative flex items-center ml-auto">

<!--            <nav class="text-sm leading-6 text-slate-200">-->
<!--              <ul class="flex space-x-8">-->
<!--                <li>-->
<!--                  <a href="/docs/installation">-->
<!--                    <a class="hover:text-sky-400">Docs</a>-->
<!--                  </a>-->
<!--                </li>-->
<!--                <li>-->
<!--                  <a href="https://tailwindui.com" class="hover:text-sky-400">-->
<!--                    Components-->
<!--                  </a>-->
<!--                </li>-->
<!--                <li>-->
<!--                  <a href="/blog">-->
<!--                    <a class="hover:text-sky-400">Blog</a>-->
<!--                  </a>-->
<!--                </li>-->
<!--              </ul>-->
<!--            </nav>-->
            <div v-if="$store.getters.isLoggedIn" class="hidden lg:flex justify-between items-center space-x-4">
              <router-link to="/upload" class="px-4 py-1 bg-sky-800 text-sm text-sky-100 rounded-xl hover:shadow-lg hover:bg-sky-700 hover:shadow-sky-700/20 duration-200">
                Upload torrent
              </router-link>
            </div>

            <div class="ml-6 pl-6 flex items-center">
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

export default {
  name: 'Navbar',
  components: {Sidebar, Breadcrumb, Profile, Logo},
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
