# 🛡️ Project Aegis

**Project Aegis** is a light-weight Windows utility application designed specifically for shared gaming environments, computer labs, and esports arenas. 

Built with **Tauri, Svelte, and Rust**, Aegis provides a centralized dashboard to provision temporary user accounts, sanitize system credentials between players, and forcefully apply competitive hardware optimizations via the Windows API.

![Project Aegis Dashboard](https://img.shields.io/badge/Version-0.1.0-50fa7b?style=for-the-badge)
![Tauri](https://img.shields.io/badge/Tauri-24C8D8?style=for-the-badge&logo=tauri&logoColor=white)
![Svelte](https://img.shields.io/badge/Svelte-FF3E00?style=for-the-badge&logo=svelte&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

---

## ✨ Core Features

### 👤 Account Provisioning
* **Instant Local Accounts:** Rapidly deploy standard 6-month temporary Windows local user accounts.
* **Graceful Handoff:** Built-in automated logoff triggers to seamlessly swap into the newly provisioned account.

### 🧹 System Sanitization
* **One-Click Credential Nuke:** Forcefully terminates background processes and wipes local `AppData` session tokens for major platforms to ensure zero account overlap between players.
* **Supported Platforms:** Discord, Epic Games, Steam, Spotify, Battle.net, Riot Games, Google Chrome, and Microsoft Edge.

### ⚡ Hardware Optimizations
* **Raw Mouse Input:** Toggles Windows "Enhanced Pointer Precision" in the registry for 1:1 competitive aiming.
* **Network & CPU Priority:** Flushes DNS, disables background UWP bloatware, and removes Windows network throttling to prioritize game packet delivery.
* **Overlay Management:** Safely disables the Xbox Game Bar and background DVR services to free up system resources.
* **Power & Display:** Injects the hidden Windows "Ultimate Performance" power plan and provides quick-access launchers for NVIDIA Control Panel and Display Settings.

---

## 🚀 Installation & Usage

For standard deployment, you do not need to build the project from source. 

1. Navigate to the `Installers` directory in this repository.
2. Download either the `.exe` (standalone) or `.msi` (installer) file.
3. **Important:** Project Aegis requires elevated privileges to modify the Windows Registry and manage local user accounts. It is recommended to deploy this alongside privilege elevation tools (like `MakeMeAdmin` or `RunAs`) depending on your lab's security policies.

---

## 🛠️ Development & Building from Source

If you want to modify the source code or compile the application yourself:

**Prerequisites:**
* [Node.js](https://nodejs.org/)
* [Rust](https://www.rust-lang.org/tools/install)
* Windows Build Tools (C++ build tools required by Rust)

**Setup:**
```bash
# Clone the repository
git clone [https://github.com/LuzCabreraRios/TauriProject.git](https://github.com/LuzCabreraRios/TauriProject.git)

# Navigate to the source code directory
cd "TauriProject/Seminar Project Aegis/Aegis"

# Install frontend dependencies
npm install

# Run the development server
npm run tauri dev

# Compile the final executable for production
npm run tauri build
```
## 🎓 Credits
Developed by **Luz Cabrera** Wayne State College, Computer Science & Computer Information Systems Built as a targeted solution for managing competitive integrity and system cleanliness in collegiate esports environments and public gaming dens.