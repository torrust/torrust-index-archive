import Vue from "vue";
import HttpService from "@/common/http-service";

const USER = {
    token: '',
    username: '',
    admin: false,
}

const user_encoded = localStorage.getItem('user');
const user = user_encoded ? JSON.parse(user_encoded) : { ...USER };

const initialState = {
    user: user
};

export default {
    state: {
        authModalOpen: false,
        ...initialState
    },
    getters: {
        isAuthenticationModalOpen: state => {
            return state.authModalOpen;
        },
        isLoggedIn: state => {
            return !!(state.user.token && state.user.username);
        },
        isAdministrator: state => {
            return state.user.admin;
        },
        getToken: (state, getters) => {
            return getters.isLoggedIn ? state.user.token : '';
        },
    },
    mutations: {
        setAuthModal(state, opened) {
            state.authModalOpen = opened
        },
        authSuccess(state, data) {
            state.user = data;

            localStorage.setItem('user', JSON.stringify(data));

            Vue.notify({
                title: 'Authentication',
                text: 'Successfully logged in!',
                type: 'success'
            });
        },
        logout(state) {
            state.user = { ...USER }

            localStorage.removeItem('user');
        }
    },
    actions: {
        login({dispatch, commit}, data) {
            HttpService.post('/user/login', data, (res) => {
                const data = res.data.data;

                commit('authSuccess', data);
                dispatch('closeAuthModal');
            });
        },
        register(store, data) {
            return new Promise((resolve) => HttpService.post('/user/register', data, () => {
                Vue.notify({
                    title: 'Authentication',
                    text: 'Account has been created!',
                    type: 'success',
                });

                resolve();
            }))
        },
        logout({commit}) {
            commit('logout');

            Vue.notify({
                title: 'Authentication',
                text: 'You have been logged out',
                type: 'info'
            });
        }
    }
}
