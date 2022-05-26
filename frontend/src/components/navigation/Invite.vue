<template>
  <div class="relative inline-block text-left" v-click-outside="() => (dropdownOpened = false)">

    <button  class="px-4 py-1.5 rounded-md border border-slate-800 text-sm text-slate-400 flex items-center relative cursor-pointer transition duration-200 hover:text-slate-200 hover:border-slate-200" @click="toggle">
      <UserAddIcon size="18" class="opacity-90" />
      <!-- <ChevronDownIcon
          class="w-5 h-5 ml-2 -mr-1"
          aria-hidden="true"
      /> -->
    </button>

    <div class="origin-top-right absolute right-0 mt-2 z-10" :class="{hidden: !dropdownOpened}">
      <div @click.prevent="() => (dropdownOpened = true)" class="py-2 px-2 w-max flex flex-col bg-slate-800 text-sm rounded-md shadow-lg">
        <!-- <router-link to="/settings" replace class="py-1.5 text-center text-slate-100 border border-transparent rounded-md transition duration-200 hover:bg-slate-700 hover:border-slate-700"> -->
	  <span class="inline text-right align-middle ">
          
		<div class="overflow-x-auto inline-block text-center text-slate-400 border border-transparent rounded-md transition duration-200 hover:bg-slate-700 hover:border-slate-700"> {{this.$store.state.settings.net.base_url+':'+this.$store.state.settings.net.port}}/register?invite_code={{ link }} </div>
	  <button class="px-1 py-1.5 rounded-md border border-slate-800 text-sm text-slate-400 inline items-center relative cursor-pointer transition duration-200 hover:text-slate-200 hover:border-slate-400" @click="copyLinkToClipboard">
		<ClipboardCopyIcon />
	  </button>
	  </span>
        <!-- </router-link>
        <hr class="my-2 border-slate-700" />
        <button @click="$store.dispatch('logout')" class="py-1.5 bg-red-500 bg-opacity-10 text-red-400 border border-transparent rounded-md transition duration-200 hover:text-red-500">Sign out</button> -->
      </div>
    </div>

  </div>
</template>

<script>
import { UserIcon, ClipboardCopyIcon } from "@vue-hero-icons/outline";
import { UserAddIcon } from '@vue-hero-icons/solid'
import {mapState} from "vuex";
import HttpService from "@/common/http-service";

export default {
  name: "Invite",
  components: { UserAddIcon, ClipboardCopyIcon },
  data: () => ({
	dropdownOpened: false,
	link: ''
  }),
  methods: {
	toggle() {
	  //get the new invite link
	  if (!this.dropdownOpened)
	  	HttpService.get('/user/create_invite', (res) => this.link = res.data)
	  this.dropdownOpened=!this.dropdownOpened;
	},
	copyLinkToClipboard() {
		if(navigator.clipboard) 
		navigator.clipboard.writeText(this.link);
	}
  },
  computed: {
    ...mapState({
      user: state => state.auth.user,
    })
  }
}
</script>

<style scoped>

</style>
