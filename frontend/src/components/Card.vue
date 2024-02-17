<script lang="ts" setup>
defineProps({
  title: {
    type: String,
    required: true,
  },
  subtitle: {
    type: String,
    default: ""
  },
});
</script>

<template>
  <div
    class="card bg-neutral w-full text-neutral-content rounded-xl mt-12 lg:mt-18 shadow-2xl"
  >
    <div class="card-body">
      <div class="mb-10">
        <h2 class="card-title text-xl md:text-3xl">
          {{ title }}
        </h2>
        <h3 v-if="subtitle" class="text-md md:text-xl">
          {{ subtitle }}
        </h3>
      </div>
      <slot />
    </div>
  </div>
</template>
