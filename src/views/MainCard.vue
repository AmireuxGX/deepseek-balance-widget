<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount } from "vue";
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import { listen } from "@tauri-apps/api/event";
import * as api from "../lib/api";
import { getSkin, applySkin, applyOpacity } from "../skins";

const appWindow = getCurrentWindow();

const config = ref<api.Config | null>(null);
const result = ref<api.BalanceResult | null>(null);
const busy = ref(false);
const prevTotal = ref<number | null>(null);
const delta = ref<number | null>(null);
const lastUpdate = ref<Date | null>(null);
const now = ref(new Date());

let queryTimer: number | null = null;
let clockTimer: number | null = null;

const hasKey = computed(() => !!(config.value?.apiKey?.trim()));

const symbol = computed(() => {
  const info = result.value?.infos?.[0];
  return api.currencySymbol(info?.currency ?? "CNY");
});

const balanceText = computed(() => {
  if (!result.value?.success) return "--";
  const total = result.value.infos[0]?.total ?? 0;
  return symbol.value + api.formatAmount(total);
});

const deltaText = computed(() => {
  if (delta.value === null) return "";
  if (Math.abs(delta.value) < 0.000001) return "— 0.00";
  const sign = delta.value < 0 ? "↓ " : "↑ ";
  return sign + symbol.value + api.formatAmount(Math.abs(delta.value));
});

const deltaClass = computed(() => {
  if (delta.value === null) return "";
  if (Math.abs(delta.value) < 0.000001) return "flat";
  return delta.value < 0 ? "down" : "up";
});

const dotClass = computed(() => {
  if (busy.value) return "busy";
  if (!result.value?.success) return "bad";
  return result.value.isAvailable ? "ok" : "bad";
});

const subText = computed(() => {
  if (busy.value) return "正在查询…";
  if (!result.value?.success) return "查询失败";
  const info = result.value.infos[0];
  const cur = info?.currency ?? "CNY";
  return `总余额 · ${cur} · ${result.value.isAvailable ? "可用" : "余额不足"}`;
});

const detailText = computed(() => {
  const info = result.value?.infos?.[0];
  if (!result.value?.success || !info) {
    const err = result.value?.error ?? "";
    return err.length > 34 ? err.slice(0, 34) + "…" : err;
  }
  return `充值 ${symbol.value}${api.formatAmount(info.topped)} · 赠送 ${symbol.value}${api.formatAmount(info.granted)}`;
});

const footerText = computed(() => {
  if (!lastUpdate.value) return "";
  if (!result.value?.success) return "60 秒后自动重试";
  const sec = Math.floor((now.value.getTime() - lastUpdate.value.getTime()) / 1000);
  const rel =
    sec < 60
      ? "刚刚"
      : sec < 3600
        ? `${Math.floor(sec / 60)} 分钟前`
        : `${Math.floor(sec / 3600)} 小时前`;
  return `${rel} 更新 · 每 ${config.value?.intervalMinutes ?? 5} 分钟刷新`;
});

async function refresh() {
  if (busy.value || !config.value?.apiKey) return;
  busy.value = true;
  try {
    const r = await api.fetchBalance(config.value.apiKey);
    result.value = r;
    if (r.success) {
      const total = r.infos[0]?.total ?? 0;
      if (prevTotal.value !== null) delta.value = total - prevTotal.value;
      prevTotal.value = total;
      lastUpdate.value = new Date();
    }
  } finally {
    busy.value = false;
    now.value = new Date();
  }
}

async function init() {
  config.value = await api.getConfig();
  const skin = getSkin(config.value.skin, config.value.customSkin);
  applySkin(skin);
  applyOpacity(config.value.opacity);
  const iv = Math.max(1, config.value.intervalMinutes) * 60000;
  if (config.value.apiKey) {
    refresh();
    queryTimer = window.setInterval(refresh, iv);
  }
  clockTimer = window.setInterval(() => {
    now.value = new Date();
  }, 30000);
}

async function reloadConfig() {
  config.value = await api.getConfig();
  const skin = getSkin(config.value.skin, config.value.customSkin);
  applySkin(skin);
  applyOpacity(config.value.opacity);
  prevTotal.value = null;
  delta.value = null;
  if (queryTimer) window.clearInterval(queryTimer);
  const iv = Math.max(1, config.value.intervalMinutes) * 60000;
  if (config.value.apiKey) {
    queryTimer = window.setInterval(refresh, iv);
    refresh();
  }
}

let unlisten: (() => void) | null = null;
let previewUnlisten: (() => void) | null = null;
let cancelUnlisten: (() => void) | null = null;
let menuRefreshUnlisten: (() => void) | null = null;

function applyPreview(payload: { skin: string; customSkin: api.CustomSkin | null; opacity: number }) {
  applySkin(getSkin(payload.skin, payload.customSkin));
  applyOpacity(payload.opacity);
}

