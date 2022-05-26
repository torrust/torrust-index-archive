import Layout from "../components/Layout.vue";
import TorrentDetail from "../views/Torrent.vue";
import CategoryOverview from "../views/CategoryOverview.vue";
import TorrentUpload from "../views/TorrentUpload.vue";
import VueRouter from "vue-router";
import Torrents from "../views/Torrents.vue";
import Welcome from "../views/Welcome.vue";
import Settings from "../views/settings/Settings.vue";
import Register from "../views/Register.vue";

const routes = [
    {
        path: '/',
        component:
        Layout,
        children: [
            {
                path: '/',
                name: 'Welcome',
                component: Welcome,
            },
            {
                path: 'settings',
                name: 'Settings',
                component: Settings,
            },
            {
                path: 'categories',
                name: "Category's",
                component: CategoryOverview
            },
            {
                path: 'torrent/:torrentId/:title?',
                name: 'Torrent',
                component: TorrentDetail
            },
            {
                path: 'torrents/:sorting?',
                name: 'Browse Torrents',
                component: Torrents,
            },
            {
                path: 'upload',
                name: 'Upload Torrent',
                component: TorrentUpload
            },
	    {
		path: 'register',
		name: 'Register User',
		component: Register
	    },
        ]
    }
];

const router = new VueRouter({
    mode: 'history',
    routes
});

export default router
