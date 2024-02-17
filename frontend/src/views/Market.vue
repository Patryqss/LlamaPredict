<script setup lang="ts">
import { ADMIN_ADDRESS } from "~/config";
import { emitter } from "~/main";
import jsonMarkets from "~/markets/markets.json";
import {
  abbreviate,
  formatPctValue,
  formatUSDAmount,
  parseMarket,
  getMarketHash,
  validateInput,
} from "~/utils";
import { Market } from "~/types";

type MarketTxn = "PREDICT" | "ADD_LIQ";

const state = reactive({
  myPrediction: null as boolean | null,
  amount: "",
  predictError: "",
  amountError: "",
  type: "PREDICT" as MarketTxn,
  loadingMarket: true,
  loadingTxn: false,
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

const route = useRoute();
const market = ref(null as Market | null);

onBeforeMount(async () => {
  const rawMarket = await accountStore.getMarket(Number(route.params.id));
  if (!rawMarket) {
    // No market with this id on the blockchain
    state.loadingMarket = false;
    return;
  }
  const matchingJSON = jsonMarkets.find(
    (m) => getMarketHash(m) === rawMarket.market.hash_,
  );
  if (!matchingJSON) {
    // No matching market in our DB
    state.loadingMarket = false;
    return;
  }

  market.value = parseMarket(matchingJSON, Number(route.params.id), rawMarket);
  state.loadingMarket = false;
});

function onTypeChange() {
  state.type = state.type === "ADD_LIQ" ? "PREDICT" : "ADD_LIQ";
  state.amount = "";
  state.myPrediction = null;
  state.predictError = "";
  state.amountError = "";
}
function onPredictChoice(value: boolean) {
  state.predictError = "";
  state.myPrediction = value;
}
function onAmountChange(value: string) {
  state.amountError = "";
  state.amount = value;
  state.amountError = validateInput(value, 0, accountStore.balance, 6);

  if (!state.amountError) calculateStats();
}
function calculateStats() {
  // TODO
}
function onClose() {
  // TODO
}
async function onSubmit() {
  state.loadingTxn = true;

  if (state.type === "ADD_LIQ") await onAddLiq();
  else await onPredict();

  state.loadingTxn = false;
}
async function onAddLiq() {
  if (!state.amount) {
    if (!state.amount) state.amountError = "This value is required";
    return;
  }

  await accountStore.addLiquidity(
    Number(route.params.id),
    Number(state.amount),
  );

  // emitter.emit("txn-success", "r812rc08723r8c2b083rb702873b");
  state.amount = "";
}
async function onPredict() {
  if (state.myPrediction === null || !state.amount) {
    if (state.myPrediction === null)
      state.predictError = "A choice is required";
    if (!state.amount) state.amountError = "This value is required";
    return;
  }

  emitter.emit("txn-success", "r812rc08723r8c2b083rb702873b");
  state.myPrediction = null;
  state.amount = "";
}
</script>

<template>
  <div v-if="state.loadingMarket" class="mt-40 flex w-full justify-center">
    <p class="loading loading-bars loading-lg" />
  </div>
  <Card v-else-if="!market" title="">
    <p class="mb-10 text-center text-xl font-bold">Market not found!</p>
  </Card>
  <Card v-else title="Market Guess" :subtitle="market.description">
    <div class="flex flex-col gap-x-10 md:flex-row">
      <div :style="{ flex: '3 3 0%' }">
        <div>
          <MarketChart />
        </div>
        <div class="mb-5 text-sm">
          <span>Resolution Source:</span>
          <a
            class="ml-1 underline"
            :href="`https://alephzero.subscan.io/account/${ADMIN_ADDRESS}`"
            target="_blank"
            rel="noopener noreferrer"
            >{{ abbreviate(ADMIN_ADDRESS, 10) }}</a
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
        <div class="flex items-center justify-between">
          <p
            class="cursor-pointer font-bold hover:opacity-80"
            :class="state.type === 'PREDICT' && 'text-accent'"
            @click="state.type = 'PREDICT'"
          >
            PREDICT
          </p>
          <input
            type="checkbox"
            class="toggle bg-accent border-accent"
            :checked="state.type === 'ADD_LIQ'"
            @input="onTypeChange"
          />
          <p
            class="cursor-pointer text-right font-bold hover:opacity-80"
            :class="state.type === 'ADD_LIQ' && 'text-accent'"
            @click="state.type = 'ADD_LIQ'"
          >
            ADD LIQUIDITY
          </p>
        </div>

        <div class="divider" />

        <div v-if="state.type === 'PREDICT'">
          <p class="label-text mb-2 ml-2">Prediction</p>
          <div class="flex gap-x-5">
            <button
              class="btn btn-primary flex-1"
              :class="
                state.myPrediction === false &&
                'outline-accent outline outline-4 outline-offset-2'
              "
              @click="() => onPredictChoice(false)"
            >
              No
            </button>
            <button
              class="btn btn-primary flex-1"
              :class="
                state.myPrediction === true &&
                'outline-accent outline outline-4 outline-offset-2'
              "
              @click="() => onPredictChoice(true)"
            >
              Yes
            </button>
          </div>
          <div v-if="state.predictError" class="label">
            <p class="text-error label-text-alt">{{ state.predictError }}</p>
          </div>
        </div>

        <NumberInput
          class="my-5"
          :value="state.amount"
          :label="state.type === 'PREDICT' ? 'Bet Size' : 'Amount'"
          :max="accountStore.balance"
          :error="state.amountError"
          @input="onAmountChange"
        />

        <div v-if="state.type === 'PREDICT'">
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
        </div>

        <button
          class="btn btn-accent mt-5 w-full"
          :disabled="!!state.amountError || !!state.predictError"
          @click="onSubmit"
        >
          <span
            v-if="state.loadingTxn"
            class="loading loading-spinner loading-md"
          />
          Submit
        </button>
      </div>
    </div>
  </Card>
</template>
