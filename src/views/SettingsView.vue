<script setup lang="ts">
import { ref, onMounted } from "vue";
import { emit } from "@tauri-apps/api/event";
import * as api from "../lib/api";
import {
  PRESET_SKINS,
  getSkin,
  applySkin,
  applyOpacity,
  type SkinColors,
} from "../skins";
import { enable, disable, isEnabled } from "@tauri-apps/plugin-autostart";

const config = ref<api.Config | null>(null);
const apiKey = ref("");
const interval = ref(5);
const opacity = ref(0.92);
const skinId = ref("light");
const customSkin = ref<api.CustomSkin>({
  bg: "#eef2f8",
  card: "#ffffff",
  accent: "#4d6bfe",
  text: "#1f2937",
  subText: "#5b6b84",
  border: "#d8e4f2",
});
const autostart = ref(false);
const showKey = ref(false);
const saved = ref(false);
const clearing = ref(false);

const intervals = [1, 2, 5, 10, 30];

const customFields: { key: keyof SkinColors; label: string }[] = [
  { key: "bg", label: "背景" },
  { key: "card", label: "卡片" },
  { key: "accent", label: "主色" },
  { key: "text", label: "文字" },
  { key: "subText", label: "副文字" },
  { key: "border", label: "边框" },
];

function applyCurrentSkin() {
  const skin = getSkin(skinId.value, customSkin.value);
  applySkin(skin);
  applyOpacity(opacity.value);
}

async function emitPreview() {
  try {
    await emit("preview-changed", {
      skin: skinId.value,
      customSkin: skinId.value === "custom" ? { ...customSkin.value } : null,
      opacity: opacity.value,
    });
  } catch {
    /* ignore */
  }
}

async function init() {
  config.value = await api.getConfig();
  apiKey.value = config.value.apiKey;
  interval.value = config.value.intervalMinutes;
  opacity.value = config.value.opacity;
  skinId.value = config.value.skin;
  if (config.value.customSkin) customSkin.value = config.value.customSkin;
  try {
    autostart.value = await isEnabled();
  } catch {
    autostart.value = false;
  }
  applyCurrentSkin();
}

onMounted(init);

function selectSkin(id: string) {
  skinId.value = id;
  applyCurrentSkin();
  emitPreview();
}

function onCustomColor(key: keyof SkinColors, val: string) {
  customSkin.value[key] = val;
  applyCurrentSkin();
  emitPreview();
}

function onOpacityInput() {
  applyOpacity(opacity.value);
  emitPreview();
}

async function save() {
  if (!config.value) return;
  const cfg: api.Config = {
    ...config.value,
    apiKey: apiKey.value.trim(),
    intervalMinutes: interval.value,
    opacity: opacity.value,
    skin: skinId.value,
    customSkin: skinId.value === "custom" ? { ...customSkin.value } : null,
    window: config.value.window,
  };
  await api.saveConfig(cfg);
  try {
    if (autostart.value) await enable();
    else await disable();
  } catch {
    /* ignore */
  }
  saved.value = true;
  window.setTimeout(() => (saved.value = false), 2000);
}

async function clearHistory() {
  if (!window.confirm("确定要清空全部余额历史记录吗？")) return;
  clearing.value = true;
  try {
    await api.clearHistory();
  } finally {
    clearing.value = false;
  }
}
</script>

<template>
  <div class="settings">
    <h1>设置</h1>

    <section>
      <label class="label">DeepSeek API Key</label>
      <div class="key-row">
        <input
          class="input"
          :type="showKey ? 'text' : 'password'"
          v-model="apiKey"
          placeholder="sk-..."
          spellcheck="false"
        />
        <button class="ghost" @click="showKey = !showKey">{{ showKey ? "隐藏" : "显示" }}</button>
      </div>
      <p class="tip">密钥仅保存在本机配置中，用于查询余额，请勿外泄。</p>
    </section>

    <section>
      <label class="label">刷新间隔</label>
      <div class="chips">
        <button
          v-for="m in intervals"
          :key="m"
          class="chip"
          :class="{ active: interval === m }"
          @click="interval = m"
        >
          每 {{ m }} 分钟
        </button>
      </div>
    </section>

    <section>
      <label class="label">不透明度：{{ Math.round(opacity * 100) }}%</label>
      <input class="slider" type="range" min="0.4" max="1" step="0.01" v-model.number="opacity" @input="onOpacityInput" />
    </section>

    <section>
      <label class="label">主题皮肤</label>
      <div class="skins">
        <button
          v-for="s in PRESET_SKINS"
          :key="s.id"
          class="skin"
          :class="{ active: skinId === s.id }"
          @click="selectSkin(s.id)"
        >
          <span class="skin-swatch" :style="{ background: s.colors.accent }"></span>
          <span class="skin-name">{{ s.name }}</span>
        </button>
        <button
          class="skin"
          :class="{ active: skinId === 'custom' }"
          @click="selectSkin('custom')"
        >
          <span class="skin-swatch rainbow"></span>
          <span class="skin-name">自定义</span>
        </button>
      </div>
    </section>

    <section v-if="skinId === 'custom'">
      <label class="label">自定义颜色</label>
      <div class="colors">
        <div v-for="f in customFields" :key="f.key" class="color-row">
          <span class="color-label">{{ f.label }}</span>
          <input
            type="color"
            class="color-input"
            :value="customSkin[f.key]"
            @input="onCustomColor(f.key, ($event.target as HTMLInputElement).value)"
          />
        </div>
      </div>
    </section>

    <section>
      <label class="label">开机自启</label>
      <label class="switch">
        <input type="checkbox" v-model="autostart" />
        <span class="slider-round"></span>
        <span class="switch-text">{{ autostart ? "已开启" : "已关闭" }}</span>
      </label>
    </section>

    <section>
      <label class="label">数据</label>
      <button class="ghost danger" :disabled="clearing" @click="clearHistory">清空历史记录</button>
    </section>

    <div class="footer">
      <button class="save" @click="save">保存</button>
      <span v-if="saved" class="saved">已保存 ✓</span>
    </div>
  </div>
