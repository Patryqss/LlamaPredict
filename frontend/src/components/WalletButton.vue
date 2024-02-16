<script setup lang="ts">
import {
  ClipboardCopyIcon,
  CreditCardIcon,
  LogoutIcon,
} from "@heroicons/vue/outline";
import { emitter } from "~/main";
import { abbreviate } from "~/utils";

const copied = ref(false);

async function onAddressCopy() {
  await navigator.clipboard.writeText(accountStore.activeAccount!);
  copied.value = true;

  setTimeout(() => {
    copied.value = false;
  }, 5 * 1000); // allow to copy again after 5 seconds
}

async function onConnect() {
  await accountStore.connectToSigner();
}

async function onAccountChange() {
  emitter.emit("select-account");
}

function onDisconnect() {
  accountStore.disconnect();
}
</script>

<template>
  <button
    v-if="!accountStore.activeAccount"
    class="btn-accent btn"
    @click="onConnect"
  >
    <span v-if="accountStore.loading" class="loading loading-spinner" />
    <span v-else>Connect wallet</span>
  </button>
  <div v-else class="dropdown dropdown-end">
    <label tabindex="0" class="btn-accent btn">{{
      abbreviate(accountStore.activeAccount)
    }}</label>
    <ul
      tabindex="0"
      class="dropdown-content z-[1] menu mt-1 p-2 shadow bg-accent text-accent-content rounded-box w-52"
    >
      <li>
        <div class="flex" @click="onAddressCopy">
          <ClipboardCopyIcon class="w-4 shrink-0" />
          <span v-if="copied">Copied!</span>
          <span v-else>Copy Address</span>
        </div>
      </li>
      <li v-if="accountStore.accounts.length > 1">
        <div class="flex" @click="onAccountChange">
          <CreditCardIcon class="w-4 shrink-0" />
          <span>Change Account</span>
        </div>
      </li>
      <hr class="my-1">
      <li>
        <div class="flex" @click="onDisconnect">
          <LogoutIcon class="w-4 shrink-0" />
          <span>Disconnect</span>
        </div>
      </li>
    </ul>
  </div>
</template>
