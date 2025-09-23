/*
 * NiflVeil - A window minimizer for Hyprland
 * Copyright (C) 2024 Maui The Magnificent (Charon)
 * Contact: Maui_The_Magnificent@proton.me
 * Project repository: https://github.com/Mauitron/NiflVeil.git
*/


use std::{
    collections::HashMap,
    env,
    fs::{self},
    io::{self},
    path::Path,
    process::Command,
};

const CACHE_DIR: &str = "/tmp/minimize-state";
const CACHE_FILE: &str = "/tmp/minimize-state/windows.json";
const PREVIEW_DIR: &str = "/tmp/window-previews";
const ICONS: &[(&str, &str)] = &[
    ("firefox", ""),
    ("Alacritty", ""),
    ("discord", "󰙯"),
    ("Steam", ""),
    ("chromium", ""),
    ("code", "󰨞"),
    ("spotify", ""),
    ("default", "󰖲"),
];

#[derive(Clone)]
struct MinimizedWindow {
    address: String,
    display_title: String,
    class: String,
    original_title: String,
    preview_path: Option<String>,
    icon: String,
}

fn get_focused_monitor() -> io::Result<String> {
    let output = Command::new("hyprctl")
        .args(["monitors", "-j"])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let monitors_json = String::from_utf8_lossy(&output.stdout);

            // Try to parse the JSON
            if let Ok(monitors) = serde_json::from_str::<serde_json::Value>(&monitors_json) {
                if let Some(monitor) = monitors
                    .as_array()
                    .and_then(|arr| arr.iter().find(|m| m["focused"].as_bool().unwrap_or(false)))
                {
                    if let Some(id) = monitor["id"].as_i64() {
                        return Ok(id.to_string());
                    }
                }
            }

            // Parsing failed or no focused monitor found -> fallback
            eprintln!("⚠️  Could not determine focused monitor, falling back to 0");
            Ok("0".to_string())
        }
        _ => {
            // hyprctl itself failed -> fallback
            eprintln!("⚠️  hyprctl failed, falling back to monitor 0");
            Ok("0".to_string())
        }
    }
}

fn get_app_icon(class_name: &str) -> String {
    ICONS
        .iter()
        .find(|(name, _)| class_name.to_lowercase().contains(&name.to_lowercase()))
        .map(|(_, icon)| *icon)
        .unwrap_or(ICONS.last().unwrap().1)
        .to_string()
}

fn capture_window_preview(window_id: &str, geometry: &str) -> io::Result<String> {
    let preview_path = format!("{}/{}.png", PREVIEW_DIR, window_id);
    let thumb_path = format!("{}/{}.thumb.png", PREVIEW_DIR, window_id);

    Command::new("grim")
        .args(["-g", geometry, &preview_path])
        .output()?;

    Command::new("convert")
        .args([
            &preview_path,
            "-resize",
            "200x150^",
            "-gravity",
            "center",
            "-extent",
            "200x150",
            &thumb_path,
        ])
        .output()?;

    fs::remove_file(&preview_path)?;

    Ok(thumb_path)
}

fn create_json_output(windows: &[MinimizedWindow]) -> String {
    let mut output = String::from("[");
    for (i, window) in windows.iter().enumerate() {
        if i > 0 {
            output.push(',');
        }
        output.push_str(&format!(
            "{{\"address\":\"{}\",\"display_title\":\"{}\",\"class\":\"{}\",\"original_title\":\"{}\",\"preview\":\"{}\",\"icon\":\"{}\"}}",
            window.address,
            window.display_title.replace("\"", "\\\""),
            window.class,
            window.original_title.replace("\"", "\\\""),
            window.preview_path.as_ref().unwrap_or(&String::new()),
            window.icon
        ));
    }
    output.push(']');
    output
}

fn parse_window_info(info: &str) -> io::Result<HashMap<String, String>> {
    let mut result = std::collections::HashMap::new();
    let content = info.trim_matches(|c| c == '{' || c == '}');

    for pair in content.split(',') {
        if let Some((key, value)) = pair.split_once(':') {
            let clean_key = key.trim().trim_matches('"');
            let clean_value = value.trim().trim_matches('"');
            result.insert(clean_key.to_string(), clean_value.to_string());
        }
    }

    Ok(result)
}

