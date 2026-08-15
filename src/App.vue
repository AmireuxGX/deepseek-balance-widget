<script setup lang="ts">
import { computed } from "vue";
import MainCard from "./views/MainCard.vue";
import ChartView from "./views/ChartView.vue";
import SettingsView from "./views/SettingsView.vue";

const route = computed(() => window.location.hash.replace(/^#\/?/, ""));

const view = computed(() => {
  if (route.value.startsWith("chart")) return ChartView;
  if (route.value.startsWith("settings")) return SettingsView;
  return MainCard;
});

// 根据窗口类型设置 body class（决定背景是否透明）
document.body.className = route.value.startsWith("chart")
  ? "app-chart"
  : route.value.startsWith("settings")
    ? "app-settings"
    : "app-main";
</script>

<template>
  <component :is="view" />
</template>
