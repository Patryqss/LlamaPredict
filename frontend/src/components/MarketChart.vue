<script lang="ts" setup>
import ApexChart from "vue3-apexcharts";
import { format } from "date-fns";
import { formatPctValue } from "~/utils";

const dateFormat = "MMM dd, HH:mm";

const state = reactive({
  isLoading: false,
  activeData: {
    lastPoint: -1,
    date: "-",
    value: 0,
    display: "-",
  },
  xAxisData: [] as string[],
  yAxisData: [] as number[],
});

onBeforeMount(() => {
  // Mocked data for hackathon purposes. Normally it would be fetched from BE
  state.xAxisData = [
    new Date("02-16-2024 02:13").toString(),
    new Date("02-16-2024 04:58").toString(),
    new Date("02-16-2024 08:58").toString(),
    new Date("02-16-2024 12:01").toString(),
    new Date("02-16-2024 13:23").toString(),
    new Date("02-16-2024 14:23").toString(),
  ];
  state.yAxisData = [50, 34, 62, 70, 48, 30];
});

const hasData = computed(
  () => state.xAxisData.length > 0 && state.yAxisData.length > 0,
);
const chartOptions = computed(() => ({
  chart: {
    type: "area",
    zoom: { enabled: false },
    toolbar: { show: false },
  },
  xaxis: {
    type: "datetime",
    axisBorder: { show: false },
    axisTicks: { show: false },
    labels: { style: { colors: "#FFF" }, datetimeUTC: false },
    tooltip: { enabled: false },
    categories: state.xAxisData,
  },
  yaxis: {
    show: true,
    labels: {
      style: { colors: "#FFF" },
      formatter: (v: number) => formatPctValue(v),
    },
    min: 0,
    max: 100,
  },
  tooltip: {
    cssClass: "chart-tooltip",
    x: {
      formatter: (_: any, data: any) => {
        if (typeof data === "object")
          return format(
            new Date(state.xAxisData[data.dataPointIndex]),
            dateFormat,
          );
      },
    },
  },
  dataLabels: {
    enabled: false,
  },
  stroke: {
    width: 1,
    curve: "smooth",
  },
  markers: { hover: { size: 2 } },
  fill: {
    gradient: {
      type: "vertical",
      opacityFrom: 0.6,
      opacityTo: 0,
    },
  },
  colors: ["#FFF"],
  grid: {
    show: false,
  },
}));
const chartData = computed(() => [
  {
    name: "Prediction",
    data: state.yAxisData,
  },
]);

watch(
  () => state.yAxisData,
  () => setActiveDataToLastPoint(),
);

onMounted(() => {
  setActiveDataToLastPoint();
});

function setActiveDataToLastPoint() {
  if (!hasData.value) {
    state.activeData = {
      lastPoint: -1,
      date: "-",
      value: 0,
      display: "-",
    };
    return;
  }

  state.activeData = {
    date: format(
      new Date(state.xAxisData[state.xAxisData.length - 1]),
      dateFormat,
    ),
    value: state.yAxisData[state.yAxisData.length - 1],
    lastPoint: -1,
    display: "",
  };
  formatDisplayedData();
}
function updateHoveredData(
  event: any,
  chartContext: any,
  { seriesIndex, dataPointIndex }: any,
) {
  if (
    seriesIndex >= 0 &&
    dataPointIndex >= 0 &&
    state.activeData.lastPoint !== dataPointIndex
  ) {
    state.activeData = {
      date: format(new Date(state.xAxisData[dataPointIndex]), dateFormat),
      value: state.yAxisData[dataPointIndex],
      lastPoint: dataPointIndex,
      display: "",
    };
    formatDisplayedData();
  }
}
function formatDisplayedData() {
  state.activeData.display = formatPctValue(state.activeData.value);
}
</script>

<template>
  <div>
    <div class="mb-1 flex items-start justify-between lg:items-center">
      <div>
        <p class="text-xl sm:text-3xl">
          <span>{{ state.activeData.display }}</span>
        </p>
        <p class="flex space-x-1 text-xs sm:text-sm">
          <span class="tracking-wider">Market Confidence</span>
          <span class="uppercase tracking-wider opacity-50">
            {{ state.activeData.date }}
          </span>
        </p>
      </div>
    </div>
    <div class="w-full" :class="state.isLoading && 'flex justify-center'">
      <div
        v-if="state.isLoading"
        class="loading loading-bars loading-lg mx-auto my-8"
      />
      <ApexChart
        v-else-if="hasData"
        type="area"
        height="400px"
        :options="chartOptions"
        :series="chartData"
        @click="updateHoveredData"
        @mouse-move="updateHoveredData"
        @mouse-leave="setActiveDataToLastPoint"
      />
      <p v-else class="text-gray-lighter my-8 mx-auto text-center">
        No data found
      </p>
    </div>
  </div>
</template>
