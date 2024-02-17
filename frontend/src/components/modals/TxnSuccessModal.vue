<script setup lang="ts">
import { BadgeCheckIcon } from "@heroicons/vue/outline";
import { modalsID } from "~/config";

defineProps({
  txnHash: {
    type: String,
    required: true,
  },
});

function onClose() {
  const dialog = document.getElementById(modalsID.TXN_SUCCESS);
  (dialog as any).close();
}
</script>

<template>
  <BaseModal
    :id="modalsID.TXN_SUCCESS"
    title="Success!"
    btn-text="Close"
    @submit="onClose"
  >
    <div class="text-success text-xl">
      Your transaction has been successfully processed.
    </div>
    <div class="mt-2 flex flex-col items-center">
      <BadgeCheckIcon class="text-success w-32" />
      <p class="text-xs opacity-80">Txn hash: {{ txnHash }}</p>
    </div>
  </BaseModal>
</template>
