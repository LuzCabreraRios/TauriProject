// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::fs;
use std::process::Command;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

#[tauri::command]
fn sanitize_credentials() -> Vec<String> {
    let mut results = Vec::new();

    let local_app_data = env::var("LOCALAPPDATA").unwrap_or_else(|_| "C:\\".to_string());
    let app_data = env::var("APPDATA").unwrap_or_else(|_| "C:\\".to_string());

    // Define targets: (App Name, Process Name, Array of Paths to Token File/Folder)
    let targets = vec![
        (
            "Discord",
            "discord.exe",
            vec![PathBuf::from(&app_data).join("discord").join("Local Storage").join("leveldb")],
        ),
        (
            "Epic Games",
            "EpicGamesLauncher.exe",
            // Epic uses webcache folders to store the actual session token
            vec![
                // The Data folder holds the actual session/auth tokens
                PathBuf::from(&local_app_data).join("EpicGamesLauncher").join("Saved").join("Data"),
                // The Config folder holds the launcher preferences (auto-login, remember me, etc.)
                PathBuf::from(&local_app_data).join("EpicGamesLauncher").join("Saved").join("Config"),
                // Keeping webcache just to clear the store UI cache for the next user
                PathBuf::from(&local_app_data).join("EpicGamesLauncher").join("Saved").join("webcache"),
                PathBuf::from(&local_app_data).join("EpicGamesLauncher").join("Saved").join("webcache_4147"),
                PathBuf::from(&local_app_data).join("EpicGamesLauncher").join("Saved").join("webcache_4430"),
            ],
        ),
        (
            "Steam",
            "steam.exe",
            vec![PathBuf::from("C:\\Program Files (x86)\\Steam\\config\\loginusers.vdf")],
        ),
        (
            "Spotify",
            "Spotify.exe",
            vec![PathBuf::from(&local_app_data).join("Spotify").join("Users")],
        ),
        (
            "Battle.net",
            "Battle.net.exe",
            vec![PathBuf::from(&app_data).join("Battle.net")],
        ),
        (
            "Riot Games",
            "RiotClientServices.exe",
            vec![PathBuf::from(&local_app_data).join("Riot Games").join("Riot Client").join("Data").join("Sessions")],
        ),
        (
            "Google Chrome",
            "chrome.exe",
            vec![PathBuf::from(&local_app_data).join("Google").join("Chrome").join("User Data")],
        ),
        (
            "Microsoft Edge",
            "msedge.exe",
            vec![PathBuf::from(&local_app_data).join("Microsoft").join("Edge").join("User Data")],
        ),
    ];

    // 1. First Pass: Kill all target processes
    for (_, process, _) in &targets {
        let _ = Command::new("taskkill")
            .args(["/F", "/T", "/IM", process])
            .output();
            
        // Explicitly kill EpicWebHelper just in case the tree kill misses the detached Chromium processes
        if *process == "EpicGamesLauncher.exe" {
             let _ = Command::new("taskkill")
                .args(["/F", "/T", "/IM", "EpicWebHelper.exe"])
                .output();
        }
    }

    // 2. Give Windows a moment to physically release the file locks!
    thread::sleep(Duration::from_millis(1500));

    // 3. Second Pass: Attempt deletion
    for (name, _, paths) in targets {
        let mut path_found = false;
        let mut has_error = false;
        let mut error_msg = String::new();

        for path in paths {
            if path.exists() {
                path_found = true;
                let delete_result = if path.is_dir() {
                    fs::remove_dir_all(&path)
                } else {
                    fs::remove_file(&path)
                };

                if let Err(e) = delete_result {
                    has_error = true;
                    error_msg = e.to_string();
                }
            }
        }

        if !path_found {
            results.push(format!("⚠️ {} not found (already clean).", name));
        } else if has_error {
            results.push(format!("❌ {} failed to clear: {}", name, error_msg));
        } else {
            results.push(format!("✅ {} sanitized successfully.", name));
        }
    }

    results
}

