<script>
// @ts-nocheck
    import { invoke } from '@tauri-apps/api/core';
    import { open } from '@tauri-apps/plugin-shell'; // Make sure to run `npm run tauri add shell` if this errors
    import { fly } from 'svelte/transition';
    import { elasticIn, elasticOut, quintOut } from 'svelte/easing';
    
    // Account Creation Variables
    let username = '';
    let password = '';
    let confirmPassword = '';
    let statusMessage = '';
    let isError = false;
    let accountCreated = false;
    
    // Sanitization Variables
    let sanitizationLogs = [];
    let isSanitizing = false;
    let showIcons = true; // State to trigger the "heaven" animation

    // Optimization Variables
    let optimizationStatus = '';
    let gameBarEnabled = false; 
    let networkOptimized = false;

    // --- State for our custom CSS animation ---
    let nukeState = 'idle'; // Can be 'idle', 'exploding', or 'returning'

    // Added Chrome and Edge to the visual list! 
    const launchers = [
        { name: 'Discord', icon: '/icons/discord.svg' },
        { name: 'Epic Games', icon: '/icons/epicgames.svg' },
        { name: 'Steam', icon: '/icons/steam.svg' },
        { name: 'Spotify', icon: '/icons/spotify.svg' },
        { name: 'Battle.net', icon: '/icons/battlenet.svg' },
        { name: 'Riot Games', icon: '/icons/riot.svg' },
        { name: 'Chrome', icon: '/icons/chrome.svg' },
        { name: 'Edge', icon: '/icons/edge.svg' }
    ];

    async function handleCreateAccount() {
        if (!username || !password) {
            statusMessage = "Username and password cannot be empty.";
            isError = true;
            return;
        }
        if (password !== confirmPassword) {
            statusMessage = "Passwords do not match!";
            isError = true;
            return;
        }

        statusMessage = "Provisioning account...";
        isError = false;
        accountCreated = false;

        try {
            const result = await invoke('create_windows_account', { username, password });
            statusMessage = result;
            accountCreated = true; 
            
            username = '';
            password = '';
            confirmPassword = '';
        } catch (error) {
            statusMessage = error;
            isError = true;
        }
    }

    async function handleLogout() {
        try {
            await invoke('logout_windows');
        } catch (error) {
            statusMessage = `Logout failed: ${error}`;
            isError = true;
        }
    }

    // --- The "Funny Nuke" Animation Logic ---
// --- PRODUCTION VERSION: Armed and ready ---
    async function handleSanitize() {
        if (isSanitizing) return; // Prevent spam-clicking the button
        
        isSanitizing = true;
        sanitizationLogs = ["Initializing sanitization protocol..."];
        
        // 1. Trigger the CSS Explosion
        nukeState = 'exploding';

        // Wait 600ms for the CSS explosion animation to finish playing visually
        await new Promise(r => setTimeout(r, 600));

        try {
            // 2. The REAL Backend Call 
            // The app will wait here while Rust forcefully closes apps and deletes folders
            const results = await invoke('sanitize_credentials');
            sanitizationLogs = results;
            
        } catch (error) {
            sanitizationLogs = [`Critical failure: ${error}`];
        } finally {
            // 3. Trigger the CSS Return from Heaven
            nukeState = 'returning';
            
            // Wait 1 second for the bouncy return animation to finish
            await new Promise(r => setTimeout(r, 1000));
            
            // 4. Reset state so the button can be used again
            nukeState = 'idle';
            isSanitizing = false;
        }
    }

//     //COMMENT FUNTION AFTER TESTING
// // --- TESTING VERSION: Does NOT delete real credentials ---
//     async function handleSanitize() {
//         if (isSanitizing) return; // Prevent spam-clicking
        
//         isSanitizing = true;
//         sanitizationLogs = ["Initializing sanitization protocol..."];
        
//         // 1. Trigger the CSS Explosion
//         nukeState = 'exploding';

//         // Wait 600ms for the CSS explosion animation to finish playing
//         await new Promise(r => setTimeout(r, 600));

//         try {
//             // Fake backend work
//             await new Promise(r => setTimeout(r, 2000));
            
//             sanitizationLogs = [
//                 "✅ Discord sanitized successfully.",
//                 "✅ Steam sanitized successfully.",
//                 "⚠️ Epic Games not found (already clean)."
//             ];
            
//         } catch (error) {
//             sanitizationLogs = [`Critical failure: ${error}`];
//         } finally {
//             // 2. Trigger the CSS Return from Heaven
//             nukeState = 'returning';
            
