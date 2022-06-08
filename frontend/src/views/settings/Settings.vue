<template>
  <div>
<!--    <ul class="flex flex-wrap border-b border-slate-200/5">-->
<!--      <li>-->
<!--        <router-link :to="'/settings/general'" class="tab">General</router-link>-->
<!--      </li>-->
<!--      <li class="ml-2">-->
<!--        <router-link :to="'/settings/categories'" class="tab">Categories</router-link>-->
<!--      </li>-->
<!--    </ul>-->

    <div class="w-full flex flex-col max-w-2xl mx-auto">

      <div id="general-settings">
        <h2 class="text-xl text-white">General</h2>
        <label>Site Name</label>
        <div class="py-1 flex flex-row">
          <input type='text' v-model="settings.website.name">
          <button @click="saveSettings" :disabled="settingsChanged || savingSettings" class="button ml-2 px-4 bg-sky-600 hover:bg-sky-500 text-white rounded-md disabled:opacity-50">Save</button>
        </div>

        <label>Categories</label>
        <ul class="py-2">
          <li v-for="(category, index) in categoriesState" :key="index">
            <div class="py-2 px-4 mt-2 bg-slate-800 rounded-md flex flex-row">
              <span class="text-white">{{ category.name }} ({{ category.num_torrents }})</span>
              <svg xmlns="http://www.w3.org/2000/svg" @click="deleteCategory(category.name)" class="h-6 w-6 ml-auto px-1 rounded-lg bg-opacity-10 text-red-400 hover:text-red-500 text-center cursor-pointer" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
              </svg>
            </div>
          </li>
        </ul>
        <div class="mt-2 flex flex-row">
          <input v-model="newCategory" type='text' placeholder='Enter category'>
          <button @click="addCategory" :disabled="!newCategory" class="button ml-2 px-4 bg-sky-600 hover:bg-sky-500 text-white rounded-md disabled:opacity-50">Add</button>
        </div>

        <label>Pages</label>
        <ul class="py-2">
          <li v-for="(page, index) in pagesState" :key="index">
            <div class="py-2 px-4 mt-2 bg-slate-800 rounded-md flex flex-row">
              <button @click="selectPage(page.route)" class="text-white">{{ page.title }}</button>
              <svg xmlns="http://www.w3.org/2000/svg" @click="deletePage(page.route)" class="h-6 w-6 ml-auto px-1 rounded-lg bg-opacity-10 text-red-400 hover:text-red-500 text-center cursor-pointer" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
              </svg>
            </div>
          </li>
        </ul>
 

        <div class="mt-2 flex flex-row">
          <button @click="selectPage('new')" class="button ml-2 px-4 py-2 bg-sky-600 hover:bg-sky-500 text-white rounded-md disabled:opacity-50">Add page</button>
        </div>

      </div>

      <div id="advanced-settings">
        <div class="py-3 border-b border-slate-200/5"></div>
        <h2 class="mt-6 text-xl text-white">Advanced</h2>
        <button @click="saveSettings" :disabled="settingsChanged || savingSettings" class="changes bg-sky-600 disabled:opacity-50">Save Changes</button>
        <button @click="clearSettings" :disabled="settingsChanged || savingSettings" class="changes ml-2 bg-red-500 disabled:opacity-50">Clear Changes</button>

        <!-- Tracker -->
        <h3>Tracker</h3>
        <label>URL</label>
        <div class="setting-input-container">
          <input type='text' v-model="settings.tracker.url">
        </div>

        <label>API URL</label>
        <div class="setting-input-container">
          <input type='text' v-model="settings.tracker.api_url">
        </div>

        <label>API Token</label>
        <div class="setting-input-container">
          <input type='text' v-model="settings.tracker.token">
        </div>

        <label>Private Key Validity (seconds)</label>
        <div class="setting-input-container">
          <input type='number' v-model="settings.tracker.token_valid_seconds">
        </div>

        <!-- NET -->
        <h3>Network</h3>
        <label>Port [needs restart]</label>
        <div class="setting-input-container">
          <input type='number' v-model="settings.net.port">
        </div>

        <label>Domain [optional]</label>
        <div class="setting-input-container">
          <input type='text' v-model="settings.net.base_url" placeholder="optional">
        </div>

        <!-- Authentication -->
        <h3>Authentication</h3>
        <label>Min. Password Length</label>
        <div class="setting-input-container">
          <input type='number' v-model="settings.auth.min_password_length">
        </div>

        <label>Max. Password Length</label>
        <div class="setting-input-container">
          <input type='number' v-model="settings.auth.max_password_length">
        </div>

        <label>Secret Key</label>
        <div class="setting-input-container">
          <input type='text' v-model="settings.auth.secret_key">
        </div>

        <!-- Database -->
        <h3>Database</h3>
        <label>Connect URL</label>
        <div class="setting-input-container">
          <input type='text' v-model="settings.database.connect_url">
        </div>

        <label>Torrent Status Update Interval (seconds)</label>
        <div class="setting-input-container">
          <input type='number' v-model="settings.database.torrent_info_update_interval">
        </div>

        <!-- Storage -->
        <h3>Storage</h3>
        <label>Torrent Upload Path</label>
        <div class="setting-input-container">
          <input type='text' v-model="settings.storage.upload_path">
        </div>

        <!-- Mail -->
        <h3>Mail</h3>
        <label>Email Verification Enabled (true or false)</label>
        <div class="setting-input-container">
          <input type='text' v-model="settings.mail.email_verification_enabled">
        </div>

        <label>Server</label>
        <div class="setting-input-container">
          <input type='text' v-model="settings.mail.server">
        </div>

        <label>Port</label>
        <div class="setting-input-container">
          <input type='text' v-model="settings.mail.port">
        </div>

        <label>Username</label>
        <div class="setting-input-container">
          <input type='text' v-model="settings.mail.username">
        </div>

        <label>Password</label>
        <div class="setting-input-container">
          <input type='text' v-model="settings.mail.password">
        </div>

        <label>From</label>
        <div class="setting-input-container">
          <input type='text' v-model="settings.mail.from">
        </div>

        <label>Reply To</label>
        <div class="setting-input-container">
          <input type='text' v-model="settings.mail.reply_to">
        </div>

        <button @click="saveSettings" :disabled="settingsChanged || savingSettings" class="changes bg-sky-600 disabled:opacity-50">Save Changes</button>
        <button @click="clearSettings" :disabled="settingsChanged || savingSettings" class="changes ml-2 bg-red-500 disabled:opacity-50">Clear Changes</button>

      </div>
    </div>
  </div>
