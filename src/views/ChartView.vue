<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from "vue";
import * as echarts from "echarts";
import * as api from "../lib/api";
import { getSkin, applySkin } from "../skins";

type Range = "24h" | "7d" | "all";

const range = ref<Range>("7d");
const chartEl = ref<HTMLDivElement | null>(null);
const emptyMsg = ref("");
let chart: echarts.ECharts | null = null;
let history: api.History = { snapshots: [], recharges: [] };

const ranges: { id: Range; label: string }[] = [
  { id: "24h", label: "24 小时" },
  { id: "7d", label: "7 天" },
  { id: "all", label: "全部" },
];

function cssVar(name: string): string {
  return getComputedStyle(document.documentElement).getPropertyValue(name).trim();
}

function filterSnaps(): api.Snapshot[] {
  const now = Math.floor(Date.now() / 1000);
  const cut =
    range.value === "24h" ? now - 86400 : range.value === "7d" ? now - 604800 : 0;
  const snaps =
    cut > 0 ? history.snapshots.filter((s) => s.t >= cut) : history.snapshots;
  return snaps;
}

function buildMarkers(snaps: api.Snapshot[]) {
  const now = Math.floor(Date.now() / 1000);
  const cut =
    range.value === "24h" ? now - 86400 : range.value === "7d" ? now - 604800 : 0;

  const inRange = history.recharges
    .filter((r) => r.t >= (cut > 0 ? cut : 0))
    .sort((a, b) => b.t - a.t)
    .slice(0, 8);

  const lines: { name: string; xAxis: number }[] = [];
  const points: { name: string; coord: [number, number]; value: string }[] = [];
  const symbol = api.currencySymbol(snaps[snaps.length - 1]?.currency ?? "CNY");

  for (const r of inRange) {
    let total = snaps.find((s) => s.t >= r.t)?.total;
    if (total === undefined) total = snaps[snaps.length - 1]?.total ?? 0;
    const label = "+" + symbol + api.formatAmount(r.amount);
    lines.push({ name: "", xAxis: r.t * 1000 });
    points.push({
      name: label,
      coord: [r.t * 1000, total],
      value: label,
    });
  }
  return { lines, points };
}

function render() {
  const snaps = filterSnaps();
  if (snaps.length < 1) {
    emptyMsg.value = history.snapshots.length === 0 ? "暂无历史数据，等待自动刷新即可积累记录" : "当前时间范围内暂无数据";
    chart?.clear();
    return;
  }
  emptyMsg.value = "";

  const accent = cssVar("--skin-accent");
  const subText = cssVar("--skin-subtext");
  const border = cssVar("--skin-border");
  const text = cssVar("--skin-text");
  const ok = cssVar("--dot-ok");
  const { lines, points } = buildMarkers(snaps);

  const option: echarts.EChartsOption = {
    backgroundColor: "transparent",
    grid: { left: 66, right: 24, top: 30, bottom: 44 },
    tooltip: {
      trigger: "axis",
      valueFormatter: (v) =>
        typeof v === "number" ? api.currencySymbol(snaps[0]?.currency ?? "CNY") + api.formatAmount(v) : String(v),
    },
    xAxis: {
      type: "time",
      axisLine: { lineStyle: { color: border } },
      axisTick: { show: false },
      axisLabel: {
        color: subText,
        formatter: (val) => {
          const d = new Date(val);
          const p = (n: number) => String(n).padStart(2, "0");
          if (range.value === "24h") return `${p(d.getHours())}:${p(d.getMinutes())}`;
          if (range.value === "7d") return `${p(d.getMonth() + 1)}-${p(d.getDate())} ${p(d.getHours())}:${p(d.getMinutes())}`;
          return `${d.getFullYear()}-${p(d.getMonth() + 1)}-${p(d.getDate())}`;
        },
      },
    },
    yAxis: {
      type: "value",
      scale: true,
      splitLine: { lineStyle: { color: border, type: "dashed" } },
      axisLabel: {
        color: subText,
        formatter: (val) => api.formatAmount(Number(val)),
      },
    },
    series: [
      {
        type: "line",
        data: snaps.map((s) => [s.t * 1000, s.total]),
        smooth: true,
        showSymbol: snaps.length < 40,
        symbolSize: 5,
        lineStyle: { color: accent, width: 2 },
        itemStyle: { color: accent },
        areaStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: accent + "33" },
            { offset: 1, color: accent + "00" },
          ]),
        },
        markLine: lines.length
          ? {
              silent: true,
              symbol: "none",
              lineStyle: { color: ok, type: "dashed", width: 1 },
              label: { show: false },
              data: lines,
            }
          : undefined,
        markPoint: points.length
          ? {
              symbol: "circle",
              symbolSize: 8,
              itemStyle: { color: ok, borderColor: "#fff", borderWidth: 1 },
              label: {
                show: true,
                position: "top",
                formatter: (p) => String(p.name ?? p.value),
                color: ok,
                fontSize: 11,
                fontWeight: "bold",
              },
              data: points,
            }
          : undefined,
      },
    ],
  };

  if (!chart) {
    chart = echarts.init(chartEl.value!);
  }
  chart.setOption(option, true);
  chart.resize();
}

async function load() {
  const cfg = await api.getConfig();
  applySkin(getSkin(cfg.skin, cfg.customSkin));
  history = await api.getHistory();
  render();
}

function setRange(r: Range) {
  range.value = r;
  render();
}

function refresh() {
  load();
}

onMounted(() => {
  load();
  window.addEventListener("resize", onResize);
});

onBeforeUnmount(() => {
  window.removeEventListener("resize", onResize);
  chart?.dispose();
  chart = null;
});

function onResize() {
  chart?.resize();
}
</script>

<template>
  <div class="chart-page">
    <div class="toolbar">
      <div class="seg">
        <button
          v-for="r in ranges"
          :key="r.id"
          class="seg-btn"
          :class="{ active: range === r.id }"
          @click="setRange(r.id)"
        >
          {{ r.label }}
        </button>
      </div>
      <button class="refresh" @click="refresh">⟳ 刷新</button>
    </div>
    <div class="hint" v-if="emptyMsg">{{ emptyMsg }}</div>
    <div ref="chartEl" class="chart" :style="{ display: emptyMsg ? 'none' : 'block' }"></div>
  </div>
</template>

<style scoped>
.chart-page {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 12px 16px;
}

.toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
}

.seg {
  display: flex;
  background: color-mix(in srgb, var(--skin-subtext) 12%, transparent);
  border-radius: 8px;
  padding: 3px;
  gap: 2px;
}

.seg-btn {
  border: none;
  background: transparent;
  color: var(--skin-subtext);
  font-size: 12px;
  padding: 5px 12px;
  border-radius: 6px;
  transition: all 0.15s;
}
.seg-btn.active {
  background: var(--skin-accent);
  color: #fff;
}

.refresh {
  border: 1px solid var(--skin-border);
  background: var(--skin-card);
  color: var(--skin-text);
  font-size: 12px;
  padding: 5px 12px;
  border-radius: 8px;
}

.hint {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--skin-subtext);
  font-size: 13px;
}

.chart {
  flex: 1;
  min-height: 0;
}
</style>