#[tauri::command]
fn create_windows_account(username: &str, password: &str) -> Result<String, String> {
    // 1. Construct the payload script
    let ps_script = format!(
        "$ErrorActionPreference = 'Stop'; \n\
         $Password = ConvertTo-SecureString '{}' -AsPlainText -Force; \n\
         $ExpDate = (Get-Date).AddMonths(6); \n\
         New-LocalUser -Name '{}' -Password $Password -AccountExpires $ExpDate -Description 'Aegis Gaming Den Account'; \n\
         Add-LocalGroupMember -Group 'Users' -Member '{}'",
        password, username, username
    );

    // 2. Write it to a temporary file
    let temp_dir = env::temp_dir();
    let script_path = temp_dir.join("aegis_provision.ps1");
    fs::write(&script_path, &ps_script).map_err(|e| format!("Failed to write temp script: {}", e))?;

    // 3. Execute the temp file with elevated privileges (Triggers UAC)
    // -Wait ensures Tauri pauses until the PowerShell window closes
    let runas_command = format!(
        "Start-Process powershell -ArgumentList '-NoProfile -ExecutionPolicy Bypass -WindowStyle Hidden -File \"{}\"' -Verb RunAs -Wait",
        script_path.display()
    );

    let output = Command::new("powershell")
        .args(["-Command", &runas_command])
        .output()
        .map_err(|e| e.to_string())?;

    // 4. Clean up the temp file so we don't leave credentials sitting on the drive
    let _ = fs::remove_file(script_path);

    // Note: Start-Process opens a new window, so output.status just tells us if the 
    // elevated prompt successfully launched, not if the inner script succeeded. 
    if output.status.success() {
        Ok(format!("Account '{}' provisioned.", username))
    } else {
        Err(String::from("Failed to elevate privileges or UAC was canceled."))
    }
}

