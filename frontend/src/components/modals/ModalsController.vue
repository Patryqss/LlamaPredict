<script lang="ts" setup>
import { modalsID } from "~/config";
import { emitter } from "~/main";

const initialModalsData = {
  accountsSelectOpened: false,
};

const state = reactive({ ...initialModalsData });

onMounted(() => {
  emitter.on("select-account", openAccountsModal);
});

function openAccountsModal() {
  closeModals();
  state.accountsSelectOpened = true;
  openModal(modalsID.SELECT_ACCOUNT);
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
  </div>
</template>
