<script setup lang="ts">
import { ADMIN_ADDRESS } from "~/config";
import { router } from "~/router";

const state = reactive({
  title: "",
  description: "",
  expireDate: "",
  isLoading: false,
  error: "",
});

onBeforeMount(() => {
  if (accountStore.activeAccount !== ADMIN_ADDRESS) router.push("/");
});

async function addMarket() {
  if (!state.title || !state.description || !state.expireDate) {
    state.error = "All inputs are required";
    return;
  }
  state.error = "";
  state.isLoading = true;

  try {
    await accountStore.addMarket(
      state.title,
      state.description,
      state.expireDate,
    );
  } catch (e) {
    console.error(e);
  }

  state.isLoading = false;
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
        <span class="label-text text-neutral-content">Expire Date</span>
      </label>
      <input
        v-model="state.expireDate"
        type="datetime-local"
        class="input input-bordered input-primary text-neutral-content rounded-lg bg-opacity-20"
        autocomplete="off"
      />
      <div class="form-control mt-5 opacity-70 w-max">
      <label class="cursor-pointer label">
        <span class="label-text mr-3">Private</span>
        <input type="checkbox" disabled class="checkbox" />
      </label>
    </div>

      <button class="btn btn-primary mt-5" @click="addMarket">
        <span
          v-if="state.isLoading"
          class="loading loading-spinner loading-md"
        />
        ADD
      </button>
      <p v-if="state.error" class="text-error mt-1 text-center text-sm">
        {{ state.error }}
      </p>
    </div>
  </Card>
</template>
