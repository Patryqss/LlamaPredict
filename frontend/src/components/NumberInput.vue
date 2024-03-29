<script lang="ts" setup>
const props = defineProps({
  value: {
    // https://forum.vuejs.org/t/how-to-define-a-nullable-prop/117642/4
    type: null as unknown as PropType<string | number | null>,
    default: null,
  },
  label: {
    type: String,
    default: "Amount",
  },
  max: {
    type: Number,
    default: 0,
  },
  error: {
    type: String,
    default: "",
  },
});

watch(
  () => props.value,
  () => (inputValue.value = props.value),
);

const emit = defineEmits(["input"]);
const inputValue = ref(props.value);

function setMax() {
  inputValue.value = props.max;
  emit("input", props.max);
}

function onKeyPress(e: KeyboardEvent) {
  const allowedKeys = /[0-9]|\.|,/;
  if (
    !allowedKeys.test(e.key) ||
    ((e.key === "." || e.key === ",") &&
      inputValue.value!.toString().includes("."))
  )
    e.preventDefault();
}

function onInput() {
  const toEmit = inputValue
    .value!.toString()
    .replace(",", ".")
    .replaceAll(",", "")
    .replaceAll(" ", "")
    .replace(/\..*/, (c) => "." + c.replace(/\./g, () => ""));
  // replace first comma with dot and then reomve all other commas and whitespaces and in the end remove all dots except the first one
  // it's ugly but also the only way to make it work on mobiles since preventDefault() cannot be triggered there like on desktop
  if (Number(toEmit) || Number(toEmit) === 0) emit("input", toEmit);
}
</script>

<template>
  <div class="form-control relative w-full">
    <label class="label">
      <span class="label-text text-neutral-content">{{ label }}</span>
    </label>
    <div class="relative">
      <input
        v-model="inputValue"
        type="text"
        class="input input-bordered input-primary text-neutral-content w-full rounded-lg bg-opacity-20 pl-6"
        :class="error ? 'input-error' : 'input-primary'"
        pattern="/[0-9]|\.|,/"
        inputmode="decimal"
        lang="en-US"
        placeholder="0.00"
        autocomplete="off"
        @keypress="onKeyPress"
        @input="onInput"
      />
      <p
        class="pointer-events-none absolute left-2 flex items-center text-lg sm:text-2xl"
        :style="{ top: '50%', transform: 'translateY(-50%)' }"
      >
        <span class="text-neutral-content ml-1 text-base">$</span>
      </p>
    </div>
    <label v-if="error" class="label">
      <span class="label-text-alt text-error">{{ error }}</span>
    </label>
    <label v-if="max" class="label">
      <span></span>
      <span class="label-text-alt text-neutral-content">
        <span class="text-accent cursor-pointer" @click="setMax">MAX</span>
      </span>
    </label>
  </div>
</template>
