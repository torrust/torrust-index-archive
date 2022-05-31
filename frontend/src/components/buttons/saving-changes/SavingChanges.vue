<template>
  <transition
      enter-active-class="transition ease-out duration-100 transform"
      enter-class="opacity-0 scale-95"
      enter-to-class="opacity-100 scale-100"
      leave-active-class="transition ease-in duration-75 transform"
      leave-class="opacity-100 scale-100"
      leave-to-class="opacity-0 scale-95"
  >
    <div v-if="show" class="flex flex-row">
      <CancelButton :click="cancel" :disabled="disabled" class="ml-auto" />
      <SaveButton :click="save" :disabled="disabled" class="ml-2" />
    </div>
  </transition>
</template>

<script>
import CancelButton from "@/components/buttons/saving-changes/CancelButton.vue";
import SaveButton from "@/components/buttons/saving-changes/SaveButton.vue";

export default {
  name: "SavingChanges",
  components: {
    CancelButton,
    SaveButton
  },
  props: ['cancel', 'save', 'disabled', 'show']
}
</script>

<style scoped>

</style>
