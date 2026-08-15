<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from "vue";
import * as echarts from "echarts";
import * as api from "../lib/api";
import { getSkin, applySkin } from "../skins";
import { applyGlassWindowEffect } from "../lib/windowEffects";

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
  void applyGlassWindowEffect();
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
    <div class="chart-shell">
      <header class="toolbar">
        <div class="chart-title">
          <h1>余额趋势</h1>
          <p>用量与充值记录会在每次刷新后自动沉淀</p>
        </div>
        <div class="toolbar-actions">
          <div class="seg" aria-label="时间范围">
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
          <button class="refresh" @click="refresh" aria-label="刷新曲线数据">
            <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M20 11a8.1 8.1 0 00-15.5-2.8L3 10"/><path d="M3 4v6h6"/><path d="M4 13a8.1 8.1 0 0015.5 2.8L21 14"/><path d="M21 20v-6h-6"/></svg>
            刷新
          </button>
        </div>
      </header>
      <div class="chart-area">
        <div class="hint" v-if="emptyMsg">{{ emptyMsg }}</div>
        <div ref="chartEl" class="chart" :style="{ display: emptyMsg ? 'none' : 'block' }"></div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.chart-page {
  width: 100%;
  height: 100%;
  padding: 18px;
  background:
    radial-gradient(circle at 7% 0%, color-mix(in srgb, var(--skin-accent) 23%, transparent), transparent 30%),
    radial-gradient(circle at 100% 100%, color-mix(in srgb, var(--skin-bg) 72%, var(--skin-accent)), transparent 42%),
    transparent;
}

.chart-shell {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border: 1px solid color-mix(in srgb, var(--glass-stroke) 85%, transparent);
  border-radius: 20px;
  background:
    linear-gradient(135deg, color-mix(in srgb, #ffffff 34%, transparent), transparent 29%),
    var(--glass-fill-strong);
  box-shadow: 0 18px 48px var(--glass-shadow), inset 0 1px 0 color-mix(in srgb, #ffffff 72%, transparent);
  backdrop-filter: blur(22px) saturate(1.16);
  -webkit-backdrop-filter: blur(22px) saturate(1.16);
}

.toolbar {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 18px;
  padding: 20px 22px 16px;
  border-bottom: 1px solid color-mix(in srgb, var(--skin-border) 56%, transparent);
}

.chart-title h1 {
  font-size: 18px;
  line-height: 1.15;
  letter-spacing: -0.025em;
  font-weight: 740;
}

.chart-title p {
  margin-top: 5px;
  color: var(--skin-subtext);
  font-size: 12px;
}

.toolbar-actions {
  display: flex;
  align-items: center;
  gap: 10px;
}

.seg {
  display: flex;
  padding: 3px;
  gap: 3px;
  border: 1px solid color-mix(in srgb, var(--skin-border) 58%, transparent);
  border-radius: 10px;
  background: color-mix(in srgb, var(--skin-card) 24%, transparent);
}

.seg-btn {
  border: 1px solid transparent;
  background: transparent;
  color: var(--skin-subtext);
  font-size: 12px;
  font-weight: 600;
  padding: 6px 11px;
  border-radius: 7px;
  transition: background 160ms ease, color 160ms ease, box-shadow 160ms ease;
}
.seg-btn.active {
  background: color-mix(in srgb, var(--skin-accent) 88%, var(--skin-card));
  color: #fff;
  box-shadow: 0 4px 11px color-mix(in srgb, var(--skin-accent) 28%, transparent);
}

.refresh {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  border: 1px solid color-mix(in srgb, var(--skin-border) 76%, transparent);
  background: color-mix(in srgb, var(--skin-card) 38%, transparent);
  color: var(--skin-text);
  font-size: 12px;
  font-weight: 600;
  padding: 7px 11px;
  border-radius: 9px;
  transition: transform 160ms ease, background 160ms ease;
}

.refresh:hover {
  background: color-mix(in srgb, var(--skin-card) 64%, transparent);
  transform: translateY(-1px);
}

.refresh svg {
  width: 14px;
  height: 14px;
  fill: none;
  stroke: currentColor;
  stroke-width: 2;
  stroke-linecap: round;
  stroke-linejoin: round;
}

.chart-area {
  position: relative;
  flex: 1;
  min-height: 0;
  display: flex;
  padding: 4px 12px 12px;
}

.hint {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--skin-subtext);
  font-size: 13px;
  text-align: center;
  padding: 32px;
}

.chart {
  flex: 1;
  min-height: 0;
}

@media (max-width: 620px) {
  .chart-page { padding: 10px; }
  .toolbar { padding: 16px; }
  .toolbar-actions { gap: 7px; }
  .chart-title p { display: none; }
  .seg-btn { padding-inline: 8px; }
  .refresh { padding-inline: 9px; }
}
</style>