fn parse_windows_from_json(content: &str) -> io::Result<Vec<MinimizedWindow>> {
    let mut windows = Vec::new();
    let content = content.trim();

    if content.starts_with('[') && content.ends_with(']') {
        let content = &content[1..content.len() - 1];
        for window_json in content.split("},").map(|s| s.trim_end_matches(']')) {
            let window_json = window_json.trim_start_matches(',').trim();
            if window_json.is_empty() {
                continue;
            }

            let mut window_data = parse_window_info(window_json)?;
            windows.push(MinimizedWindow {
                address: window_data.remove("address").unwrap_or_default(),
                display_title: window_data.remove("display_title").unwrap_or_default(),
                class: window_data.remove("class").unwrap_or_default(),
                original_title: window_data.remove("original_title").unwrap_or_default(),
                icon: window_data.remove("icon").unwrap_or_default(),
                preview_path: Some(window_data.remove("preview").unwrap_or_default()),
            });
        }
    }

    Ok(windows)
}

fn restore_specific_window(window_id: &str) -> io::Result<()> {
    println!("Attempting to restore window: {}", window_id);

    let output = Command::new("hyprctl")
        .args(["activeworkspace", "-j"])
        .output()?;

    if !output.status.success() {
        println!("Failed to get active workspace");
        return Ok(());
    }

    let workspace_info =
        String::from_utf8(output.stdout).expect("cannot get workspace_info from stdout");
    println!("Workspace info: {}", workspace_info);
    let workspace_data = parse_window_info(&workspace_info)?;
    let current_ws = workspace_data
        .get("id")
        .and_then(|id| id.parse().ok())
        .unwrap_or(1);

    let move_from_special = Command::new("hyprctl")
        .args([
            "dispatch",
            "movetoworkspace",
            &format!("{},address:{}", current_ws, window_id),
        ])
        .output()?;

    println!(
        "Move from special workspace result: {:?}",
        move_from_special
    );

    let focus_cmd = Command::new("hyprctl")
        .args(["dispatch", "focuswindow", &format!("address:{}", window_id)])
        .output()?;

    println!("Focus command result: {:?}", focus_cmd);

    let content = fs::read_to_string(CACHE_FILE)?;
    let windows = parse_windows_from_json(&content)?;
    let updated_windows: Vec<MinimizedWindow> = windows
        .into_iter()
        .filter(|w| w.address != window_id)
        .collect();
    fs::write(CACHE_FILE, create_json_output(&updated_windows))?;
    signal_waybar();

    Ok(())
}

fn restore_all_windows() -> io::Result<()> {
    let content = fs::read_to_string(CACHE_FILE)?;
    let windows = parse_windows_from_json(&content)?;

    for window in windows {
        restore_specific_window(&window.address)?;
    }

    Ok(())
}

fn show_restore_menu() -> io::Result<()> {
    println!("Starting restore menu...");

    if !Path::new(CACHE_FILE).exists() {
        println!("Cache file does not exist");
        return Ok(());
    }

    let content = fs::read_to_string(CACHE_FILE)?;
    println!("Read cache file content: {}", content);

    let windows = parse_windows_from_json(&content)?;
    println!("Parsed {} windows", windows.len());

    if windows.is_empty() {
        println!("No minimized windows");
        return Ok(());
    }

    let monitor = get_focused_monitor()?;
    let eww_result = Command::new("eww")
        .args([
            "--config",
            "/etc/xdg/eww/widgets/niflveil/",
            "open",
            "--screen",
            &monitor,
            "niflveil",
        ])
        .output()?;

    if !eww_result.status.success() {
        println!(
            "Eww command failed: {}",
            String::from_utf8_lossy(&eww_result.stderr)
        );
    } else {
        println!("Eww window opened successfully");
    }

    // Command::new("eww")
    //     .args([
    //         "--config",
    //         "/etc/xdg/eww/widgets/niflveil/",
    //         "close",
    //         "niflveil",
    //     ])
    //     .output()?;

    Ok(())
}

