<script setup lang="ts">
import { emitter } from "~/main";
import { validateInput } from "~/utils";

const state = reactive({
  amount: "",
  isLoading: false,
  error: "",
});

function onAmountChange(value: string) {
  state.error = "";
  state.amount = value;
  state.error = validateInput(value, 0, 1000, 6);
}

async function onMint() {
  if (!state.amount) {
    state.error = "This value is required";
    return;
  }
  state.isLoading = true;

  try {
    const txnHash = await accountStore.mintUSD(state.amount);
    emitter.emit('txn-success', txnHash!);
    state.amount = "";
    accountStore.udpateBalance();
  } catch (e) {
    console.error(e);
  }

  state.isLoading = false;
}
</script>

<template>
  <Card title="Faucet">
    <div class="flex justify-center">
      <div class="w-100 flex max-w-full flex-col">
        <NumberInput
          class="my-5"
          :value="state.amount"
          label="Amount"
          :error="state.error"
          @input="onAmountChange"
        />

        <button
          class="btn btn-primary mt-5"
          :disabled="!!state.error"
          @click="onMint"
        >
          <span
            v-if="state.isLoading"
            class="loading loading-spinner loading-md"
          />
          MINT
        </button>
      </div>
    </div>
  </Card>
</template>
