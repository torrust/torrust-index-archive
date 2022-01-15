<template>
  <div>
    <h1 class="view-title text-white">Site Settings</h1>
    <button @click="saveSettings" :disabled="settingsChanged || savingSettings" class="changes bg-blue-500 disabled:opacity-50">Save Changes</button>
    <button @click="clearSettings" :disabled="settingsChanged || savingSettings" class="changes ml-2 bg-red-500 disabled:opacity-50">Clear Changes</button>

    <div class="w-full mt-4 max-w-2xl">
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

      </div>
      <div id="categories">
        <h2 class="mt-6 text-xl text-white">Categories</h2>
        <ul class="py-2">
          <li v-for="(category, index) in categories" :key="index">
            <div class="py-2 px-4 mt-2 bg-primary rounded-xl flex flex-row">
              <span class="text-white">{{ category.name }} ({{ category.num_torrents }})</span>
              <a @click="deleteCategory(category.name)" class="ml-auto px-3 rounded-lg bg-red-500 bg-opacity-10 text-red-400 hover:text-red-500 text-center cursor-pointer">
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

<!--        <form class="mt-6 border-t border-gray-400 pt-4">-->
<!--          <div class='flex flex-wrap -mx-3 mb-6'>-->
<!--            <div class='w-full md:w-full px-3 mb-6'>-->
<!--              <label class='block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2' for='grid-text-1'>email address</label>-->
<!--              <input class='appearance-none block w-full bg-white text-gray-700 border border-gray-400 shadow-inner rounded-md py-3 px-4 leading-tight focus:outline-none  focus:border-gray-500' id='grid-text-1' type='text' placeholder='Enter email'  required>-->
<!--            </div>-->
<!--            <div class='w-full md:w-full px-3 mb-6 '>-->
<!--              <label class='block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2'>password</label>-->
<!--              <button class="appearance-none bg-gray-200 text-gray-900 px-2 py-1 shadow-sm border border-gray-400 rounded-md ">change your password</button>-->
<!--            </div>-->
<!--            <div class='w-full md:w-full px-3 mb-6'>-->
<!--              <label class='block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2'>pick your country</label>-->
<!--              <div class="flex-shrink w-full inline-block relative">-->
<!--                <select class="block appearance-none text-gray-600 w-full bg-white border border-gray-400 shadow-inner px-4 py-2 pr-8 rounded">-->
<!--                  <option>choose ...</option>-->
<!--                  <option>USA</option>-->
<!--                  <option>France</option>-->
<!--                  <option>Spain</option>-->
<!--                  <option>UK</option>-->
<!--                </select>-->
<!--                <div class="pointer-events-none absolute top-0 mt-3  right-0 flex items-center px-2 text-gray-600">-->
<!--                  <svg class="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20"><path d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z"/></svg>-->
<!--                </div>-->
<!--              </div>-->
<!--            </div>-->
<!--            <div class='w-full md:w-full px-3 mb-6'>-->
<!--              <label class='block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2'>fav language</label>-->
<!--              <div class="flex-shrink w-full inline-block relative">-->
<!--                <select class="block appearance-none text-gray-600 w-full bg-white border border-gray-400 shadow-inner px-4 py-2 pr-8 rounded">-->
<!--                  <option>choose ...</option>-->
<!--                  <option>English</option>-->
<!--                  <option>France</option>-->
<!--                  <option>Spanish</option>-->
<!--                </select>-->
<!--                <div class="pointer-events-none absolute top-0 mt-3  right-0 flex items-center px-2 text-gray-600">-->
<!--                  <svg class="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20"><path d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z"/></svg>-->
<!--                </div>-->
<!--              </div>-->
<!--            </div>-->
<!--          </div>-->
<!--        </form>-->
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
        this.$store.dispatch('getSettings').then(() => {
          this.clearSettings();
        });
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
    this.$store.dispatch('getSettings').then(() => {
      this.clearSettings();
    });
  },
  watch: {
    settingsState() {
      this.clearSettings();
    }
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
  @apply mt-2 text-lg text-gray-200;
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
