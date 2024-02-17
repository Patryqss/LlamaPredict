<script lang="ts" setup>
import { modalsID } from "~/config";
import { emitter } from "~/main";

const initialModalsData = {
  accountsSelectOpened: false,
  txnHash: null as string | null,
};

const state = reactive({ ...initialModalsData });

onMounted(() => {
  emitter.on("select-account", openAccountsModal);
  emitter.on("txn-success", openTxnSuccessModal);
});

function openAccountsModal() {
  closeModals();
  state.accountsSelectOpened = true;
  openModal(modalsID.SELECT_ACCOUNT);
}
function openTxnSuccessModal(hash: string) {
  closeModals();
  state.txnHash = hash;
  openModal(modalsID.TXN_SUCCESS);
}
async function openModal(id: string) {
  await nextTick(); // wait for modal that is going to be rendered
  const modal = document.getElementById(id);
  if (modal) (modal as any).showModal();
  else console.error("Modal could not be opened");
}
function closeModals() {
  Object.assign(state, { ...initialModalsData });
}
</script>

<template>
  <div>
    <SelectAccountModal
      v-if="state.accountsSelectOpened"
      :key="state.accountsSelectOpened.toString()"
    />
    <TxnSuccessModal
      v-if="state.txnHash"
      :key="state.txnHash"
      :txn-hash="state.txnHash"
    />
  </div>
</template>
