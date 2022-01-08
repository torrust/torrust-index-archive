import Vue from 'vue';
import App from './App.vue';
import VueRouter from 'vue-router';
import './assets/css/tailwind.css';
import store from "@/store/index";
import vClickOutside from "v-click-outside";
import Notifications from 'vue-notification'
import axios from "axios";
import mixins from "@/mixins";
import router from "@/router";

axios.defaults.baseURL = process.env.VUE_APP_API_BASE_URL;

Vue.use(VueRouter);
Vue.use(vClickOutside);
Vue.use(Notifications);

Vue.config.productionTip = false;

Vue.mixin(mixins);

new Vue({
    render: h => h(App),
    router,
    store
}).$mount('#app');
