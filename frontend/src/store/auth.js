import Vue from "vue";
import HttpService from "@/common/http-service";

const userToken = localStorage.getItem('userToken') || '';
const user = localStorage.getItem('user') || '';
const admin = localStorage.getItem('admin') || false;

const initialState = {
    userToken: userToken,
    user: user,
    loggedIn: !!(userToken && user),
    administrator: admin
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
            return state.loggedIn;
        },
        isAdministrator: state => {
            return state.administrator;
        },
    },
    mutations: {
        setAuthModal(state, opened) {
            state.authModalOpen = opened
        },
        authSuccess(state, {token, username, admin}) {
            state.loggedIn = true;
            state.userToken = token;
            state.user = username;
            state.administrator = admin;

            localStorage.setItem('userToken', token);
            localStorage.setItem('user', username);
            localStorage.setItem('admin', admin);

            Vue.notify({
                title: 'Authentication',
                text: 'Successfully logged in!',
                type: 'success'
            });
        },
        logout(state) {
            state.loggedIn = false;
            state.userToken = '';

            localStorage.removeItem('userToken');


        }
    },
    actions: {
        login({dispatch, commit}, data) {
            HttpService.post('/user/login', data, (res) => {
                const data = res.data.data;

                commit('authSuccess', {token: data.token, username: data.username, admin: data.admin});
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
