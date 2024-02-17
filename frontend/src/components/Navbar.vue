<script setup lang="ts">
import logo from "~/assets/logo.png";
import { formatAssetAmount } from "~/utils";

const tabs = [{
  label: 'Home',
  url: '/'
}, {
  label: 'Faucet',
  url: '/faucet'
}]
</script>

<template>
  <div
    class="text-base-content sticky top-0 z-30 mb-6 flex h-16 w-full justify-center backdrop-blur transition-all duration-100"
  >
    <nav class="navbar w-full">
      <div class="navbar-start">
        <div class="dropdown dropdown-hover">
          <label tabindex="0" class="btn btn-ghost lg:hidden">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-5 w-5"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M4 6h16M4 12h8m-8 6h16"
              />
            </svg>
          </label>
          <ul
            tabindex="0"
            class="menu menu-sm dropdown-content bg-accent text-accent-content rounded-box z-[1] w-52 p-2 shadow"
          >
            <li v-for="tab in tabs" :key="tab.label">
              <RouterLink :to="tab.url">{{ tab.label }}</RouterLink>
            </li>
          </ul>
        </div>
        <RouterLink
          to="/"
          aria-current="page"
          aria-label="Homepage"
          class="btn-ghost btn px-2"
        >
          <div
            class="text-primary inline-flex items-center text-lg transition-all duration-200 md:text-3xl"
          >
            <img alt="logo" :src="logo" class="w-10" />
            <span class="text-accent"
              >L<span class="lowercase">lama</span></span
            >
            <span class="text-base-content"
              >B<span class="lowercase">et</span></span
            >
          </div>
        </RouterLink>
      </div>

      <div class="navbar-center hidden lg:flex">
        <ul
          class="menu menu-horizontal text-primary-content px-1 text-base font-medium"
        >
          <li v-for="tab in tabs" :key="tab.label" class="bg-primary/20 text-primary-content mx-1 rounded-md">
            <RouterLink :to="tab.url">{{ tab.label }}</RouterLink>
          </li>
        </ul>
      </div>

      <div class="navbar-end flex gap-4">
        <div v-if="accountStore.activeAccount" class="text-neutral font-500">
          <p v-if="accountStore.api">${{ formatAssetAmount(accountStore.balance) }}</p>
          <p v-else class="loading loading-bars loading md"></p>
        </div>
        <WalletButton />
        <ThemeChange />
      </div>
    </nav>
  </div>
</template>