function cancelPreview() {
  if (config.value) {
    applySkin(getSkin(config.value.skin, config.value.customSkin));
    applyOpacity(config.value.opacity);
  }
}

onMounted(async () => {
  await init();
  unlisten = await listen("config-changed", () => {
    reloadConfig();
  });
  previewUnlisten = await listen<{ skin: string; customSkin: api.CustomSkin | null; opacity: number }>(
    "preview-changed",
    (event) => {
      applyPreview(event.payload);
    }
  );
  cancelUnlisten = await listen("preview-cancel", () => {
    cancelPreview();
  });
  menuRefreshUnlisten = await listen("menu-refresh", () => {
    refresh();
  });
});
onBeforeUnmount(() => {
  if (queryTimer) window.clearInterval(queryTimer);
  if (clockTimer) window.clearInterval(clockTimer);
  if (unlisten) unlisten();
  if (previewUnlisten) previewUnlisten();
  if (cancelUnlisten) cancelUnlisten();
  if (menuRefreshUnlisten) menuRefreshUnlisten();
  window.removeEventListener("mousemove", onResizeMove);
  window.removeEventListener("mouseup", onResizeUp);
});

function onCardMouseDown(e: MouseEvent) {
  if (e.button !== 0) return;
  const t = e.target as HTMLElement;
  if (t.closest(".btn, .resize-handle")) return;
  appWindow.startDragging();
}

let resizing = false;
let rsx = 0;
let rsy = 0;
let rsw = 0;
let rsh = 0;
function onResizeDown(e: MouseEvent) {
  resizing = true;
  rsx = e.screenX;
  rsy = e.screenY;
  rsw = window.innerWidth;
  rsh = window.innerHeight;
  e.preventDefault();
  e.stopPropagation();
}
function onResizeMove(e: MouseEvent) {
  if (!resizing) return;
  const w = Math.max(260, Math.min(900, rsw + (e.screenX - rsx)));
  const h = Math.max(160, Math.min(600, rsh + (e.screenY - rsy)));
  appWindow.setSize(new LogicalSize(w, h));
}
function onResizeUp() {
  resizing = false;
}
window.addEventListener("mousemove", onResizeMove);
window.addEventListener("mouseup", onResizeUp);

function onContextMenu(e: MouseEvent) {
  e.preventDefault();
  api.popupMenu();
}

function onDoubleClick() {
  refresh();
}
</script>

<template>
  <div class="stage">
    <div class="card" @mousedown="onCardMouseDown" @dblclick="onDoubleClick" @contextmenu="onContextMenu">
      <div class="card-body">
        <header class="head">
          <span class="dot" :class="dotClass"></span>
          <span class="title">DeepSeek 余额</span>
          <div class="actions">
            <button class="btn" title="余额曲线" @click="api.openChart()" @dblclick.stop>
              <svg viewBox="0 0 24 24"><path d="M3 3v18h18" /><path d="M7 14l4-4 3 3 5-6" /></svg>
            </button>
            <button class="btn" title="设置" @click="api.openSettings()" @dblclick.stop>
              <svg viewBox="0 0 24 24"><path d="M12 15a3 3 0 100-6 3 3 0 000 6z" /><path d="M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 11-2.83 2.83l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 11-4 0v-.09a1.65 1.65 0 00-1-1.51 1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 11-2.83-2.83l.06-.06a1.65 1.65 0 00.33-1.82 1.65 1.65 0 00-1.51-1H3a2 2 0 110-4h.09a1.65 1.65 0 001.51-1 1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 112.83-2.83l.06.06a1.65 1.65 0 001.82.33h0a1.65 1.65 0 001-1.51V3a2 2 0 114 0v.09a1.65 1.65 0 001 1.51h0a1.65 1.65 0 001.82-.33l.06-.06a2 2 0 112.83 2.83l-.06.06a1.65 1.65 0 00-.33 1.82v0a1.65 1.65 0 001.51 1H21a2 2 0 110 4h-.09a1.65 1.65 0 00-1.51 1z" /></svg>
            </button>
            <button class="btn" title="立即刷新" @click="refresh" @dblclick.stop>
              <svg viewBox="0 0 24 24"><path d="M23 4v6h-6" /><path d="M20.49 15a9 9 0 11-2.12-9.36L23 10" /></svg>
            </button>
            <button class="btn btn-close" title="隐藏到托盘" @click="appWindow.hide()" @dblclick.stop>
              <svg viewBox="0 0 24 24"><path d="M18 6L6 18M6 6l12 12" /></svg>
            </button>
          </div>
        </header>

        <template v-if="hasKey">
          <div class="balance-row">
            <div class="balance">{{ balanceText }}</div>
            <div class="delta" :class="deltaClass">{{ deltaText }}</div>
          </div>
          <div class="sub">{{ subText }}</div>
          <div class="detail">{{ detailText }}</div>
        </template>
        <template v-else>
          <div class="empty">
            <p>尚未配置 API Key</p>
            <button class="go-settings" @click="api.openSettings()">前往设置</button>
          </div>
        </template>

        <footer class="foot">{{ footerText }}</footer>
      </div>

      <div class="resize-handle" @mousedown="onResizeDown" @dblclick.stop></div>
    </div>
  </div>
