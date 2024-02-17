<script setup lang="ts">
const markets = [
  {
    id: 1,
    title: "AZERO closing price",
    description: "What will be the value of AZERO on 18 February 2024?",
    shortPct: 20,
    longPct: 80,
    expireDate: new Date("02-17-2024 21:37"),
  },
  {
    id: 2,
    title: "BTC closing price",
    description: "What will be the value of Bitcoin on 19 February 2024?",
    shortPct: 50,
    longPct: 50,
    expireDate: new Date("02-19-2024 05:15"),
  },
  {
    id: 3,
    title: "ETH closing price",
    description: "What will be the value of ETH on 22 February 2024?",
    shortPct: 55,
    longPct: 45,
    expireDate: new Date("02-22-2024 12:00"),
  },
  {
    id: 4,
    title: "Doge closing price",
    description: "What will be the value of Doge on 23 February 2024?",
    shortPct: 73,
    longPct: 27,
    expireDate: new Date("02-23-2024 15:00"),
  },
];
</script>

<template>
  <div class="text-primary-content min-h-screen">
    <h1 class="my-10 text-5xl font-bold">All Markets</h1>

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

<style scoped></style>
