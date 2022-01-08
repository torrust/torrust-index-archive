<template>
  <div class="leading-normal tracking-normal" id="main-body">
    <div class="flex flex-wrap">

      <Sidebar class="bg-primary z-10 lg:z-0" />

      <div class="rounded-l-3xl bg-secondary w-full ml-0 lg:ml-64 min-h-screen flex flex-col z-0" :class="sideBarOpen ? 'overlay' : ''" id="main-content">

        <Navbar />

        <div class="p-6 px-4 lg:p-6">
          <Breadcrumb />
          <router-view />
        </div>

        <Footer />

      </div>
    </div>
    <AuthenticationModal />
  </div>
</template>

<script>
import { mapState } from 'vuex'

import Sidebar from './navigation/Sidebar'
import Navbar from './navigation/Navbar'
import Footer from './Footer'
import Breadcrumb from "@/components/Breadcrumb";
import AuthenticationModal from "../views/AuthenticationModal";

export default {
  name: 'Layout',
  computed: {
    ...mapState(['sideBarOpen'])
  },
  components: {
    AuthenticationModal,
    Breadcrumb,
    Sidebar,
    Navbar,
    Footer
  }
}
</script>

<style scoped>

</style>
