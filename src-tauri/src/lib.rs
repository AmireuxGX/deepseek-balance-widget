mod api;
mod config;
mod history;

use config::{Config, WindowState};
use history::History;

use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Emitter, Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent};

struct MenuState(std::sync::Mutex<Option<Menu<tauri::Wry>>>);

fn config_path(app: &tauri::AppHandle) -> std::path::PathBuf {
    app.path()
        .app_config_dir()
        .unwrap_or_default()
        .join("config.json")
}

fn history_path(app: &tauri::AppHandle) -> std::path::PathBuf {
    app.path()
        .app_config_dir()
        .unwrap_or_default()
        .join("history.json")
}

fn show_main(app: &tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.unminimize();
        let _ = w.set_focus();
    }
}

fn persist_geometry(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let path = config_path(app);
        let mut cfg = Config::load(&path);
        let scale = window.scale_factor().unwrap_or(1.0);
        let pos = window.outer_position().ok();
        let size = window.inner_size().ok();
        cfg.window = Some(WindowState {
            x: pos.map(|p| p.x).unwrap_or(0),
            y: pos.map(|p| p.y).unwrap_or(0),
            width: size.map(|s| (s.width as f64 / scale).round() as u32).unwrap_or(300),
            height: size.map(|s| (s.height as f64 / scale).round() as u32).unwrap_or(180),
        });
        cfg.save(&path);
    }
}

fn open_chart_window(app: &tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("chart") {
        let _ = w.show();
        let _ = w.set_focus();
    }
}

fn open_settings_window(app: &tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("settings") {
        let _ = w.show();
        let _ = w.set_focus();
    }
}

#[tauri::command]
fn get_config(app: tauri::AppHandle) -> Config {
    Config::load(&config_path(&app))
}

#[tauri::command]
fn save_config(app: tauri::AppHandle, config: Config) {
    config.save(&config_path(&app));
    let _ = app.emit("config-changed", ());
}

#[tauri::command]
fn fetch_balance(app: tauri::AppHandle, api_key: String) -> api::BalanceResult {
    let result = api::fetch_balance(&api_key);
    if result.success {
        if let Some(info) = result.infos.first() {
            let path = history_path(&app);
            let mut history = History::load(&path);
            history.add_snapshot(
                info.total,
                info.topped,
                info.granted,
                info.currency.clone(),
            );
            history.save(&path);
        }
    }
    result
}

#[tauri::command]
fn get_history(app: tauri::AppHandle) -> History {
    History::load(&history_path(&app))
}

#[tauri::command]
fn clear_history(app: tauri::AppHandle) {
    let path = history_path(&app);
    let mut history = History::load(&path);
    history.clear();
    history.save(&path);
}

#[tauri::command]
fn open_chart(app: tauri::AppHandle) {
    open_chart_window(&app);
}

#[tauri::command]
fn open_settings(app: tauri::AppHandle) {
    open_settings_window(&app);
}

#[tauri::command]
fn show_main_window(app: tauri::AppHandle) {
    show_main(&app);
}

#[tauri::command]
fn quit_app(app: tauri::AppHandle) {
    app.exit(0);
}

#[tauri::command]
fn popup_menu(app: tauri::AppHandle, window: tauri::Window) {
    if let Some(menu) = app.state::<MenuState>().0.lock().ok().and_then(|m| m.clone()) {
        let _ = window.popup_menu(&menu);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            show_main(app);
        }))
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_config,
            fetch_balance,
            get_history,
            clear_history,
            open_chart,
            open_settings,
            show_main_window,
            quit_app,
            popup_menu,
        ])
        .setup(|app| {
            // 恢复窗口位置与大小
            let path = config_path(app.handle());
            let cfg = Config::load(&path);
            if let Some(ws) = cfg.window {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.set_position(tauri::PhysicalPosition::new(ws.x, ws.y));
                    let _ = window.set_size(tauri::LogicalSize::new(ws.width as f64, ws.height as f64));
                }
            }

            // 预创建曲线 / 设置窗口（隐藏），避免在 command 中动态创建导致 webview 白屏
            if let Err(e) = WebviewWindowBuilder::new(app, "chart", WebviewUrl::App("index.html#/chart".into()))
                .title("余额使用曲线")
                .inner_size(780.0, 500.0)
                .min_inner_size(540.0, 380.0)
                .resizable(true)
                .visible(false)
                .build()
            {
                eprintln!("pre-create chart window failed: {e}");
            }
            if let Err(e) = WebviewWindowBuilder::new(app, "settings", WebviewUrl::App("index.html#/settings".into()))
                .title("设置")
                .inner_size(520.0, 640.0)
                .min_inner_size(460.0, 540.0)
                .resizable(true)
                .visible(false)
                .build()
            {
                eprintln!("pre-create settings window failed: {e}");
            }

            // 托盘
            let show_i = MenuItem::with_id(app, "show", "显示悬浮窗", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;
            let _tray = TrayIconBuilder::with_id("main-tray")
                .tooltip("DeepSeek 余额悬浮窗")
                .icon(
                    tauri::image::Image::from_bytes(include_bytes!("../icons/32x32.png"))
                        .expect("failed to load tray icon"),
                )
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => show_main(app),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        show_main(tray.app_handle());
                    }
                })
                .build(app)?;

            // 原生右键菜单（悬浮窗）
            let mi_refresh = MenuItem::with_id(app, "menu-refresh", "立即刷新", true, None::<&str>)?;
            let mi_chart = MenuItem::with_id(app, "menu-chart", "查看余额曲线", true, None::<&str>)?;
            let mi_settings = MenuItem::with_id(app, "menu-settings", "设置", true, None::<&str>)?;
            let mi_sep = PredefinedMenuItem::separator(app)?;
            let mi_exit = MenuItem::with_id(app, "menu-exit", "退出", true, None::<&str>)?;
            let cm = Menu::with_items(app, &[&mi_refresh, &mi_chart, &mi_settings, &mi_sep, &mi_exit])?;
            app.manage(MenuState(std::sync::Mutex::new(Some(cm))));

            // 首次启动无 API Key 时自动打开设置
            let need_key = Config::load(&config_path(app.handle())).api_key.is_empty();
            if need_key {
                open_settings_window(app.handle());
            }

            Ok(())
        })
        .on_menu_event(|app, event| match event.id().as_ref() {
            "menu-refresh" => {
                let _ = app.emit("menu-refresh", ());
            }
            "menu-chart" => open_chart_window(app),
            "menu-settings" => open_settings_window(app),
            "menu-exit" => app.exit(0),
            _ => {}
        })
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                let label = window.label().to_string();
                if label == "main" {
                    persist_geometry(window.app_handle());
                } else if label == "settings" {
                    // 设置窗口关闭（未保存）时，通知主窗口取消预览、恢复已保存配置
                    let _ = window.app_handle().emit("preview-cancel", ());
                }
                api.prevent_close();
                let _ = window.hide();
            }
            _ => {}
        });

    builder
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            if let tauri::RunEvent::ExitRequested { .. } = event {
                persist_geometry(app_handle);
            }
        });
}
