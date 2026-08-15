import { Effect, getCurrentWindow } from "@tauri-apps/api/window";

/**
 * Keep the frameless widget fully transparent so its rounded HTML surface does
 * not reveal a rectangular native backdrop. Regular windows may use Mica;
 * unsupported systems simply retain the CSS glass treatment.
 */
export async function applyGlassWindowEffect() {
  const appWindow = getCurrentWindow();

  if (appWindow.label === "main") {
    try {
      await appWindow.clearEffects();
    } catch {
      // The transparent CSS surface is already the intended fallback.
    }
    return;
  }

  try {
    await appWindow.setEffects({ effects: [Effect.Mica] });
  } catch {
    // Windows 10 and non-native previews use the CSS fallback.
  }
}
