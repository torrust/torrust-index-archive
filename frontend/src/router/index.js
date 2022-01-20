import Layout from "../components/Layout.vue";
import TorrentDetail from "../views/Torrent.vue";
import CategoryOverview from "../views/CategoryOverview.vue";
import TorrentUpload from "../views/TorrentUpload.vue";
import VueRouter from "vue-router";
import Torrents from "../views/Torrents.vue";
import Welcome from "../views/Welcome.vue";
import Settings from "../views/Settings.vue";

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
                name: 'CategoryOverview',
                component: CategoryOverview
            },
            {
                path: 'torrent/:torrentId',
                name: 'TorrentDetail',
                component: TorrentDetail
            },
            {
                path: 'torrents/:sorting?',
                name: 'TorrentList',
                component: Torrents,
            },
            {
                path: 'upload',
                name: 'TorrentUpload',
                component: TorrentUpload
            },
        ]
    }
];

const router = new VueRouter({
    mode: 'history',
    routes
});

export default router
