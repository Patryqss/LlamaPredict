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
  loadingClose: false,
  maxWin: 0,
  slippage: 0,
});

const position = reactive({
  prediction: null as string | null,
  invested: 0,
  PnL: 0,
  currentValue: 0,
  balanceA: 0,
  balanceB: 0,
});

const route = useRoute();
const market = ref(null as Market | null);

watch(
  () => accountStore.activeAccount,
  () => updatePosition(),
);

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

  updatePosition();
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
  calculateStats();
}
function onAmountChange(value: string) {
  state.amountError = "";
  state.amount = value;
  state.amountError = validateInput(value, 0, accountStore.balance, 6);

  if (!state.amountError) calculateStats();
}
async function updatePosition() {
  const positionData = await accountStore.getPosition(Number(route.params.id));
  console.log(positionData);
  const marketData = await accountStore.getUserMarketData(
    Number(route.params.id),
  );

  position.balanceA = positionData.balanceA;
  position.balanceB = positionData.balanceB;
  position.currentValue = positionData.positionValue;
  position.invested = marketData.deposited - marketData.claimed;
  position.PnL = position.currentValue - position.invested;
  if (positionData.balanceA > positionData.balanceB)
    position.prediction = "YES";
  else if (positionData.balanceA < positionData.balanceB)
    position.prediction = "NO";
}
async function calculateStats() {
  if (state.myPrediction !== null && Number(state.amount) > 0) {
    const predictionStats = await accountStore.getPredictionStats(
      Number(route.params.id),
      Number(state.amount),
      state.myPrediction ? "A" : "B",
    );
    state.maxWin = predictionStats.maxWin;
    state.slippage = predictionStats.slippage;
  }
}
async function onClose() {
  state.loadingClose = true;

  try {
    const txnHash = await accountStore.closeMarket(Number(route.params.id), position.balanceA, position.balanceB);

    if (txnHash) emitter.emit("txn-success", txnHash);
  } catch(e) {
    console.error(e);
  }

  accountStore.udpateBalance();
  updatePosition();
  state.loadingClose = false;
}
async function onSubmit() {
  state.loadingTxn = true;

  if (state.type === "ADD_LIQ") await onAddLiq();
  else await onPredict();

  accountStore.udpateBalance();
  updatePosition();
  state.loadingTxn = false;
}
async function onAddLiq() {
  if (!state.amount) {
    if (!state.amount) state.amountError = "This value is required";
    return;
  }

  try {
    const txnHash = await accountStore.addLiquidity(
      Number(route.params.id),
      Number(state.amount),
    );

    if (txnHash) emitter.emit("txn-success", txnHash);
  } catch (e) {
    console.error(e);
  }
  state.amount = "";
}
async function onPredict() {
  if (state.myPrediction === null || !state.amount) {
    if (state.myPrediction === null)
      state.predictError = "A choice is required";
    if (!state.amount) state.amountError = "This value is required";
    return;
  }

  try {
    const txnHash = await accountStore.predict(
      Number(route.params.id),
      Number(state.amount),
      state.myPrediction ? "A" : "B",
    );

    if (txnHash) emitter.emit("txn-success", txnHash);
    state.maxWin = 0;
  } catch (e) {
    console.error(e);
  }

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
                }}
              </p>
            </div>
            <div>
              <p class="stat-title">Invested</p>
              <p class="stat-value text-2xl">
                {{ formatUSDAmount(position.invested) }}
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
              class="btn btn-primary col-span-2 mt-3 sm:col-span-1 md:mt-0"
              :disabled="!accountStore.activeAccount"
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
                state.myPrediction === true &&
                'outline-accent outline outline-4 outline-offset-2'
              "
              @click="() => onPredictChoice(true)"
            >
              Yes
            </button>
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
        </div>

        <button
          class="btn btn-primary mt-5 w-full"
          :disabled="
            !!state.amountError ||
            !!state.predictError ||
            !accountStore.activeAccount
          "
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
