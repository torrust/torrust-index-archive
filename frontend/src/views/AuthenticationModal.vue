<template>
  <div
      class="flex fixed top-0 left-0 z-30 flex-col justify-center w-full min-h-screen bg-black bg-opacity-50"
      :class="{hidden: !authModalOpen}"
      @click.self="$store.dispatch('closeAuthModal')"
  >
    <div class="mx-auto mt-8 w-11/12 sm:w-full sm:max-w-md">
      <div
          class="px-4 py-8 bg-white rounded-lg shadow sm:px-10"
      >
        <transition
            enter-active-class="transition ease-out duration-100 transform"
            enter-class="opacity-0 scale-95"
            enter-to-class="opacity-100 scale-100"
            leave-active-class="transition ease-in duration-75 transform"
            leave-class="opacity-100 scale-100"
            leave-to-class="opacity-0 scale-95"
        >
          <h2 v-if="isSignUp" class="text-2xl mb-4 text-center">Sign Up</h2>
          <h2 v-else class="text-2xl mb-4 text-center">Sign In</h2>
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
              <label for="password" class="inline text-sm font-medium text-gray-700">
                Username
              </label>
              <div class="mt-1">
                <input v-model="form.username"
                       id="username" name="username" type="text"
                       required class="form-style">
              </div>
            </div>
          </transition>

          <div>
            <label for="email" class="block text-sm font-medium text-gray-700">
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
              <label for="password" class="inline text-sm font-medium text-gray-700">
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
              <label for="password" class="inline text-sm font-medium text-gray-700">
                Confirm password
              </label>
              <div class="mt-1">
                <input v-model="form.confirm_password"
                       id="password-repeat" name="password-repeat" type="password"
                       autocomplete="new-password" required class="form-style">
              </div>
            </div>
          </transition>

          <div>
            <button type="submit"
                    class="flex justify-center px-4 py-2 w-full text-sm font-medium text-white rounded-md border border-transparent shadow-sm bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 ">
              Sign {{ isSignUp ? 'up' : 'in' }}
            </button>
          </div>
        </form>
        <div class="relative mt-4">
          <div class="flex absolute inset-0 items-center">
            <div class="w-full border-t border-gray-300"></div>
          </div>
          <div class="flex relative justify-center text-sm">
            <button v-if="isSignUp" class="px-2 text-blue-500 bg-white" @click="toggleMode">
              Already have an account? Sign in
            </button>
            <button v-else class="px-2 text-blue-500 bg-white" @click="toggleMode">
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
    }
  }
}
</script>

<style scoped>
.form-style {
  @apply appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none sm:text-sm
}
</style>
