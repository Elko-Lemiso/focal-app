// src-tauri/src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_usage;
mod platform;
mod idle;

use app_usage::AppUsage;
use chrono::Local;
use idle::get_idle_time;
use log::{ error, info };
use serde::Serialize;
use std::{ sync::{ Arc, Mutex }, thread, time::{ Duration, Instant } };
use tauri::{ State };

#[derive(Serialize, Clone)]
struct UsageRecord {
    app: String,
    window: String,
    start_time: String,
    end_time: String,
    duration_secs: u64,
}

struct AppState {
    is_tracking: Arc<Mutex<bool>>,
    usage_log: Arc<Mutex<Vec<UsageRecord>>>,
}

#[tauri::command]
fn start_tracking(state: State<AppState>) {
    let is_tracking = state.is_tracking.clone();
    let usage_log = state.usage_log.clone();

    // Check if already tracking
    {
        let tracking = is_tracking.lock().unwrap();
        if *tracking {
            return;
        }
    }

    // Set tracking flag to true
    {
        let mut tracking = is_tracking.lock().unwrap();
        *tracking = true;
    }

    thread::spawn(move || {
        info!("Starting Activity Monitor...");

        let mut current_app_info = String::new();
        let mut current_app_start_time = Local::now();

        let interval = Duration::from_secs(1);
        let mut last_checked = Instant::now();

        while *is_tracking.lock().unwrap() {
            if last_checked.elapsed() >= interval {
                match platform::get_active_window_title() {
                    Ok(new_app_info) => {
                        if new_app_info != current_app_info && !current_app_info.is_empty() {
                            let current_app_end_time = Local::now();
                            let duration = (
                                current_app_end_time - current_app_start_time
                            ).num_seconds() as u64;

                            match get_idle_time() {
                                Ok(idle_secs) => {
                                    if idle_secs < 300.0 {
                                        let usage_record = AppUsage::new(
                                            &current_app_info,
                                            current_app_start_time,
                                            current_app_end_time,
                                            duration
                                        );

                                        let record = UsageRecord {
                                            app: usage_record.app,
                                            window: usage_record.window,
                                            start_time: usage_record.start_time.to_rfc3339(),
                                            end_time: usage_record.end_time.to_rfc3339(),
                                            duration_secs: usage_record.duration_secs,
                                        };

                                        // Save to shared state
                                        let mut log = usage_log.lock().unwrap();
                                        log.push(record.clone());

                                        info!(
                                            "Switched from {} - {}. Duration: {} seconds",
                                            record.app,
                                            record.window,
                                            record.duration_secs
                                        );
                                    } else {
                                        info!("User is idle. Skipping logging.");
                                    }
                                }
                                Err(e) => {
                                    error!("Error getting idle time: {}", e);
                                }
                            }

                            current_app_start_time = Local::now();
                        }

                        current_app_info = new_app_info;
                    }
                    Err(e) => {
                        error!("Error getting active window title: {}", e);
                    }
                }
                last_checked = Instant::now();
            }

            thread::sleep(Duration::from_millis(100));
        }

        info!("Activity monitoring stopped.");
    });
}

#[tauri::command]
fn stop_tracking(state: State<AppState>) {
    let mut tracking = state.is_tracking.lock().unwrap();
    *tracking = false;
}

#[tauri::command]
fn get_usage_data(state: State<AppState>) -> Vec<UsageRecord> {
    let log = state.usage_log.lock().unwrap();
    log.clone()
}

fn main() {
    env_logger::init();

    tauri::Builder
        ::default()
        .manage(AppState {
            is_tracking: Arc::new(Mutex::new(false)),
            usage_log: Arc::new(Mutex::new(Vec::new())),
        })
        .invoke_handler(tauri::generate_handler![start_tracking, stop_tracking, get_usage_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
