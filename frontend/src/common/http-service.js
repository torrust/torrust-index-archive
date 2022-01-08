import axios from "axios";
import Vue from "vue";
import store from "@/store";

export default new class {
    constructor() {
        axios.interceptors.response.use(undefined, (err) => {
            if (err.response && err.response.status === 401) {
                store.commit('logout');
                Vue.notify({
                    title: 'Authentication',
                    text: 'Your session has expired. Please login again.',
                    type: 'warn',
                });

                return;
            }

            // If its not a 401 continue to other error handlers
            return Promise.reject(err);
        });
    }

    setToken() {
        const token = store.state.auth.userToken;
        axios.defaults.headers.common['Authorization'] = `Bearer ${token}`;
    }

    get(url, callback) {
        return axios.get(url).then(callback).catch(this.errorHandler);
    }

    // for downloading files
    getBlob(url, callback) {
        this.setToken();

        return axios.get(url, { responseType: 'blob' }).then(callback).catch(this.errorHandler);
    }

    post(url, data, callback) {
        this.setToken();

        return axios.post(url, data).then(callback).catch(this.errorHandler);
    }

    delete(url, callback) {
        this.setToken();

        return axios.delete(url).then(callback).catch(this.errorHandler);
    }

    async errorHandler(error) {
        if (!error.response) {
            console.error('Cannot connect to backend');
            Vue.notify({
                title: 'Server error',
                text: 'Cannot connect to the server, please try again later.',
                type: 'error',
            });
            return;
        }

        // const res = error.response;
        let errorString = error.response.data;

        if (
            error.request.responseType === 'blob' &&
            error.response.data instanceof Blob &&
            error.response.data.type &&
            error.response.data.type.toLowerCase().indexOf('json') !== -1
        ) {
            const errorJson = JSON.parse(await error.response.data.text());
            errorString = errorJson.error;
        }

        // console.log(error.request.responseType === 'blob');
        // console.log(res.data.text())
        Vue.notify({
            title: 'Error',
            text: errorString,
            type: 'error',
        });

        throw error;
    }
}
