<script setup lang="ts">
import Navbar from "~/components/Navbar.vue";
</script>

<template>
  <div class="antialiased">
    <div class="from-primary to-base-100 min-h-screen bg-gradient-to-bl">
      <div class="mx-auto max-w-7xl pb-20">
        <Navbar />
        <div class="mt-5 px-5 sm:px-10">
          <RouterView v-if="accountStore.api" />
          <div v-else class="mt-40 flex w-full justify-center">
            <p class="loading loading-bars loading-lg" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped></style>
