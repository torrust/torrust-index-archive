<template>
  <div class="max-w-8xl mx-auto px-4 sm:px-6 md:px-8">
    <div class="hidden lg:block fixed z-20 inset-0 pt-4 left-[max(0px,calc(50%-45rem))] right-auto w-[19.5rem] pb-10 px-8 overflow-y-auto" id="main-nav">
      <Sidebar />
    </div>
    <div class="lg:pl-[19.5rem]">
      <div class="max-w-3xl mx-auto pt-0 lg:pt-4 xl:max-w-none xl:ml-0">
        <Navbar />
        <router-view />
        <Footer />
      </div>
    </div>
<!--    <Footer />-->
    <AuthenticationModal />
  </div>
</template>

<script>
import { mapState } from 'vuex'

import Sidebar from './navigation/Sidebar.vue'
import Navbar from './navigation/Navbar.vue'
import Footer from './Footer.vue'
import Breadcrumb from "./Breadcrumb.vue";
import AuthenticationModal from "../views/AuthenticationModal.vue";

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
