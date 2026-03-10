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
fn disable_mouse_acceleration() -> Result<String, String> {
    // 1. Explicitly changed [uint[]] to [System.UInt32[]] to bypass PowerShell type accelerator issues
    // 2. Renamed class to "MouseFix" to prevent potential namespace conflicts
    let ps_script = r#"
$code = @'
using System.Runtime.InteropServices;
public class MouseFix {
    [DllImport("user32.dll")]
    public static extern bool SystemParametersInfo(uint uiAction, uint uiParam, uint[] pvParam, uint fWinIni);
}
'@
Add-Type -TypeDefinition $code -ErrorAction SilentlyContinue
Set-ItemProperty -Path "HKCU:\Control Panel\Mouse" -Name "MouseSpeed" -Value "0"
Set-ItemProperty -Path "HKCU:\Control Panel\Mouse" -Name "MouseThreshold1" -Value "0"
Set-ItemProperty -Path "HKCU:\Control Panel\Mouse" -Name "MouseThreshold2" -Value "0"
[MouseFix]::SystemParametersInfo(0x0004, 0, [System.UInt32[]]@(0,0,0), 3)
"#;

    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", ps_script])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("Enhanced pointer precision disabled successfully.".to_string())
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_windows_account, sanitize_credentials, disable_mouse_acceleration, logout_windows])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}