</template>

<script>

import {mapState} from "vuex";
import HttpService from "@/common/http-service";

export default {
  name: "Settings",
  data: () => ({
    tab: 'general',
    newCategory: '',
    savingSettings: false,
    settings: {
      website: {
        name: ""
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
  }),
  computed: {
    ...mapState(['categories', 'pages']),
    settingsChanged() {
      return JSON.stringify(this.$store.state.settings) === JSON.stringify(this.settings);
    },
    settingsState() {
      return this.$store.state.settings;
    },
    categoriesState() {
      return this.$store.state.categories;
    },
    pagesState() {
      return this.$store.state.pages;
    }
  },
  methods: {
    addCategory() {
      if (this.newCategory) {
        HttpService.post(`/category`, { name: this.newCategory }, () => {
          this.$store.dispatch('getCategories');
        }).catch(() => {
        });
      }
    },
    deleteCategory(category) {
      HttpService.delete(`/category`, { name: category }, () => {
        this.$store.dispatch('getCategories');
      }).catch(() => {
      });
    },
    deletePage(route) {
      HttpService.delete(`/page`, { route }, () => {
        this.$store.dispatch('getPages');
      }).catch(() => {} );
    },
    selectPage(page) {
       this.$router.push(`/${page}`)
    },
    saveSettings() {
      this.savingSettings = true;
      HttpService.post(`/settings`, this.settings, () => {
        this.$store.dispatch('getSettings');
        this.savingSettings = false;
      }).catch(() => {
        this.savingSettings = false;
      });
    },
    clearSettings() {
      this.settings = JSON.parse(JSON.stringify(this.$store.state.settings));
    },
  },
  beforeMount() {
    this.$store.dispatch('getSettings');
    this.$store.dispatch('getCategories');
    this.$store.dispatch('getPages');
  },
  watch: {
    settingsState() {
      this.clearSettings();
    },
  }
}
</script>

<style scoped>
.category-tile {
  @apply bg-cover !important;
  @apply w-full rounded-3xl shadow-lg text-center py-16 relative;
}

label {
  @apply mt-2 block text-gray-200;
}

h2 {
  @apply mt-6 text-xl text-white;
}

h3 {
  @apply mt-2 text-lg text-white;
}

.details {
  @apply inline-flex;
}

.setting-input-container {
  @apply py-1 flex flex-row;
}

button.changes {
  @apply mt-2 py-2 px-4 text-white rounded-md;
}

.tab {
  @apply inline-block py-2 px-4 text-sm font-medium text-center text-sky-500 bg-slate-800 rounded-t-md;
}

input {
  @apply py-2 px-4 w-full text-white bg-slate-800 border border-slate-700 rounded-md text-sm shadow-sm cursor-pointer placeholder-slate-400 hover:border-sky-500 focus:bg-slate-800
  focus:outline-none focus:border-sky-500 focus:ring-1 focus:ring-sky-500 transition duration-200;
}
</style>
