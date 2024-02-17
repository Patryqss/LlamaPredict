<script lang="ts" setup>
defineProps({
  id: {
    type: String,
    required: true,
  },
  title: {
    type: String,
    required: true,
  },
  btnText: {
    type: String,
    default: "Submit",
  },
  isLoading: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(["submit"]);

function onSubmit() {
  emit("submit");
}
</script>

<template>
  <dialog :id="id" class="modal shadow-xl backdrop-blur">
    <div class="modal-box bg-neutral text-neutral-content">
      <h1 class="mb-6 text-2xl font-bold">
        {{ title }}
      </h1>

      <slot />

      <div class="mt-10 flex w-full justify-center">
        <button class="btn btn-primary w-1/2 p-2" @click="onSubmit">
          <span v-if="isLoading" class="loading loading-spinner loading-md" />
          {{ btnText }}
        </button>
      </div>
    </div>

    <!-- A trick that's closing the dialog when clicked outside of it -->
    <form method="dialog" class="modal-backdrop opacity-100">
      <button
        class="cursor-default outline-none"
        :style="{ background: 'transparent' }"
      >
        close
      </button>
    </form>
  </dialog>
</template>
