<template>
  <div v-click-outside="() => (dropdownOpened = false)">
      <button
          type="button"
          class="relative py-2 pr-10 pl-3 w-full text-left bg-slate-800/50 text-slate-400 rounded-md border border-slate-700 cursor-pointer focus:outline-none focus:ring-1 focus:border-sky-500 sm:text-sm"
          @click="dropdownOpened = !dropdownOpened"
      >
        <span class="block truncate capitalize">{{ selected || 'Select a category...' }}</span>
        <span class="flex absolute inset-y-0 right-0 items-center pr-2 pointer-events-none">
          <SelectorIcon class="w-5 h-5 text-gray-400"/>
        </span>
      </button>

      <transition
          leave-active-class="transition duration-100 ease-in"
          leave-from-class="opacity-100"
          leave-to-class="opacity-0"
      >
        <ul
            v-if="dropdownOpened"
            class="overflow-auto absolute z-10 py-1 mt-1 w-full max-h-60 text-base bg-slate-800 rounded-md ring-1 ring-black ring-opacity-5 shadow-lg focus:outline-none sm:text-sm"
            tabindex="-1" role="listbox"
        >
          <li class="relative py-2 pr-9 pl-3 text-slate-200 cursor-pointer select-none hover:text-sky-400 hover:bg-primary-600"
              role="option"
              v-for="(option, index) in options"
              :key="index"
              @click="select(option.name)"
          >
            <span
                class="block truncate capitalize"
                :class="[selected === option.name ? 'font-semibold' : 'font-normal']"
            >
              {{ option.name }}
            </span>

            <span
                v-if="selected === option.name"
                class="flex absolute inset-y-0 right-0 items-center pr-4 text-primary-600 row-icon"
            >
              <CheckIcon class="w-5 h-5"/>
            </span>
          </li>
        </ul>
      </transition>
    </div>
</template>

<script>
import {CheckIcon, SelectorIcon} from "@vue-hero-icons/solid";

export default {
  name: "SelectComponent",
  components: {CheckIcon, SelectorIcon},
  props: ['options', 'selected'],
  emits: ['update'],
  data: () => ({
    dropdownOpened: false,
  }),
  methods: {
    select(val) {
      this.dropdownOpened = false;
      this.$emit('update', val);
    }
  }
}
</script>

<style scoped>
li:hover .row-icon {
  @apply text-white;
}
</style>
