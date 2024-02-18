<script setup lang="ts">
import { modalsID } from "~/config";
import { abbreviate } from "~/utils";

const accounts = accountStore.accounts.map((a) => a.address);
const chosenAccount = ref(accounts[0]);

function onAccountSelect(account: string) {
  chosenAccount.value = account;
}

function onConfirm() {
  accountStore.changeActiveAccount(chosenAccount.value);

  const dialog = document.getElementById(modalsID.SELECT_ACCOUNT);
  (dialog as any).close();
}
</script>

<template>
  <BaseModal
    :id="modalsID.SELECT_ACCOUNT"
    title="Select Active Account"
    btn-text="Confirm"
    @submit="onConfirm"
  >
    <div>
      <div v-for="account in accounts" :key="account">
        <label
          class="label cursor-pointer"
          @click="() => onAccountSelect(account)"
        >
          <span class="label-text">{{ abbreviate(account, 10) }}</span>
          <input
            type="radio"
            class="radio checked:bg-accent"
            :checked="chosenAccount === account"
          />
        </label>
      </div>
    </div>
  </BaseModal>
</template>
