<script setup lang="ts">
const markets = [
  {
    id: 1,
    title: "AZERO closing price",
    description: "What will be the value of AZERO on 18 February 2024?",
    minValue: 0.5,
    maxValue: 5,
    expireDate: new Date("18-02-2024"),
  },
  {
    id: 2,
    title: "BTC closing price",
    description: "What will be the value of Bitcoin on 18 February 2024?",
    minValue: 20_000,
    maxValue: 60_000,
    expireDate: new Date("18-02-2024"),
  },
  {
    id: 3,
    title: "ETH closing price",
    description: "What will be the value of Bitcoin on 18 February 2024?",
    minValue: 1_000,
    maxValue: 20_000,
    expireDate: new Date("22-02-2024"),
  },
  {
    id: 4,
    title: "Doge closing price",
    description: "What will be the value of Bitcoin on 18 February 2024?",
    minValue: 0.001,
    maxValue: 1,
    expireDate: new Date("23-02-2024"),
  },
];
</script>

<template>
  <div class="min-h-screen text-primary-content">
    <h1 class="mb-20 text-5xl font-bold">
      All Markets
    </h1>

    <div class="w-full grid md:grid-cols-3 grid-cols-1 gap-10">
      <RouterLink
        v-for="market of markets"
        :key="market.title"
        :to="`/market/${market.id}`"
      >
        <MarketCard
          :title="market.title"
          :description="market.description"
          :min-value="market.minValue"
          :max-value="market.maxValue"
          :expire-date="market.expireDate"
        />
      </RouterLink>
    </div>
  </div>
</template>

<style scoped></style>