//             // Wait 1 second for the bouncy return animation to finish
//             await new Promise(r => setTimeout(r, 1000));
            
//             // 3. Reset to normal
//             nukeState = 'idle';
//             isSanitizing = false;
//         }
//     }
    
    async function handleMouseFix(disable) {
        optimizationStatus = disable ? "Applying raw mouse input..." : "Restoring default mouse settings...";
        try {
            optimizationStatus = await invoke('toggle_mouse_acceleration', { disable: disable });
        } catch (error) {
            optimizationStatus = `Error: ${error}`;
        }
    }

    async function handleGameBar(enable) {
        optimizationStatus = enable ? "Enabling Xbox Game Bar..." : "Disabling Xbox Game Bar...";
        try {
            optimizationStatus = await invoke('toggle_game_bar', { enable: enable });
        } catch (error) {
            optimizationStatus = `Error: ${error}`;
        }
    }

    async function handlePowerPlan(ultimate) {
        optimizationStatus = ultimate ? "Setting Ultimate Power Plan..." : "Setting Balanced Power Plan...";
        try {
            optimizationStatus = await invoke('set_power_plan', { ultimate: ultimate });
        } catch (error) {
            optimizationStatus = `Error: ${error}`;
        }
    }

    async function handleNvidia() {
        try {
            optimizationStatus = await invoke('open_nvidia_panel');
        } catch (error) {
            optimizationStatus = `Error: ${error}`;
        }
    }

    async function handleDisplaySettings() {
        try {
            optimizationStatus = await invoke('open_display_settings');
        } catch (error) {
            optimizationStatus = `Error: ${error}`;
        }
    }

    async function handleNetwork(optimize) {
        optimizationStatus = optimize ? "Optimizing system performance..." : "Restoring system defaults...";
        try {
            optimizationStatus = await invoke('toggle_network_latency', { optimize: optimize });
        } catch (error) {
            optimizationStatus = `Error: ${error}`;
        }
    }

    // Helper function to handle external links via Tauri Shell
    async function openExternalLink(url) {
        try {
            await open(url);
            optimizationStatus = `Opened external link: ${url}`;
        } catch (error) {
            optimizationStatus = `Error opening link: ${error}`;
        }
    }
</script>

<header class="app-header">
    <h1>Project Aegis</h1>
    <p>Gaming Den Utility</p>
</header>

