<template>
  <div
      class="flex fixed top-0 left-0 z-30 flex-col justify-center w-full min-h-screen bg-slate-900/90 backdrop-blur"
      :class="{hidden: !authModalOpen}"
      @click.self="close"
  >
    <div class="px-2 lg:px-0 w-full">
      <div class="px-6 mb-6 mx-auto max-w-md text-center">
        <h1 class="text-white text-3xl font-bold">{{ $store.state.settings.website.name }}</h1>
      </div>
      <div
          class="px-6 py-6 text-slate-400 bg-slate-800 rounded-md shadow mx-auto max-w-md"
      >
        <transition
            enter-active-class="transition ease-out duration-100 transform"
            enter-class="opacity-0 scale-95"
            enter-to-class="opacity-100 scale-100"
            leave-active-class="transition ease-in duration-75 transform"
            leave-class="opacity-100 scale-100"
            leave-to-class="opacity-0 scale-95"
        >
          <h2 v-if="isSignUp" class="text-xl mb-4 font-semibold text-center text-slate-400 uppercase">Create account</h2>
          <h2 v-else class="text-xl mb-4 font-semibold text-center text-slate-400 uppercase">Login</h2>
        </transition>

        <form
            @submit.prevent="submit"
            class="space-y-6"
        >
          <transition
              enter-active-class="transition ease-out duration-100 transform"
              enter-class="opacity-0 scale-95"
              enter-to-class="opacity-100 scale-100"
              leave-active-class="transition ease-in duration-75 transform"
              leave-class="opacity-100 scale-100"
              leave-to-class="opacity-0 scale-95"
          >
            <div v-if="isSignUp">
              <label for="password" class="inline text-sm font-medium text-white">
                Username
              </label>
              <div class="mt-1">
                <input v-model="form.username"
                       id="username" name="username" type="text"
                       required
                       class="form-style text-black">
              </div>
            </div>
          </transition>

          <div>
            <label for="email" class="block text-sm font-medium text-white">
              {{ !isSignUp ? 'Username or ' : '' }} Email address
            </label>
            <div class="mt-1">
              <input v-model="form.email"
                     id="email" name="email" type="text" autocomplete="email" required
                     class="form-style">
            </div>
          </div>

          <div>
            <div class="flex justify-between">
              <label for="password" class="inline text-sm font-medium text-white">
                Password
              </label>
              <!--              <div v-if="!isSignUp" class="mt-2 text-xs text-right">-->
              <!--                <a href="#" class="font-medium text-primary-600 hover:text-primary-500">-->
              <!--                  Forgot your password?-->
              <!--                </a>-->
              <!--              </div>-->
            </div>
            <div class="mt-1">
              <input v-model="form.password"
                     id="password" name="password" type="password"
                     :autocomplete="[isSignUp ? 'new-password' : 'current-password']" required class="form-style">
            </div>
          </div>

          <!-- Password confirmation for signup -->
          <transition
              enter-active-class="transition ease-out duration-100 transform"
              enter-class="opacity-0 scale-95"
              enter-to-class="opacity-100 scale-100"
              leave-active-class="transition ease-in duration-75 transform"
              leave-class="opacity-100 scale-100"
              leave-to-class="opacity-0 scale-95"
          >
            <div v-if="isSignUp">
              <label for="password" class="inline text-sm font-medium text-white">
                Confirm password
              </label>
              <div class="mt-1">
                <input v-model="form.confirm_password"
                       id="password-repeat" name="password-repeat" type="password"
                       autocomplete="new-password" required class="form-style">
              </div>
            </div>
          </transition>

          <div class="flex flex-row space-x-2">
            <button type="button"
                class="px-3 py-2 w-full justify-center flex flex-row text-sm text-red-200 hover:text-white bg-red-700 hover:bg-red-600 rounded-md hover:shadow-lg hover:shadow-red-700/25 transition duration-200"
                @click="close"
            >
              Cancel
            </button>
            <button type="submit"
                    class="px-3 py-2 w-full justify-center flex flex-row text-sm text-sky-200 hover:text-white bg-sky-800 hover:bg-sky-700 rounded-md hover:shadow-lg hover:shadow-sky-700/25 transition duration-200">
              Sign {{ isSignUp ? 'up' : 'in' }}
            </button>
          </div>
        </form>
        <div class="relative mt-6">
          <div class="flex relative justify-center text-sm">
            <button v-if="isSignUp" class="px-2 font-semibold text-slate-400 hover:text-slate-200 transition duration-200" @click="toggleMode">
              Already have an account? Sign in
            </button>
            <button v-else class="px-2 font-semibold text-slate-400 hover:text-slate-200 transition duration-200" @click="toggleMode">
              Don't have an account? Sign up
            </button>
          </div>
        </div>
      </div>
    </div>

  </div>
</template>

<script>
import {mapState} from "vuex";

export default {
  name: "AuthenticationModal",
  computed: {
    ...mapState({
      authModalOpen: state => state.auth.authModalOpen
    })
  },
  data: () => ({
    isSignUp: false,
    form: {
      username: "",
      email: "",
      password: "",
      confirm_password: "",
    },
  }),
  methods: {
    toggleMode() {
      this.isSignUp = !this.isSignUp;
    },
    submit() {
      if (this.isSignUp) {
        this.$store.dispatch('register', this.form).then(() => {
          this.toggleMode();
        });
      } else {
        this.$store.dispatch('login', {login: this.form.email, password: this.form.password});
      }
    },
    close() {
      this.$store.dispatch('closeAuthModal');
    },
  }
}
</script>

<style scoped>

label {
  @apply text-left text-xs font-medium uppercase tracking-wider text-slate-400 hover:text-slate-200;
}

.form-style {
  @apply mt-1 w-full px-3 py-2 text-white bg-slate-800 border border-slate-700 rounded-md text-sm shadow-sm cursor-pointer placeholder-slate-400 hover:border-sky-500 focus:bg-slate-800
  focus:outline-none focus:border-sky-500 focus:ring-1 focus:ring-sky-500 transition duration-200;
}
</style>
