<script setup lang="ts">
const state = reactive({
  title: "",
  description: "",
  minValue: "",
  maxValue: "",
  expireDate: "",
  source: "",
});

function addMarket() {
  console.log(state);
  accountStore.addMarket();

  // accountStore.getMarkets();
  // TODO
}
</script>

<template>
  <Card title="Add New Market">
    <div class="w-100 flex max-w-full flex-col">
      <label class="label">
        <span class="label-text text-neutral-content">Title</span>
      </label>
      <input
        v-model="state.title"
        type="text"
        class="input input-bordered input-primary text-neutral-content rounded-lg bg-opacity-20"
        autocomplete="off"
      />
      <label class="label">
        <span class="label-text text-neutral-content">Description</span>
      </label>
      <textarea
        v-model="state.description"
        type="text"
        class="textarea input-bordered input-primary text-neutral-content rounded-lg bg-opacity-20"
        autocomplete="off"
      />
      <label class="label">
        <span class="label-text text-neutral-content">Min Value</span>
      </label>
      <input
        v-model="state.minValue"
        type="text"
        class="input input-bordered input-primary text-neutral-content rounded-lg bg-opacity-20"
        autocomplete="off"
      />
      <label class="label">
        <span class="label-text text-neutral-content">Max Value</span>
      </label>
      <input
        v-model="state.maxValue"
        type="text"
        class="input input-bordered input-primary text-neutral-content rounded-lg bg-opacity-20"
        autocomplete="off"
      />
      <label class="label">
        <span class="label-text text-neutral-content">Expire Date</span>
      </label>
      <input
        v-model="state.expireDate"
        type="datetime-local"
        class="input input-bordered input-primary text-neutral-content rounded-lg bg-opacity-20"
        autocomplete="off"
      />

      <button class="btn btn-primary mt-5" @click="addMarket">ADD</button>
    </div>
  </Card>
</template>
