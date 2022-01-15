import Vue from 'vue'
import Vuex from 'vuex'
import authStore from './auth';
import HttpService from "@/common/http-service";

Vue.use(Vuex)

export default new Vuex.Store({
    state: {
        sideBarOpen: false,
        categories: [],
        categoryFilters: [],
        settings: {
            website: {
                name: "Torrust"
            },
            tracker: {
                url: "",
                api_url: "",
                token: "",
                token_valid_seconds: 0
            },
            net: {
                port: 0,
                base_url: null
            },
            auth: {
                min_password_length: 0,
                max_password_length: 0,
                secret_key: ""
            },
            database: {
                connect_url: "",
                torrent_info_update_interval: 0
            },
            storage: {
                upload_path: ""
            },
            mail: {
                email_verification_enabled: false,
                from: "",
                reply_to: "",
                username: "",
                password: "",
                server: "",
                port: 0
            }
        }
    },
    getters: {
        sideBarOpen: state => {
            return state.sideBarOpen;
        },
    },
    mutations: {
        toggleSidebar (state) {
            state.sideBarOpen = !state.sideBarOpen
        },
        setCategories(state, categories) {
            Vue.set(state, 'categories', categories);
        },
        setCategoryFilters(state, categoryFilters) {
            Vue.set(state, 'categoryFilters', categoryFilters);
        },
        setSettings(state, settings) {
            Vue.set(state, 'settings', settings);
        },
    },
    actions: {
        closeAuthModal({commit}) {
            commit('setAuthModal', false);
            document.body.classList.remove("modal-open");
        },
        openAuthModal({commit}) {
            commit('setAuthModal', true);
            document.body.classList.add("modal-open");
        },
        toggleSidebar({commit}) {
            commit('toggleSidebar')
        },
        getCategories({commit}) {
            HttpService.get('/category', (res) => {
                commit('setCategories', res.data.data);
            }).catch(() => {});
        },
        getSettings({commit}) {
            HttpService.get('/settings', (res) => {
                commit('setSettings', res.data.data);
                window.document.title = res.data.data.website.name;
            }).catch(() => {});
        }
    },
    modules: {
        auth: authStore
    }
})