<main class="dashboard">
    
    <div class="card">
        <h2>Account Provisioning</h2>
        <p class="description">Create a temporary 6-month Windows local user.</p>

        <div class="form-group">
            <label for="username">New Username</label>
            <input id="username" type="text" bind:value={username} placeholder="e.g., PlayerOne" disabled={accountCreated} />
        </div>

        <div class="form-group">
            <label for="password">Password</label>
            <input id="password" type="password" bind:value={password} disabled={accountCreated} />
        </div>

        <div class="form-group">
            <label for="confirmPassword">Re-type Password</label>
            <input id="confirmPassword" type="password" bind:value={confirmPassword} disabled={accountCreated} />
        </div>

        {#if accountCreated}
            <div class="success-prompt">
                <p>Account ready! Please log out to switch users.</p>
                <button class="logout-btn" on:click={handleLogout}>Click here to Log Out</button>
            </div>
        {:else}
            <button class="btn-create" on:click={handleCreateAccount}>Create Account</button>
        {/if}

        {#if statusMessage && !accountCreated}
            <div class="status {isError ? 'error' : 'success'}">
                {statusMessage}
            </div>
        {/if}
    </div>

    <div class="card">
        <h2>System Sanitization</h2>
        <p class="description">Closes the displayed apps/launchers and cleans the logged in credentials.</p>
        
        <div class="launcher-grid-wrapper">
            <div class="launcher-grid">
                {#each launchers as launcher, i}
                    <div class="launcher-item {nukeState}" 
                         style="--delay: {i * 0.05}s; --x: {i % 2 === 0 ? 1 : -1}; --y: {i % 3 === 0 ? -1 : 1};">
                        <img src={launcher.icon} alt="{launcher.name} logo" class="launcher-icon" />
                        <span>{launcher.name}</span>
                    </div>
                {/each}
            </div>
        </div>
        
        <button class="warning-btn" on:click={handleSanitize} disabled={isSanitizing} title="Forcefully terminates background processes and wipes session tokens for the listed applications.">
            {isSanitizing ? 'Sanitizing...' : 'Nuke Credentials'}
        </button>

        {#if sanitizationLogs.length > 0}
            <div class="log-box">
                {#each sanitizationLogs as log}
                    <div class="log-entry">
                        {#if log.includes('✅')} <span style="color: #50fa7b;">{log}</span>
                        {:else if log.includes('❌')} <span style="color: #ff5555;">{log}</span>
                        {:else} <span style="color: #f1fa8c;">{log}</span> {/if}
                    </div>
                {/each}
            </div>
        {/if}
    </div>
    <div class="card">
        <h2>Optimizations</h2>
        <p class="description">Configure competitive gaming standards.</p>

        <div class="button-stack">
            <div class="split-buttons">
                <button class="btn-default" on:click={() => handleMouseFix(false)} title="Restores standard Windows mouse acceleration.">Default Mouse</button>
                <button class="btn-optimized" on:click={() => handleMouseFix(true)} title="Disables 'Enhanced Pointer Precision' in the registry to ensure raw 1:1 mouse input for competitive aiming.">Disable Acceleration</button>
            </div>
            
            <div class="split-buttons">
                <button class="btn-default" on:click={() => handleNetwork(false)} title="Restores standard Windows network throttling (10 packets/ms) and allows background UWP apps to run.">Default System Performance</button>
                <button class="btn-optimized" on:click={() => handleNetwork(true)} title="Flushes DNS, disables background Windows UWP apps, and sets network throttling index to unlimited for lowest possible latency.">Optimize System Performance</button>
            </div>

            <div class="split-buttons">
                <button class="btn-default" on:click={() => handleGameBar(true)} title="Re-enables the Xbox Game Bar and background DVR recording services.">Enable Xbox Game Bar</button>
                <button class="btn-optimized" on:click={() => handleGameBar(false)} title="Forcefully disables the Xbox Game Bar in the registry to prevent overlay frame drops and background recording overhead.">Disable Xbox Game Bar</button>
            </div>

            <div class="split-buttons">
                <button class="btn-default" on:click={() => handlePowerPlan(false)} title="Applies the standard 'Balanced' Windows power plan for energy efficiency.">Balanced Power Plan</button>
                <button class="btn-optimized" on:click={() => handlePowerPlan(true)} title="Unlocks and applies the hidden 'Ultimate Performance' power plan to prevent CPU core parking and power throttling.">Ultimate Power Plan</button>
            </div>
            
            <div class="divider-small"></div>
            
            <button class="launcher-btn nvidia-btn" on:click={handleNvidia} title="Launches the NVIDIA Control Panel to configure 3D settings.">Open NVIDIA Control Panel</button>
            <button class="launcher-btn default-btn" on:click={handleDisplaySettings} title="Opens native Windows Display settings to verify resolution and refresh rate.">Open Display Settings</button>
            
            <div class="divider-small"></div>

            <h3>Support & Diagnostics Tools</h3>
            <div class="support-links">
                <button class="link-btn" on:click={() => openExternalLink('https://sourceforge.net/projects/makemeadmin/')} title="Opens the official SourceForge download page for MakeMeAdmin self-elevation tool.">
                    📥 Download MakeMeAdmin Tool
                </button>
                <button class="link-btn" on:click={() => openExternalLink('https://www.speedtest.net/')} title="Opens Speedtest by Ookla in your web browser to check network diagnostics.">
                    🌐 Open Ookla Speedtest
                </button>
            </div>
        </div>

        {#if optimizationStatus}
            <div class="status success">
                {optimizationStatus}
            </div>
        {/if}
    </div>

</main> 

<footer class="app-footer">
    <p>Credits to Luz Cabrera, Wayne State College, Class Fall 2022</p>
    <a href="https://github.com/LuzCabreraRios/TauriProject/tree/main/Seminar%20Project%20Aegis/Aegis"
     target="_blank" rel="noopener noreferrer" class="github-link">
        <img src="/icons/github.svg" alt="GitHub Repository" class="github-icon" />
    </a>
</footer>

<style>
    /* Base Theme */
    :global(body) {
        background-color: #1e1e24;
        color: #f8f8f2;
        font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
        margin: 0;
        padding: 0;
    }

    /* Header */
    .app-header {
        text-align: center;
        padding: 2rem 1rem 1rem 1rem;
    }
    .app-header h1 { margin: 0; font-size: 2.5rem; color: #f8f8f2; }
    .app-header p { margin: 0.5rem 0 0 0; color: #8be9fd; font-size: 1.1rem; }

    /* Dashboard Grid */
    .dashboard {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
        gap: 2rem;
        max-width: 1300px;
        margin: 0 auto;
        padding: 2rem;
    }

    /* The Cards */
    .card {
        background: #282a36;
        padding: 2rem;
        border-radius: 8px;
        box-shadow: 0 4px 15px rgba(0,0,0,0.3);
        display: flex;
        flex-direction: column;
    }
    
    h2 { color: #ffb86c; margin-top: 0; margin-bottom: 0.5rem; font-size: 1.5rem; }
    .description { color: #8be9fd; font-size: 0.9rem; margin-top: 0; margin-bottom: 1.5rem; }

    /* Form Elements */
    .form-group { margin-bottom: 1.5rem; display: flex; flex-direction: column; }
    label { margin-bottom: 0.5rem; font-weight: bold; color: #f8f8f2; font-size: 0.9rem; }
    input { padding: 0.75rem; border-radius: 4px; border: 1px solid #6272a4; background: #44475a; color: white; width: 100%; box-sizing: border-box; transition: 0.2s; }
    input:focus { border-color: #bd93f9; outline: none; }
    input:disabled { opacity: 0.5; cursor: not-allowed; }

    /* Primary Buttons & Glows */
    .btn-create { 
        padding: 1rem; background: #50fa7b; color: #282a36; font-weight: bold; font-size: 1rem; border: none; border-radius: 4px; cursor: pointer; transition: 0.2s; margin-top: auto; 
        box-shadow: 0 0 12px rgba(80, 250, 123, 0.3);
    }
    .btn-create:hover { background: #40c963; transform: translateY(-1px); box-shadow: 0 0 15px rgba(80, 250, 123, 0.5); }
    
    .warning-btn { 
        background: #ff5555; color: white; width: 100%; margin-top: 1rem; padding: 1rem; font-weight: bold; font-size: 1rem; border: none; border-radius: 4px; cursor: pointer; transition: 0.2s;
        box-shadow: 0 0 12px rgba(255, 85, 85, 0.3);
    }
    .warning-btn:hover { background: #ff6e6e; transform: translateY(-1px); box-shadow: 0 0 15px rgba(255, 85, 85, 0.5);}
    .warning-btn:disabled { background: #6272a4; cursor: not-allowed; box-shadow: none; transform: none; }
    
    .logout-btn { padding: 1rem; background: #bd93f9; color: #282a36; font-weight: bold; font-size: 1rem; border: none; border-radius: 4px; cursor: pointer; transition: 0.2s; width: 100%; }
    .logout-btn:hover { background: #d6b4fc; }

    /* Status Messages */
    .status { margin-top: 1rem; padding: 1rem; border-radius: 4px; font-weight: 500;}
    .error { background: rgba(255, 85, 85, 0.2); color: #ff5555; border: 1px solid #ff5555; }
    .success { background: rgba(80, 250, 123, 0.2); color: #50fa7b; border: 1px solid #50fa7b; }
    
    .success-prompt { margin-top: 1rem; padding: 1rem; background: #1e1e24; border: 1px solid #50fa7b; border-radius: 4px; text-align: center; }
    .success-prompt p { color: #50fa7b; font-weight: bold; margin-bottom: 1rem; margin-top: 0; }

    /* Launcher Visuals & Animation */
    .launcher-grid-wrapper {
        background: #1e1e24;
        padding: 1rem;
        border-radius: 8px;
        border: 1px solid #44475a;
        margin-bottom: 1.5rem;
        min-height: 140px; /* Preserve space when icons are "heavenly" */
        position: relative;
        overflow: hidden; /* Contain the explosion */
    }
    .launcher-grid {
        display: grid;
        grid-template-columns: repeat(4, 1fr);
        gap: 1rem;
    }
    .launcher-item {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        will-change: transform, opacity;
        backface-visibility: hidden;
    }
    .launcher-icon { width: 28px; height: 28px; opacity: 0.8; }
    .launcher-item span { font-size: 0.7rem; color: #f8f8f2; text-align: center; }



    /* Logs */
    .log-box { margin-top: 1.5rem; padding: 1rem; background: #1e1e24; border-radius: 4px; border: 1px solid #44475a; font-family: 'Consolas', monospace; font-size: 0.85rem; line-height: 1.4;}
    .log-entry { margin-bottom: 0.25rem; }
    .log-entry:last-child { margin-bottom: 0; }

    /* Optimization Button Styling */
    .button-stack { display: flex; flex-direction: column; gap: 0.75rem; }
    .button-stack h3 { color: #ffb86c; font-size: 1rem; margin: 1rem 0 0.25rem 0; border-bottom: 1px solid #44475a; padding-bottom: 0.25rem; }

    .launcher-btn { padding: 1rem; font-weight: bold; font-size: 1rem; border-radius: 4px; cursor: pointer; transition: 0.2s; width: 100%; margin: 0; border: 1px solid transparent; }
    .nvidia-btn { background: #1e1e24; color: #50fa7b; border-color: #50fa7b; }
    .nvidia-btn:hover { background: rgba(80, 250, 123, 0.1); }
    
    .default-btn { background: #1e1e24; color: #f8f8f2; border-color: #6272a4; }
    .default-btn:hover { background: rgba(98, 114, 164, 0.2); }
    
    .divider-small { height: 1px; background: #44475a; margin: 0.5rem 0; }

    /* SEGMENTED CONTROLS (Connected Buttons) */
    .split-buttons { 
        display: flex; 
        width: 100%; 
        margin: 0; 
    }
    
    .split-buttons button { 
        width: 50%; 
        margin: 0; 
        border: none;
        font-size: 0.8rem;
        padding: 0.85rem 0.25rem; 
        font-weight: bold;
        transition: 0.2s;
        cursor: pointer;
    }

    .split-buttons button:first-child { border-top-left-radius: 4px; border-bottom-left-radius: 4px; }
    .split-buttons button:last-child { border-top-right-radius: 4px; border-bottom-right-radius: 4px; }

    .btn-default { background: #8be9fd; color: #1e1e24; }
    .btn-default:hover { background: #6fe1f9; z-index: 1; transform: translateY(-1px); }

    .btn-optimized { background: #50fa7b; color: #1e1e24; box-shadow: 0 0 8px rgba(80, 250, 123, 0.2); }
    .btn-optimized:hover { background: #40c963; z-index: 1; transform: translateY(-1px); box-shadow: 0 0 12px rgba(80, 250, 123, 0.4); }

    /* Support Links Section */
    .support-links {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }
    .link-btn {
        background: #1e1e24;
        color: #bd93f9;
        border: 1px solid #bd93f9;
        padding: 0.85rem;
        border-radius: 4px;
        font-weight: bold;
        cursor: pointer;
        transition: 0.2s;
        text-align: left;
        font-size: 0.9rem;
    }
    .link-btn:hover {
        background: rgba(189, 147, 249, 0.1);
        transform: translateX(3px);
    }

    /* Footer Styling */
    .app-footer {
        text-align: center;
        padding: 2rem 1rem;
        margin-top: 2rem;
        color: #6272a4; 
        font-size: 0.85rem;
        font-weight: 500;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.75rem;
    }
    .app-footer p { margin: 0; }
    .github-link { display: inline-block; opacity: 0.7; transition: 0.2s; }
    .github-link:hover { opacity: 1; transform: scale(1.1); }
    .github-icon {
        width: 24px;
        height: 24px;
        filter: invert(100%) sepia(0%) saturate(0%) hue-rotate(93deg) brightness(103%) contrast(103%);
    }
    /* --- THE NUKE ANIMATIONS --- */
    
    /* Default state: visible and normal */
    .launcher-item.idle {
        opacity: 1;
        transform: scale(1) translate(0, 0);
    }

    /* Exploding state: trigger the violent outward animation */
    .launcher-item.exploding {
        /* forwards ensures it stays invisible at the end of the animation */
        animation: explodeAnim 0.6s cubic-bezier(0.1, 0.9, 0.2, 1) forwards; 
    }

    /* Returning state: trigger the bouncy drop-in from above */
    .launcher-item.returning {
        opacity: 0; /* Start invisible before the delay kicks in */
        animation: heavenReturn 0.8s cubic-bezier(0.175, 0.885, 0.32, 1.275) forwards;
        animation-delay: var(--delay); /* Stagger the drops! */
    }

    /* Keyframes for the Explosion */
    @keyframes explodeAnim {
        0% { 
            transform: scale(1) translate(0, 0) rotate(0deg); 
            opacity: 1; 
        }
        100% { 
            /* Uses the custom --x and --y variables from the HTML to scatter them */
            transform: scale(2.5) translate(calc(var(--x) * 60px), calc(var(--y) * 60px)) rotate(calc(var(--x) * 45deg)); 
            opacity: 0; 
        }
    }

    /* Keyframes for the Return */
    @keyframes heavenReturn {
        0% { 
            transform: translateY(-80px) scale(0.8); 
            opacity: 0; 
        }
        100% { 
            transform: translateY(0) scale(1); 
            opacity: 1; 
        }
    }
</style>