fn restore_window(window_id: Option<&str>) -> Result<(), io::Error> {
    match window_id {
        Some(id) => restore_specific_window(&id),
        None => show_restore_menu(),
    }
}

fn minimize_window() -> Result<(), io::Error> {
    let output = Command::new("hyprctl")
        .args(["activewindow", "-j"])
        .output()?;

    if !output.status.success() {
        return Ok(());
    }

    let window_info =
        String::from_utf8(output.stdout).expect("can't get window_info from output, stdout");
    let window_data = parse_window_info(&window_info)?;

    if window_data.get("class").map_or(false, |c| c == "wofi") {
        return Ok(());
    }

    let window_addr = window_data
        .get("address")
        .ok_or("No address found")
        .expect("error");
    let short_addr: String = window_addr.chars().rev().take(4).collect();
    let class_name = window_data
        .get("class")
        .ok_or("No class found")
        .expect("error");
    let title = window_data
        .get("title")
        .ok_or("No title found")
        .expect("error");
    let icon = get_app_icon(class_name);

    let geometry = window_data.get("at").and_then(|at| {
        window_data
            .get("size")
            .map(|size| format!("{},{}", at.trim(), size.trim()))
    });

    let preview_path = if let Some(geom) = geometry {
        capture_window_preview(window_addr, &geom).ok()
    } else {
        None
    };

    let window = MinimizedWindow {
        address: window_addr.to_string(),
        display_title: format!("{} {} - {} [{}]", icon, class_name, title, short_addr),
        class: class_name.to_string(),
        original_title: title.to_string(),
        preview_path,
        icon,
    };

    let output = Command::new("hyprctl")
        .args([
            "dispatch",
            "movetoworkspacesilent",
            &format!("special:minimum,address:{}", window_addr),
        ])
        .output()?;

    if output.status.success() {
        let content = fs::read_to_string(CACHE_FILE)?;
        let mut windows = parse_windows_from_json(&content)?;
        windows.push(window);
        fs::write(CACHE_FILE, create_json_output(&windows))?;
        signal_waybar();
    }

    Ok(())
}

fn show_status() -> io::Result<()> {
    let content = fs::read_to_string(CACHE_FILE)?;
    let windows = parse_windows_from_json(&content)?;
    let count = windows.len();

    if count > 0 {
        println!(
            "{{\"text\":\"󰘸 {}\",\"class\":\"has-windows\",\"tooltip\":\"{} minimized windows\"}}",
            count, count
        );
    } else {
        println!("{{\"text\":\"󰘸\",\"class\":\"empty\",\"tooltip\":\"No minimized windows\"}}");
    }

    Ok(())
}

fn main() -> io::Result<()> {
    fs::create_dir_all(CACHE_DIR)?;
    fs::create_dir_all(PREVIEW_DIR)?;

    if !Path::new(CACHE_FILE).exists() {
        fs::write(CACHE_FILE, "[]")?;
    }

    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("");

    match command {
        "minimize" => {
            minimize_window()?;
        }
        "restore" => {
            let window_id = args.get(2).map(|s| s.to_string());
            restore_window(window_id.as_deref())?;
        }
        "restore-all" => {
            restore_all_windows()?;
        }
        "restore-last" => {
            if let Ok(content) = fs::read_to_string(CACHE_FILE) {
                if let Ok(windows) = parse_windows_from_json(&content) {
                    if let Some(window) = windows.last() {
                        restore_window(Some(&window.address))?;
                    }
                }
            }
        }
        "show" => {
            show_status()?;
        }
        _ => {
            println!("Unknown command: {}", command);
            println!("Usage: niflveil  [window_id]");
        }
    }
    Ok(())
}

fn signal_waybar() {
    Command::new("pkill")
        .args(["-RTMIN+8", "waybar"])
        .output()
        .ok();
}