#[tauri::command]
fn toggle_mouse_acceleration(disable: bool) -> Result<String, String> {
    // Standard Windows defaults: Speed = 1, Threshold1 = 6, Threshold2 = 10
    // Gaming optimization (Raw input): Speed = 0, Threshold1 = 0, Threshold2 = 0
    let speed = if disable { 0 } else { 1 };
    let threshold1 = if disable { 0 } else { 6 };
    let threshold2 = if disable { 0 } else { 10 };

    // Notice we use double braces {{ }} for the C# class definition to escape them in the format! macro
    let ps_script = format!(r#"
$code = @'
using System.Runtime.InteropServices;
public class MouseFix {{
    [DllImport("user32.dll")]
    public static extern bool SystemParametersInfo(uint uiAction, uint uiParam, uint[] pvParam, uint fWinIni);
}}
'@
Add-Type -TypeDefinition $code -ErrorAction SilentlyContinue
Set-ItemProperty -Path "HKCU:\Control Panel\Mouse" -Name "MouseSpeed" -Value "{}"
Set-ItemProperty -Path "HKCU:\Control Panel\Mouse" -Name "MouseThreshold1" -Value "{}"
Set-ItemProperty -Path "HKCU:\Control Panel\Mouse" -Name "MouseThreshold2" -Value "{}"

# We pass the thresholds and speed into the array to immediately broadcast the change
[MouseFix]::SystemParametersInfo(0x0004, 0, [System.UInt32[]]@({}, {}, {}), 3)
"#, speed, threshold1, threshold2, threshold1, threshold2, speed);

    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", &ps_script])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        let state = if disable { "disabled (Raw Input)" } else { "restored to Windows default" };
        Ok(format!("Enhanced pointer precision {}.", state))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[tauri::command]
fn logout_windows() -> Result<(), String> {
    // The "/l" flag tells Windows to perform a graceful logoff for the current user
    let output = Command::new("shutdown")
        .args(["/l"])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[tauri::command]
fn toggle_game_bar(enable: bool) -> Result<String, String> {
    let value = if enable { 1 } else { 0 };
    let ps_script = format!(r#"
        Set-ItemProperty -Path "HKCU:\System\GameConfigStore" -Name "GameDVR_Enabled" -Value {} -ErrorAction SilentlyContinue
        Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\GameDVR" -Name "AppCaptureEnabled" -Value {} -ErrorAction SilentlyContinue
    "#, value, value);

    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", &ps_script])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        let state = if enable { "enabled" } else { "disabled" };
        Ok(format!("Xbox Game Bar {}.", state))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[tauri::command]
fn set_power_plan(ultimate: bool) -> Result<String, String> {
    let ps_script = if ultimate {
        r#"
        # Check if Ultimate Performance exists, if not, duplicate it
        $plan = powercfg -l | Select-String "Ultimate Performance"
        if (-not $plan) {
            powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61 | Out-Null
        }
        $guidMatches = (powercfg -l | Select-String "Ultimate Performance") -match "[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}"
        if ($matches[0]) {
            powercfg -setactive $matches[0]
        } else {
            throw "Could not locate Ultimate Performance GUID."
        }
        "#
    } else {
        r#"
        # The GUID for the standard Windows Balanced power plan is always the same
        powercfg -setactive 381b4222-f694-41f0-9685-ff5bb260df2e
        "#
    };

    let output = std::process::Command::new("powershell")
        .args(["-NoProfile", "-Command", ps_script])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        let plan_name = if ultimate { "Ultimate Performance" } else { "Balanced" };
        Ok(format!("{} power plan applied.", plan_name))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[tauri::command]
fn open_nvidia_panel() -> Result<String, String> {
    let ps_script = r#"
        $storeApp = "shell:AppsFolder\NVIDIACorp.NVIDIAControlPanel_56jybvy8sckqj!NVIDIACorp.NVIDIAControlPanel"
        $exePath = "C:\Program Files\NVIDIA Corporation\Control Panel Client\nvcplui.exe"
        
        try {
            # Try launching the modern UWP app first
            Start-Process "explorer.exe" -ArgumentList $storeApp -ErrorAction Stop
        } catch {
            # Fallback to standard exe
            if (Test-Path $exePath) {
                Start-Process $exePath
            } else {
                throw "NVIDIA Control Panel not found on this system."
            }
        }
    "#;

    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", ps_script])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("Opening NVIDIA Control Panel...".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[tauri::command]
fn open_display_settings() -> Result<String, String> {
    // Windows URI scheme to open Display Settings directly
    Command::new("cmd")
        .args(["/C", "start", "ms-settings:display"])
        .spawn()
        .map_err(|e| e.to_string())?;
        
    Ok("Opening Windows Display Settings...".to_string())
}
#[tauri::command]
fn toggle_network_latency(optimize: bool) -> Result<String, String> {
    // We explicitly declare the type as u32 so the huge number fits
    let throttle_val: u32 = if optimize { 4294967295 } else { 10 };
    let resp_val: u32 = if optimize { 0 } else { 20 };
    
    // 1 to disable background apps (bloatware), 0 to allow them
    let bg_val: u32 = if optimize { 1 } else { 0 };
    
    // Note: Double braces {{ }} are used to escape the literal brackets in PowerShell for Rust's format! macro
    let ps_script = format!(r#"
        # 1. Network Throttling & CPU Responsiveness
        $path = "HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile"
        Set-ItemProperty -Path $path -Name "NetworkThrottlingIndex" -Value {} -ErrorAction SilentlyContinue
        Set-ItemProperty -Path $path -Name "SystemResponsiveness" -Value {} -ErrorAction SilentlyContinue
        
        # 2. Disable Background Windows Apps (Bloatware)
        $bgPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications"
        if (-not (Test-Path $bgPath)) {{ New-Item -Path $bgPath -Force | Out-Null }}
        Set-ItemProperty -Path $bgPath -Name "GlobalUserDisabled" -Value {} -ErrorAction SilentlyContinue
        
        # 3. Flush DNS
        ipconfig /flushdns | Out-Null
    "#, throttle_val, resp_val, bg_val);

    let output = std::process::Command::new("powershell")
        .args(["-NoProfile", "-Command", &ps_script])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        let state = if optimize { "optimized (DNS flushed, Bloatware disabled)" } else { "restored to Windows defaults" };
        Ok(format!("System performance {}.", state))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_windows_account, sanitize_credentials, toggle_mouse_acceleration, logout_windows, 
            toggle_game_bar, set_power_plan, open_nvidia_panel, open_display_settings, toggle_network_latency])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}