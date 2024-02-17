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
  description: "What will be the value of Bitcoin on 18 February 2024?",
  minValue: 20_000,
  maxValue: 60_000,
  expireDate: new Date("18-02-2024"),
  source: "5E7UzafXNhFLhuRNEekUd9xM7Sc1bffMfnGEyGGtsY3uW4cF",
};

const state = reactive({
  myPrediction: "",
  myPredictionSlider: "0",
  betSize: "",
  predictError: "",
  betError: "",
  isLoading: false, // TODO: fetch market data and change this to false
  maxWin: 0,
  slippage: 0,
  fee: 0,
});

const position = reactive({
  target: 0,
  size: 0,
  PnL: 0,
  currentValue: 0,
});

watch(
  () => state.myPredictionSlider,
  () => onPreditcionChange(state.myPredictionSlider),
);

function onPreditcionChange(value: string) {
  state.predictError = "";
  state.myPrediction = value;
  state.myPredictionSlider = value;
  state.predictError = validateInput(
    value,
    market.minValue,
    market.maxValue,
    2,
  );

  if (!state.predictError) calculateStats();
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
  if (state.predictError || state.betError) return;
  console.log(state.myPrediction);
  // TODO: perform actual txn
  emitter.emit("txn-success", "r812rc08723r8c2b083rb702873b");
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
              <p class="stat-title">Target</p>
              <p class="stat-value text-2xl">
                {{ formatUSDAmount(position.target) }}
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
        <NumberInput
          :value="state.myPrediction"
          label="My prediction"
          :error="state.predictError"
          @input="onPreditcionChange"
        />
        <input
          v-model="state.myPredictionSlider"
          type="range"
          step="0.01"
          :min="market.minValue"
          :max="market.maxValue"
          class="range range-accent mt-3"
        />
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
