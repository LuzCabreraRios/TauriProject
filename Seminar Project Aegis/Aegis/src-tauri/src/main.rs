// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::fs;
use std::process::Command;

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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_windows_account])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}