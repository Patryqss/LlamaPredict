<script setup lang="ts">
import { emitter } from "~/main";
import {
  abbreviate,
  formatPctValue,
  formatUSDAmount,
  validateInput,
} from "~/utils";

const market = {
  id: 2,
  title: "BTC closing price",
  description:
    "Will the value of Bitcoin be above $50,000.00 on 18 February 2024?",
  minValue: 20_000,
  maxValue: 60_000,
  expireDate: new Date("18-02-2024"),
  source: "5E7UzafXNhFLhuRNEekUd9xM7Sc1bffMfnGEyGGtsY3uW4cF",
};

const state = reactive({
  myPrediction: null as boolean | null,
  betSize: "",
  predictError: "",
  betError: "",
  isLoading: false, // TODO: fetch market data and change this to false
  maxWin: 0,
  slippage: 0,
  fee: 0,
});

const position = reactive({
  prediction: null,
  size: 0,
  PnL: 0,
  currentValue: 0,
});

function onPredict(value: boolean) {
  state.predictError = "";
  state.myPrediction = value;
}
function onBetChange(value: string) {
  state.betError = "";
  state.betSize = value;
  state.betError = validateInput(
    value,
    0,
    Number.POSITIVE_INFINITY, // TODO: change to users balance(?)
    2,
  );

  if (!state.betError) calculateStats();
}

function calculateStats() {
  // TODO
}

function onClose() {
  // TODO
}

function onSubmit() {
  if (state.myPrediction === null || !state.betSize) {
    if (state.myPrediction === null)
      state.predictError = "A choice is required";
    if (!state.betSize) state.betError = "This value is required";
    return;
  }
  // TODO: perform actual txn

  emitter.emit("txn-success", "r812rc08723r8c2b083rb702873b");
  state.myPrediction = null;
  state.betSize = "";
}
</script>

<template>
  <Card title="Market Guess" :subtitle="market.description">
    <div class="flex flex-col gap-x-10 md:flex-row">
      <div :style="{ flex: '3 3 0%' }">
        <div>
          <MarketChart
            :min-value="market.minValue"
            :max-value="market.maxValue"
          />
        </div>
        <div class="mb-5 text-sm">
          <span>Resolution Source:</span>
          <a
            class="ml-1 underline"
            :href="`https://alephzero.subscan.io/account/${market.source}`"
            target="_blank"
            rel="noopener noreferrer"
            >{{ abbreviate(market.source, 10) }}</a
          >
        </div>
        <div class="bg-primary/20 rounded-lg px-5 py-2">
          <p class="mb-5 text-xl font-bold">Position</p>
          <div
            class="grid grid-cols-2 justify-between sm:grid-cols-5 sm:items-end"
          >
            <div>
              <p class="stat-title">Prediction</p>
              <p class="stat-value text-2xl">
                {{
                  position.prediction === null
                    ? "-"
                    : position.prediction
                      ? "YES"
                      : "NO"
                }}
              </p>
            </div>
            <div>
              <p class="stat-title">Size</p>
              <p class="stat-value text-2xl">
                {{ formatUSDAmount(position.size) }}
              </p>
            </div>
            <div>
              <p class="stat-title">PnL</p>
              <p class="stat-value text-2xl">
                {{ formatUSDAmount(position.PnL) }}
              </p>
            </div>
            <div>
              <p class="stat-title">Current Value</p>
              <p class="stat-value text-2xl">
                {{ formatUSDAmount(position.currentValue) }}
              </p>
            </div>
            <button
              class="btn btn-accent col-span-2 mt-3 sm:col-span-1 md:mt-0"
              @click="onClose"
            >
              Close
            </button>
          </div>
        </div>
      </div>
      <div class="mt-10 flex-1 md:mt-0">
        <p class="label-text mb-2 ml-2">Prediction</p>
        <div class="flex gap-x-5">
          <button
            class="btn btn-primary flex-1"
            :class="
              state.myPrediction === false &&
              'outline-accent outline outline-4 outline-offset-2'
            "
            @click="() => onPredict(false)"
          >
            No
          </button>
          <button
            class="btn btn-primary flex-1"
            :class="
              state.myPrediction === true &&
              'outline-accent outline outline-4 outline-offset-2'
            "
            @click="() => onPredict(true)"
          >
            Yes
          </button>
        </div>
        <div v-if="state.predictError" class="label">
          <p class="text-error label-text-alt">{{ state.predictError }}</p>
        </div>

        <NumberInput
          class="my-5"
          :value="state.betSize"
          label="Bet Size"
          :error="state.betError"
          @input="onBetChange"
        />
        <div class="flex w-full items-center justify-between">
          <p>Max Win:</p>
          <p class="text-right">
            {{ formatUSDAmount(state.maxWin) }}
          </p>
        </div>
        <div class="my-3 flex w-full items-center justify-between">
          <p>Slippage:</p>
          <p class="text-right">
            {{ formatPctValue(state.slippage) }}
          </p>
        </div>
        <div class="flex w-full items-center justify-between">
          <p>Fee:</p>
          <p class="text-right">
            {{ formatPctValue(state.fee) }}
          </p>
        </div>

        <button
          class="btn btn-accent mt-5 w-full"
          :disabled="!!state.betError || !!state.predictError"
          @click="onSubmit"
        >
          <span
            v-if="state.isLoading"
            class="loading loading-spinner loading-md"
          />
          Submit
        </button>
      </div>
    </div>
  </Card>
</template>
