<template>
  <div>
    <h1 class="view-title text-white">Categories</h1>

    <div class="w-full mt-4 max-w-2xl">

      <div id="categories">
        <h2 class="mt-6 text-xl text-white">Categories</h2>
        <ul class="py-2">
          <li v-for="(category, index) in categoriesState" :key="index">
            <div class="py-2 px-4 mt-2 bg-primary rounded-xl flex flex-row">
              <span class="text-white">{{ category.name }} ({{ category.num_torrents }})</span>
              <a @click="deleteCategory(category.name)" class="ml-auto px-1 rounded-lg bg-opacity-10 text-red-400 hover:text-red-500 text-center cursor-pointer">
                X
              </a>
            </div>
          </li>
        </ul>
        <div class="flex flex-row">
          <input v-model="newCategory" class="py-2 px-4 appearance-none w-full bg-white text-gray-700 rounded-xl leading-tight focus:outline-none" type='text' placeholder='Enter category'>
          <button @click="addCategory" class="button ml-2 px-4 bg-green-600 hover:bg-green-500 text-white rounded-xl">Add</button>
        </div>
      </div>

      <h1 class="mt-6 view-title text-white">Site Settings</h1>
      <button @click="saveSettings" :disabled="settingsChanged || savingSettings" class="changes bg-blue-500 disabled:opacity-50">Save Changes</button>
      <button @click="clearSettings" :disabled="settingsChanged || savingSettings" class="changes ml-2 bg-red-500 disabled:opacity-50">Clear Changes</button>

      <div id="general-settings">
        <h2 class="text-xl text-white">General</h2>
        <label>Site Name</label>
        <div class="py-1 flex flex-row">
          <input type='text' v-model="settings.website.name">
        </div>
      </div>

      <div id="advanced-settings">
        <h2 class="mt-6 text-xl text-white">Advanced</h2>

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

        <button @click="saveSettings" :disabled="settingsChanged || savingSettings" class="changes bg-blue-500 disabled:opacity-50">Save Changes</button>
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
    ...mapState(['categories']),
    settingsChanged() {
      return JSON.stringify(this.$store.state.settings) === JSON.stringify(this.settings);
    },
    settingsState() {
      return this.$store.state.settings;
    },
    categoriesState() {
      return this.$store.state.categories;
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

input {
  @apply py-2 px-4 appearance-none w-full bg-white text-gray-700 rounded-xl leading-tight focus:outline-none;
}

button.changes {
  @apply mt-2 py-2 px-4 text-white rounded-xl;
}
</style>
