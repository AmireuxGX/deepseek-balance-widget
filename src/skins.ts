import type { CustomSkin } from "./lib/api";

export interface SkinColors {
  bg: string;
  card: string;
  accent: string;
  text: string;
  subText: string;
  border: string;
}

export interface Skin {
  id: string;
  name: string;
  dark: boolean;
  colors: SkinColors;
}

export const PRESET_SKINS: Skin[] = [
  {
    id: "light",
    name: "浅色",
    dark: false,
    colors: { bg: "#eef2f8", card: "#ffffff", accent: "#4d6bfe", text: "#1f2937", subText: "#5b6b84", border: "#d8e4f2" },
  },
  {
    id: "dark",
    name: "深色",
    dark: true,
    colors: { bg: "#14161f", card: "#1b2130", accent: "#7c8cff", text: "#e5eaf3", subText: "#9aa6bc", border: "#2e3850" },
  },
  {
    id: "ocean",
    name: "海洋",
    dark: false,
    colors: { bg: "#e7f3fb", card: "#ffffff", accent: "#0ea5e9", text: "#0f172a", subText: "#64748b", border: "#bae6fd" },
  },
  {
    id: "forest",
    name: "森林",
    dark: false,
    colors: { bg: "#e9f6ee", card: "#ffffff", accent: "#10b981", text: "#1a2e25", subText: "#5f7d6d", border: "#bbf7d0" },
  },
  {
    id: "rose",
    name: "玫瑰",
    dark: false,
    colors: { bg: "#fdeef2", card: "#ffffff", accent: "#f43f5e", text: "#2d1b22", subText: "#84616b", border: "#fecdd3" },
  },
  {
    id: "violet",
    name: "紫罗兰",
    dark: false,
    colors: { bg: "#f1edfb", card: "#ffffff", accent: "#8b5cf6", text: "#231d33", subText: "#6b6284", border: "#ddd6fe" },
  },
  {
    id: "midnight",
    name: "午夜",
    dark: true,
    colors: { bg: "#0b0d14", card: "#151a2a", accent: "#38bdf8", text: "#e2e8f0", subText: "#8493b0", border: "#28324a" },
  },
];

export function getSkin(id: string, custom: CustomSkin | null): Skin {
  if (id === "custom" && custom) {
    return { id: "custom", name: "自定义", dark: false, colors: { ...custom } };
  }
  return PRESET_SKINS.find((s) => s.id === id) ?? PRESET_SKINS[0];
}

const CSS_VARS: Record<keyof SkinColors, string> = {
  bg: "--skin-bg",
  card: "--skin-card",
  accent: "--skin-accent",
  text: "--skin-text",
  subText: "--skin-subtext",
  border: "--skin-border",
};

export function applySkin(skin: Skin) {
  const root = document.documentElement;
  (Object.keys(CSS_VARS) as (keyof SkinColors)[]).forEach((k) => {
    root.style.setProperty(CSS_VARS[k], skin.colors[k]);
  });
  root.setAttribute("data-theme", skin.dark ? "dark" : "light");
}

export function applyOpacity(opacity: number) {
  document.documentElement.style.setProperty("--opacity", String(opacity));
}
