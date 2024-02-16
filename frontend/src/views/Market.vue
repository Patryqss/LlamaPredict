<script setup lang="ts">
import { formatPctValue, formatUSDAmount, validateInput } from "~/utils";

const market = {
  id: 2,
  title: "BTC closing price",
  description: "What will be the value of Bitcoin on 18 February 2024?",
  minValue: 20_000,
  maxValue: 60_000,
  expireDate: new Date("18-02-2024"),
};

const state = reactive({
  myPrediction: "",
  betSize: "",
  predictError: "",
  betError: "",
  isLoading: true, // TODO: fetch market data and change this to false
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

function onPreditcionChange(value: string) {
  state.myPrediction = value;
  state.predictError = validateInput(
    value,
    market.minValue,
    market.maxValue,
    2,
  );

  if (!state.predictError)
    calculateStats();
}
function onBetChange(value: string) {
  state.betSize = value;
  state.betError = validateInput(
    value,
    0,
    Number.POSITIVE_INFINITY, // TODO: change to users balance(?)
    2,
  );

  if (!state.betError)
    calculateStats();
}

function calculateStats() {
  // TODO
}

function onClose() {
  // TODO
}

function onSubmit() {
  // TODO
}
</script>

<template>
  <Card title="Market Guess: $45,000.00">
    <div class="flex gap-x-10">
      <div :style="{ flex: '3 3 0%' }">
        <div>
          CHART WILL BE HERE
        </div>
        <div class="bg-primary/20 px-5 py-2 rounded-lg">
          <p class="text-xl font-bold mb-5">
            Position
          </p>
          <div class="flex items-center justify-between">
            <div>
              <p class="font-bold">
                Target
              </p>
              <p>{{ formatUSDAmount(position.target) }}</p>
            </div>
            <div>
              <p class="font-bold">
                Size
              </p>
              <p>{{ formatUSDAmount(position.size) }}</p>
            </div>
            <div>
              <p class="font-bold">
                PnL
              </p>
              <p>{{ formatUSDAmount(position.PnL) }}</p>
            </div>
            <div>
              <p class="font-bold">
                Current Value
              </p>
              <p>{{ formatUSDAmount(position.currentValue) }}</p>
            </div>
            <button class="btn btn-accent" @click="onClose">
              Close
            </button>
          </div>
        </div>
      </div>
      <div class="flex-1">
        <NumberInput
          :value="state.myPrediction"
          label="My prediction"
          :error="state.predictError"
          @input="onPreditcionChange"
        />

        <NumberInput
          class="my-5"
          :value="state.betSize"
          label="Bet Size"
          :error="state.betError"
          @input="onBetChange"
        />

        <div class="flex w-full justify-between items-center">
          <p>Max Win:</p>
          <p class="text-right">
            {{ formatUSDAmount(state.maxWin) }}
          </p>
        </div>
        <div class="flex w-full my-3 justify-between items-center">
          <p>Slippage:</p>
          <p class="text-right">
            {{ formatPctValue(state.slippage) }}
          </p>
        </div>
        <div class="flex w-full justify-between items-center">
          <p>Fee:</p>
          <p class="text-right">
            {{ formatPctValue(state.fee) }}
          </p>
        </div>

        <button class="mt-5 w-full btn btn-accent" @click="onSubmit">
          Submit
        </button>
      </div>
    </div>
  </Card>
</template>
