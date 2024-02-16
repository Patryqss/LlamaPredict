<script lang="ts" setup>
import { formatUSDAmount } from "~/utils";

defineProps({
  title: {
    type: String,
    required: true,
  },
  description: {
    type: String,
    required: true,
  },
  minValue: {
    type: Number,
    required: true,
  },
  maxValue: {
    type: Number,
    required: true,
  },
  expireDate: {
    type: Date,
    required: true,
  },
});
</script>

<template>
  <div
    class="card bg-neutral text-neutral-content rounded-lg shadow-lg"
  >
    <div class="card-body px-10 py-5 space-y-5">
      <h2 class="card-title text-center text-md md:text-xl">
        {{ title }}
      </h2>
      <p>{{ description }}</p>
      <div class="flex bg-primary px-4 py-2 rounded-3xl text-primary-content justify-between">
        <span>{{ formatUSDAmount(minValue) }}</span>
        <span>{{ formatUSDAmount(maxValue) }}</span>
      </div>
    </div>
  </div>
</template>
