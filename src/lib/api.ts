import { invoke } from "@tauri-apps/api/core";

export interface WindowState {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface CustomSkin {
  bg: string;
  card: string;
  accent: string;
  text: string;
  subText: string;
  border: string;
}

export interface Config {
  apiKey: string;
  intervalMinutes: number;
  opacity: number;
  dark: boolean;
  skin: string;
  customSkin: CustomSkin | null;
  window: WindowState | null;
}

export interface BalanceInfo {
  currency: string;
  total: number;
  topped: number;
  granted: number;
}

export interface BalanceResult {
  success: boolean;
  isAvailable: boolean;
  infos: BalanceInfo[];
  error: string;
  raw: string;
}

export interface Snapshot {
  t: number;
  total: number;
  topped: number;
  granted: number;
  currency: string;
}

export interface Recharge {
  t: number;
  amount: number;
}

export interface History {
  snapshots: Snapshot[];
  recharges: Recharge[];
}

export const getConfig = () => invoke<Config>("get_config");
export const saveConfig = (config: Config) =>
  invoke<void>("save_config", { config });
export const fetchBalance = (apiKey: string) =>
  invoke<BalanceResult>("fetch_balance", { apiKey });
export const getHistory = () => invoke<History>("get_history");
export const clearHistory = () => invoke<void>("clear_history");
export const openChart = () => invoke<void>("open_chart");
export const openSettings = () => invoke<void>("open_settings");
export const showMainWindow = () => invoke<void>("show_main_window");
export const quitApp = () => invoke<void>("quit_app");
export const popupMenu = () => invoke<void>("popup_menu");

export function currencySymbol(currency: string): string {
  switch (currency) {
    case "CNY":
      return "¥";
    case "USD":
      return "$";
    case "EUR":
      return "€";
    default:
      return currency + " ";
  }
}

export function formatAmount(v: number): string {
  if (v === 0) return "0.00";
  return v.toLocaleString("zh-CN", {
    minimumFractionDigits: 2,
    maximumFractionDigits: 4,
  });
}
