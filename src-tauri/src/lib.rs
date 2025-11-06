mod models;
mod monitor;
mod gpu_monitor;
mod errors;
mod retry;
mod adaptive_refresh;

use models::*;
use monitor::SystemMonitor;
use std::sync::Arc;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, State,
};
use tokio::sync::RwLock;

/// 应用状态（优化内存使用，异步安全，支持增量更新）
pub struct AppState {
    pub monitor: Arc<RwLock<SystemMonitor>>,
    /// 使用 Arc 共享数据，避免不必要的克隆
    pub current_data: Arc<RwLock<Option<Arc<SystemInfo>>>>,
    /// 上一次的数据，用于计算增量更新
    pub last_data: Arc<RwLock<Option<Arc<SystemInfo>>>>,
}

// 获取系统信息（异步版本，提升性能，优化内存使用，增强错误处理）
#[tauri::command]
async fn get_system_info(state: State<'_, AppState>) -> Result<SystemInfo, String> {
    let system_info = {
        let mut monitor = state.monitor.write().await;
        monitor.refresh().await // 使用异步 refresh 方法
            .map_err(|e| e.to_string())? // 转换错误为String
    };

    // 优化内存使用：使用 Arc 共享数据，减少克隆
    let system_info_arc = Arc::new(system_info.clone());
    {
        let mut current_data = state.current_data.write().await;
        *current_data = Some(system_info_arc);
    }

    Ok(system_info)
}

// 获取GPU信息
#[tauri::command]
async fn get_gpu_info(state: State<'_, AppState>) -> Result<Option<GpuInfo>, String> {
    let monitor = state.monitor.read().await;
    Ok(monitor.get_gpu_info())
}

// 获取GPU监控状态
#[tauri::command]
async fn get_gpu_monitor_status(state: State<'_, AppState>) -> Result<(bool, Option<String>), String> {
    let monitor = state.monitor.read().await;
    Ok(monitor.get_gpu_monitor_status())
}

// 获取所有GPU名称
#[tauri::command]
async fn get_gpu_names(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let monitor = state.monitor.read().await;
    Ok(monitor.get_gpu_names())
}

// 获取详细GPU信息
#[tauri::command]
async fn get_detailed_gpu_info(
    device_index: u32,
    state: State<'_, AppState>
) -> Result<String, String> {
    let monitor = state.monitor.read().await;
    monitor.get_detailed_gpu_info(device_index)
}

// 获取当前系统数据（优化内存使用）
#[tauri::command]
async fn get_current_data(state: State<'_, AppState>) -> Result<Option<SystemInfo>, String> {
    let current_data = state.current_data.read().await;
    // 如果有数据，返回 Arc 内部数据的克隆，这比完全克隆更高效
    Ok(current_data.as_ref().map(|arc| (**arc).clone()))
}

// 获取系统信息增量更新（减少网络传输）
#[tauri::command]
async fn get_system_info_delta(state: State<'_, AppState>) -> Result<SystemInfoDelta, String> {
    let system_info = {
        let mut monitor = state.monitor.write().await;
        monitor.refresh().await
            .map_err(|e| e.to_string())?
    };

    // 优化内存使用：使用 Arc 共享数据
    let system_info_arc = Arc::new(system_info.clone());

    // 计算增量更新
    let delta = {
        let last_data = state.last_data.read().await;

        if let Some(ref last) = *last_data {
            // 有历史数据，计算增量
            SystemInfoDelta::from_diff(&**last, &system_info)
        } else {
            // 第一次获取，返回完整数据
            SystemInfoDelta::full(system_info.clone())
        }
    };

    // 更新状态
    {
        let mut current_data = state.current_data.write().await;
        let mut last_data = state.last_data.write().await;

        // 保存当前数据
        *current_data = Some(system_info_arc.clone());

        // 更新上一次的数据（用于下次计算增量）
        *last_data = Some(system_info_arc);
    }

    Ok(delta)
}

// 更新监控配置
#[tauri::command]
async fn update_monitor_config(
    config: MonitorConfig,
    state: State<'_, AppState>
) -> Result<(), String> {
    let mut monitor = state.monitor.write().await;
    monitor.update_config(config);
    Ok(())
}

