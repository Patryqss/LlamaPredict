<script lang="ts" setup>
import { formatDistanceToNow } from "date-fns";
import { ClockIcon } from "@heroicons/vue/outline";
import { formatPctValue } from "~/utils";

defineProps({
  title: {
    type: String,
    required: true,
  },
  description: {
    type: String,
    required: true,
  },
  shortPct: {
    type: Number || String,
    required: true,
  },
  longPct: {
    type: Number || String,
    required: true,
  },
  expireDate: {
    type: Date,
    required: true,
  },
});
</script>

<template>
  <div class="card bg-neutral text-neutral-content rounded-lg shadow-lg">
    <div class="card-body space-y-5 px-10 py-7">
      <h2 class="card-title text-md text-center md:text-xl">
        {{ title }}
      </h2>
      <p>{{ description }}</p>
      <div class="flex gap-x-1 text-center shadow-xl">
        <div class="bg-primary/20 flex-1 rounded-l-3xl px-4 py-2">
          <p class="text-sm opacity-80">No</p>
          <p class="font-bold">{{ formatPctValue(shortPct) }}</p>
        </div>
        <div class="bg-primary/20 flex-1 rounded-r-3xl px-4 py-2">
          <p class="text-sm opacity-80">Yes</p>
          <p class="font-bold">{{ formatPctValue(longPct) }}</p>
        </div>
      </div>
      <div class="absolute right-4 -top-2 flex items-center text-xs">
        <ClockIcon class="mr-0.5 w-3" />
        <span>{{ formatDistanceToNow(expireDate) }}</span>
      </div>
    </div>
  </div>
</template>