</template>

<style scoped>
.settings {
  max-width: 520px;
  margin: 0 auto;
  padding: 24px;
  color: var(--skin-text);
}

h1 {
  font-size: 20px;
  margin-bottom: 20px;
}

section {
  margin-bottom: 22px;
}

.label {
  display: block;
  font-size: 13px;
  font-weight: 600;
  margin-bottom: 8px;
  color: var(--skin-text);
}

.input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid var(--skin-border);
  border-radius: 8px;
  background: var(--skin-card);
  color: var(--skin-text);
  font-size: 13px;
  outline: none;
}
.input:focus {
  border-color: var(--skin-accent);
}

.key-row {
  display: flex;
  gap: 8px;
}

.tip {
  font-size: 11px;
  color: var(--skin-subtext);
  margin-top: 6px;
}

.chips {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.chip {
  padding: 6px 12px;
  font-size: 12px;
  border: 1px solid var(--skin-border);
  background: var(--skin-card);
  color: var(--skin-text);
  border-radius: 8px;
  cursor: pointer;
}
.chip.active {
  background: var(--skin-accent);
  border-color: var(--skin-accent);
  color: #fff;
}

.slider {
  width: 100%;
  accent-color: var(--skin-accent);
}

.skins {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.skin {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px 6px 8px;
  border: 1px solid var(--skin-border);
  background: var(--skin-card);
  border-radius: 8px;
  cursor: pointer;
}
.skin.active {
  border-color: var(--skin-accent);
  box-shadow: 0 0 0 1px var(--skin-accent);
}

.skin-swatch {
  width: 16px;
  height: 16px;
  border-radius: 50%;
}
.skin-swatch.rainbow {
  background: conic-gradient(#f43f5e, #f59e0b, #10b981, #0ea5e9, #8b5cf6, #f43f5e);
}

.skin-name {
  font-size: 12px;
  color: var(--skin-text);
}

.colors {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px 16px;
}

.color-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.color-label {
  font-size: 12px;
  color: var(--skin-subtext);
}

.color-input {
  width: 40px;
  height: 26px;
  padding: 0;
  border: 1px solid var(--skin-border);
  border-radius: 6px;
  background: transparent;
  cursor: pointer;
}

.switch {
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
}
.switch input {
  display: none;
}
.slider-round {
  width: 40px;
  height: 22px;
  background: var(--skin-border);
  border-radius: 11px;
  position: relative;
  transition: background 0.2s;
}
.slider-round::after {
  content: "";
  position: absolute;
  top: 2px;
  left: 2px;
  width: 18px;
  height: 18px;
  background: #fff;
  border-radius: 50%;
  transition: left 0.2s;
}
.switch input:checked + .slider-round {
  background: var(--skin-accent);
}
.switch input:checked + .slider-round::after {
  left: 20px;
}
.switch-text {
  font-size: 12px;
  color: var(--skin-subtext);
}

.ghost {
  padding: 6px 14px;
  border: 1px solid var(--skin-border);
  background: var(--skin-card);
  color: var(--skin-text);
  border-radius: 8px;
  font-size: 12px;
  cursor: pointer;
}
.ghost.danger {
  color: var(--dot-bad);
  border-color: color-mix(in srgb, var(--dot-bad) 40%, transparent);
}

.footer {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-top: 28px;
}

.save {
  padding: 9px 28px;
  border: none;
  border-radius: 8px;
  background: var(--skin-accent);
  color: #fff;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
}

.saved {
  font-size: 13px;
  color: var(--dot-ok);
}
</style>
