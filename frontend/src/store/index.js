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
        }
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
        }
    },
    modules: {
        auth: authStore
    }
})
