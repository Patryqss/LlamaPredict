<script setup lang="ts">
import jsonMarkets from "~/markets/markets.json";
import { Market } from "~/types";
import { parseMarket, getMarketHash } from "~/utils";

const markets = ref([] as Market[]);

onBeforeMount(async () => {
  const rawMarkets = await accountStore.getMarkets();
  if (!rawMarkets) return;

  jsonMarkets.forEach((market) => {
    const marketHash = getMarketHash(market);
    const matchingMarketId = rawMarkets.findIndex(
      (m) => m.market.hash_ === marketHash,
    );
    if (matchingMarketId !== -1) {
      const matchingMarket = rawMarkets[matchingMarketId];
      markets.value.push(parseMarket(market, matchingMarketId, matchingMarket));
    }
  });
});
</script>

<template>
  <div class="text-primary-content min-h-screen">
    <h1 class="my-10 text-5xl font-bold">Active Markets</h1>

    <div class="grid w-full grid-cols-1 gap-10 md:grid-cols-2 lg:grid-cols-3">
      <RouterLink
        v-for="market of markets"
        :key="market.title"
        :to="`/market/${market.id}`"
      >
        <MarketCard
          :title="market.title"
          :description="market.description"
          :short-pct="market.shortPct"
          :long-pct="market.longPct"
          :expire-date="market.expireDate"
        />
      </RouterLink>
    </div>
  </div>
</template>