// 智能刷新系统信息（包含自适应频率管理）
#[tauri::command]
async fn smart_refresh_system_info(state: State<'_, AppState>) -> Result<SystemInfo, String> {
    let mut monitor = state.monitor.write().await;
    monitor.smart_refresh().await
}

// 获取建议的刷新间隔
#[tauri::command]
async fn get_suggested_refresh_interval(state: State<'_, AppState>) -> Result<u64, String> {
    let monitor = state.monitor.read().await;
    Ok(monitor.suggested_refresh_interval().as_millis() as u64)
}

// 获取刷新统计信息
#[tauri::command]
async fn get_refresh_statistics(state: State<'_, AppState>) -> Result<adaptive_refresh::RefreshStatistics, String> {
    let monitor = state.monitor.read().await;
    Ok(monitor.get_refresh_statistics())
}

// 内部切换窗口显示/隐藏状态函数
fn toggle_window_internal(app_handle: &tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        match window.is_visible() {
            Ok(visible) => {
                if visible {
                    window.hide().map_err(|e| {
                        eprintln!("隐藏窗口失败: {}", e);
                        e.to_string()
                    })?;
                    println!("窗口已隐藏");
                } else {
                    window.unminimize().map_err(|e| {
                        eprintln!("取消最小化失败: {}", e);
                        e.to_string()
                    })?;
                    window.show().map_err(|e| {
                        eprintln!("显示窗口失败: {}", e);
                        e.to_string()
                    })?;
                    window.set_focus().map_err(|e| {
                        eprintln!("设置焦点失败: {}", e);
                        e.to_string()
                    })?;
                    println!("窗口已显示并获得焦点");
                }
            }
            Err(e) => {
                eprintln!("检查窗口可见性失败: {}", e);
                return Err(e.to_string());
            }
        }
    } else {
        eprintln!("找不到主窗口");
        return Err("找不到主窗口".to_string());
    }
    Ok(())
}

// 显示/隐藏主窗口
#[tauri::command]
async fn toggle_window(app_handle: tauri::AppHandle) -> Result<(), String> {
    toggle_window_internal(&app_handle)
}

// 退出应用
#[tauri::command]
async fn quit_app(app_handle: tauri::AppHandle) -> Result<(), String> {
    app_handle.exit(0);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // 创建系统监控器
            let monitor = SystemMonitor::new(MonitorConfig::default());

            // 创建应用状态（优化内存使用，异步安全，支持增量更新）
            let app_state = AppState {
                monitor: Arc::new(RwLock::new(monitor)),
                current_data: Arc::new(RwLock::new(None)),
                last_data: Arc::new(RwLock::new(None)),
            };

            // 创建托盘菜单
            let show_item = MenuItemBuilder::with_id("show", "显示/隐藏").build(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "退出").build(app)?;
            let menu = MenuBuilder::new(app)
                .items(&[&show_item, &quit_item])
                .build()?;

            // 创建托盘图标
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip("系统监控")
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "show" => {
                        // 执行切换窗口显示/隐藏逻辑
                        match toggle_window_internal(app) {
                            Ok(_) => println!("托盘菜单切换窗口状态成功"),
                            Err(e) => eprintln!("托盘菜单切换窗口状态失败: {}", e),
                        }
                    }
                    "quit" => {
                        println!("通过托盘菜单退出应用");
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        println!("托盘图标被点击，切换窗口状态");
                        match toggle_window_internal(app) {
                            Ok(_) => println!("托盘图标切换窗口状态成功"),
                            Err(e) => eprintln!("托盘图标切换窗口状态失败: {}", e),
                        }
                    }
                })
                .build(app)?;

            // 管理应用状态
            app.manage(app_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_system_info,
            get_gpu_info,
            get_gpu_monitor_status,
            get_gpu_names,
            get_detailed_gpu_info,
            get_current_data,
            get_system_info_delta,
            update_monitor_config,
            smart_refresh_system_info,
            get_suggested_refresh_interval,
            get_refresh_statistics,
            toggle_window,
            quit_app,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
