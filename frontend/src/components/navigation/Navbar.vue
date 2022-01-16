<template>
  <div class="sticky top-0 z-40 w-full backdrop-blur flex-none transition-colors duration-500 lg:z-50 lg:border-b lg:border-slate-900/10 border-slate-50/[0.06] supports-backdrop-blur:bg-white/95 bg-slate-900/75">

    <div class="max-w-8xl mx-auto">

      <div class="px-4 py-4 border-b lg:px-8 border-slate-300/10">

        <div class="relative flex items-center">

          <router-link class="mr-3 flex-none w-auto overflow-hidden" to="/">
            <span class="sr-only">{{ $store.state.settings.website.name }}</span>
            <span class="font-semibold text-white">{{ $store.state.settings.website.name }}</span>
          </router-link>

          <div class="relative hidden lg:flex items-center ml-auto">

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

            <div class="ml-6 pl-6 flex items-center border-l border-slate-800">
              <Profile />
            </div>

          </div>
        </div>
      </div>

      <div class="flex items-center p-4 border-b lg:hidden border-slate-50/[0.06]">
        <button
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
      </div>
    </div>
  </div>
</template>

<script>
import {mapState} from 'vuex'
import Profile from "./Profile.vue";
import Logo from "../Logo.vue";

export default {
  name: 'Navbar',
  components: {Profile, Logo},
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