</template>

<style scoped>
.stage {
  position: relative;
  width: 100%;
  height: 100%;
  padding: clamp(10px, 4vw, 24px);
}

.card {
  position: relative;
  width: 100%;
  height: 100%;
  border-radius: clamp(14px, 5vw, 26px);
  background: color-mix(in srgb, var(--skin-card) calc(var(--opacity) * 100%), transparent);
  box-shadow: 0 clamp(4px, 1.6vw, 12px) clamp(12px, 4.5vw, 34px) rgba(0, 0, 0, 0.18),
    0 1px 3px rgba(0, 0, 0, 0.08);
  border: 1px solid color-mix(in srgb, var(--skin-border) calc(var(--opacity) * 100%), transparent);
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.card-body {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: clamp(10px, 3.5vw, 22px);
  min-height: 0;
}

.head {
  display: flex;
  align-items: center;
  gap: clamp(4px, 1.6vw, 10px);
  flex-shrink: 0;
}

.dot {
  width: clamp(7px, 2.4vw, 12px);
  height: clamp(7px, 2.4vw, 12px);
  border-radius: 50%;
  flex-shrink: 0;
  background: var(--dot-busy);
}
.dot.ok { background: var(--dot-ok); }
.dot.bad { background: var(--dot-bad); }
.dot.busy { background: var(--dot-busy); }

.title {
  font-size: clamp(10px, 3.4vw, 20px);
  font-weight: 600;
  color: var(--skin-text);
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.actions {
  display: flex;
  gap: clamp(2px, 0.9vw, 6px);
}

.btn {
  width: clamp(20px, 7vw, 38px);
  height: clamp(20px, 7vw, 38px);
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  border-radius: clamp(6px, 2vw, 10px);
  color: var(--skin-subtext);
  transition: background 0.15s, color 0.15s;
}
.btn:hover {
  background: color-mix(in srgb, var(--skin-subtext) 18%, transparent);
  color: var(--skin-text);
}
.btn-close:hover {
  background: color-mix(in srgb, var(--dot-bad) 22%, transparent);
  color: var(--dot-bad);
}
.btn svg {
  width: clamp(13px, 4.5vw, 24px);
  height: clamp(13px, 4.5vw, 24px);
  fill: none;
  stroke: currentColor;
  stroke-width: 2;
  stroke-linecap: round;
  stroke-linejoin: round;
}

.balance-row {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: clamp(6px, 2vw, 16px);
  flex: 1;
  min-height: 0;
}

.balance {
  font-size: clamp(18px, 9vw, 56px);
  font-weight: 700;
  color: var(--skin-accent);
  line-height: 1;
  white-space: nowrap;
}

.delta {
  font-size: clamp(11px, 3.6vw, 20px);
  font-weight: 600;
  color: var(--skin-subtext);
  white-space: nowrap;
}
.delta.up { color: var(--dot-ok); }
.delta.down { color: var(--dot-bad); }

.sub {
  font-size: clamp(10px, 3.4vw, 18px);
  color: var(--skin-subtext);
  text-align: center;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  flex-shrink: 0;
}

.detail {
  font-size: clamp(10px, 3.4vw, 18px);
  color: var(--skin-subtext);
  text-align: center;
  margin-top: clamp(2px, 0.6vw, 4px);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  flex-shrink: 0;
}

.foot {
  font-size: clamp(9px, 3vw, 15px);
  color: color-mix(in srgb, var(--skin-subtext) 75%, transparent);
  margin-top: clamp(6px, 2vw, 12px);
  text-align: center;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  flex-shrink: 0;
}

.empty {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: clamp(8px, 3vw, 16px);
  color: var(--skin-subtext);
  font-size: clamp(11px, 3.6vw, 18px);
}

.go-settings {
  padding: clamp(6px, 2vw, 12px) clamp(16px, 5vw, 28px);
  border: none;
  border-radius: clamp(7px, 2.4vw, 12px);
  background: var(--skin-accent);
  color: #fff;
  font-size: clamp(12px, 3.6vw, 18px);
}

.resize-handle {
  position: absolute;
  right: 4px;
  bottom: 4px;
  width: 14px;
  height: 14px;
  cursor: nwse-resize;
}
</style>
