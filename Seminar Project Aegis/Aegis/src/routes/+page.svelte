<script>
// @ts-nocheck
    import { invoke } from '@tauri-apps/api/core';
    import { open } from '@tauri-apps/plugin-shell'; 
    import { fly, fade } from 'svelte/transition';
    import { quintOut } from 'svelte/easing';
    import { tick, onMount } from 'svelte';
    
    let mounted = false;
    let activeTab = 'dashboard'; // 'dashboard' or 'optimizations'

    // Hardware States
    let mouseOptimized = false;
    let networkOptimized = false;
    let gameBarOptimized = false; 
    let powerPlanOptimized = false;

    // Fetch existing settings when the app opens!
    onMount(async () => {
        try {
            const states = await invoke('check_system_states');
            // Expected array order: [Mouse, Network, GameBar, Power]
            if (states && states.length === 4) {
                mouseOptimized = states[0];
                networkOptimized = states[1];
                gameBarOptimized = states[2];
                powerPlanOptimized = states[3];
                addLog("System hardware states verified.", "success");
            }
        } catch (e) {
            addLog(`Failed to read initial system states.`, "warning");
        }
        mounted = true;
    });

    let username = '';
    let password = '';
    let confirmPassword = '';
    let accountCreated = false;
    let isSanitizing = false;
    let nukeState = 'idle';

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

    let terminalLogs = [
        { time: new Date().toLocaleTimeString(), message: 'System initialized. Project Aegis standing by.', type: 'info' }
    ];
    let terminalContainer; 

    async function addLog(message, type = 'info') {
        const time = new Date().toLocaleTimeString();
        terminalLogs = [...terminalLogs, { time, message, type }];
        await tick();
        if (terminalContainer) terminalContainer.scrollTop = terminalContainer.scrollHeight;
    }

    async function handleCreateAccount() {
        if (!username || !password) return addLog("Username and password cannot be empty.", "error");
        if (password !== confirmPassword) return addLog("Passwords do not match!", "error");
        addLog(`Provisioning local account for ${username}...`, "warning");
        accountCreated = false;
        try {
            const result = await invoke('create_windows_account', { username, password });
            addLog(result, "success");
            accountCreated = true; 
            username = ''; password = ''; confirmPassword = '';
        } catch (error) { addLog(`Account creation failed: ${error}`, "error"); }
    }

    async function handleLogout() {
        addLog("Initiating system logoff...", "warning");
        try { await invoke('logout_windows'); } 
        catch (error) { addLog(`Logout failed: ${error}`, "error"); }
    }

    async function handleSanitize() {
        if (isSanitizing) return;
        isSanitizing = true;
        addLog("Initializing sanitization protocol. Nuking credentials...", "warning");
        nukeState = 'exploding';
        await new Promise(r => setTimeout(r, 600));

        try {
            const results = await invoke('sanitize_credentials');
            results.forEach(log => {
                let type = 'info';
                if (log.includes('✅')) type = 'success';
                if (log.includes('❌')) type = 'error';
                addLog(log, type);
            });
        } catch (error) { addLog(`Critical sanitization failure: ${error}`, "error"); } 
        finally {
            nukeState = 'returning';
            await new Promise(r => setTimeout(r, 1000));
            nukeState = 'idle';
            isSanitizing = false;
            addLog("Sanitization complete. System clean.", "success");
        }
    }

    async function handleMouseFix(optimize) {
        addLog(optimize ? "Applying raw mouse input..." : "Restoring default mouse settings...", "info");
        try {
            const res = await invoke('toggle_mouse_acceleration', { disable: optimize });
            mouseOptimized = optimize; addLog(res, "success");
        } catch (error) { addLog(`Mouse fix error: ${error}`, "error"); }
    }

    async function handleNetwork(optimize) {
        addLog(optimize ? "Optimizing system performance..." : "Restoring system defaults...", "info");
        try {
            const res = await invoke('toggle_network_latency', { optimize: optimize });
            networkOptimized = optimize; addLog(res, "success");
        } catch (error) { addLog(`Network config error: ${error}`, "error"); }
    }

    async function handleGameBar(optimize) {
        addLog(optimize ? "Disabling Xbox Game Bar..." : "Enabling Xbox Game Bar...", "info");
        try {
            const res = await invoke('toggle_game_bar', { enable: !optimize });
            gameBarOptimized = optimize; addLog(res, "success");
        } catch (error) { addLog(`Game Bar error: ${error}`, "error"); }
    }

    async function handlePowerPlan(optimize) {
        addLog(optimize ? "Setting Ultimate Power Plan..." : "Setting Balanced Power Plan...", "info");
        try {
            const res = await invoke('set_power_plan', { ultimate: optimize });
            powerPlanOptimized = optimize; addLog(res, "success");
        } catch (error) { addLog(`Power Plan error: ${error}`, "error"); }
    }

    async function handleNvidia() {
        addLog("Launching NVIDIA Control Panel...", "info");
        try { const res = await invoke('open_nvidia_panel'); addLog(res, "success"); } 
        catch (error) { addLog(`NVIDIA launch error: ${error}`, "error"); }
    }

    async function handleDisplaySettings() {
        addLog("Opening Windows Display Settings...", "info");
        try { const res = await invoke('open_display_settings'); addLog(res, "success"); } 
        catch (error) { addLog(`Display settings error: ${error}`, "error"); }
    }

    async function openExternalLink(url) {
        addLog(`Opening external tool link...`, "info");
        try { await open(url); addLog(`Successfully launched browser.`, "success"); } 
        catch (error) { addLog(`Error opening link: ${error}`, "error"); }
    }
</script>

<header class="app-header">
    <h1>Project Aegis</h1>
    <p>Gaming Den Utility</p>
    
    <div class="tab-nav">
        <button class:active={activeTab === 'dashboard'} on:click={() => activeTab = 'dashboard'}>Main Dashboard</button>
        <button class:active={activeTab === 'optimizations'} on:click={() => activeTab = 'optimizations'}>System Optimizations</button>
    </div>
</header>

<main class="dashboard">
    {#if mounted}
        {#if activeTab === 'dashboard'}
            <div class="tab-content" in:fade={{ duration: 300 }}>
                <div class="card">
                    <div>
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
                    </div>

                    {#if accountCreated}
                        <div class="success-prompt">
                            <p>Account ready! Please log out to switch users.</p>
                            <button class="logout-btn" on:click={handleLogout}>Click here to Log Out</button>
                        </div>
                    {:else}
                        <button class="btn-create" on:click={handleCreateAccount}>Create Account</button>
                    {/if}
                </div>

                <div class="card">
                    <div>
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
                    </div>
                    
                    <button class="warning-btn" on:click={handleSanitize} disabled={isSanitizing}>
                        {isSanitizing ? 'Sanitizing...' : 'Nuke Credentials'}
                    </button>
                </div>
            </div>
        {/if}

        {#if activeTab === 'optimizations'}
            <div class="tab-content single-column" in:fade={{ duration: 300 }}>
                <div class="card wide-card">
                    <div>
                        <h2>Hardware & Registry Optimizations</h2>
                        <p class="description">Configure competitive gaming standards and manage display settings.</p>

                        <div class="optimizations-grid">
                            <div class="button-stack">
                                <h3>Core Settings</h3>
                                <div class="sliding-toggle">
                                    <div class="slider-bg {mouseOptimized ? 'right' : 'left'}"></div>
                                    <button class:active={!mouseOptimized} on:click={() => handleMouseFix(false)}>Default Mouse</button>
                                    <button class:active={mouseOptimized} on:click={() => handleMouseFix(true)}>Raw Input</button>
                                </div>

                                <div class="sliding-toggle">
                                    <div class="slider-bg {networkOptimized ? 'right' : 'left'}"></div>
                                    <button class:active={!networkOptimized} on:click={() => handleNetwork(false)}>Default Network</button>
                                    <button class:active={networkOptimized} on:click={() => handleNetwork(true)}>Optimize Latency</button>
                                </div>

                                <div class="sliding-toggle">
                                    <div class="slider-bg {gameBarOptimized ? 'right' : 'left'}"></div>
                                    <button class:active={!gameBarOptimized} on:click={() => handleGameBar(false)}>Enable Game Bar</button>
                                    <button class:active={gameBarOptimized} on:click={() => handleGameBar(true)}>Disable Game Bar</button>
                                </div>

                                <div class="sliding-toggle">
                                    <div class="slider-bg {powerPlanOptimized ? 'right' : 'left'}"></div>
                                    <button class:active={!powerPlanOptimized} on:click={() => handlePowerPlan(false)}>Balanced Power</button>
                                    <button class:active={powerPlanOptimized} on:click={() => handlePowerPlan(true)}>Ultimate Power</button>
                                </div>
                            </div>

                            <div class="button-stack">
                                <h3>Quick Launchers</h3>
                                <button class="launcher-btn nvidia-btn" on:click={handleNvidia}>Open NVIDIA Control Panel</button>
                                <button class="launcher-btn default-btn" on:click={handleDisplaySettings}>Open Display Settings</button>
                                
                                

                                <h3>Support & Diagnostics</h3>
                                <div class="support-links">
                                    <button class="link-btn" on:click={() => openExternalLink('https://sourceforge.net/projects/makemeadmin/')}>
                                        📥 Download MakeMeAdmin Tool
                                    </button>
                                    <button class="link-btn" on:click={() => openExternalLink('https://www.speedtest.net/')}>
                                        🌐 Open Ookla Speedtest
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        {/if}
    {/if}
</main> 

{#if mounted}
    <section class="terminal-wrapper" in:fly={{ y: 50, duration: 800, delay: 100, easing: quintOut }}>
        <div class="terminal-header">
            <span class="dot red"></span>
            <span class="dot yellow"></span>
            <span class="dot green"></span>
            <span class="terminal-title">Aegis_Command_Output</span>
        </div>
        <div class="terminal-body" bind:this={terminalContainer}>
            {#each terminalLogs as log}
                <div class="log-line {log.type}">
                    <span class="timestamp">[{log.time}]</span> 
                    <span class="arrow">></span> 
                    {log.message}
                </div>
            {/each}
        </div>
    </section>
{/if}

<footer class="app-footer">
    <p>Credits to Luz Cabrera, Wayne State College, Class of 2026</p>
    <a href="https://github.com/LuzCabreraRios/TauriProject/tree/main/Seminar%20Project%20Aegis/Aegis"
     target="_blank" rel="noopener noreferrer" class="github-link">
        <img src="/icons/github.svg" alt="GitHub Repository" class="github-icon" />
    </a>
</footer>

<style>
    @import url('https://fonts.googleapis.com/css2?family=Fira+Code:wght@400;500;700&family=Rajdhani:wght@500;600;700&display=swap');

    :global(body) { background-color: #1e1e24; color: #f8f8f2; font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; margin: 0; padding: 0; }
    h1, h2, h3 { font-family: 'Rajdhani', sans-serif; letter-spacing: 0.5px; }

    /* Header & Tabs */
    .app-header { text-align: center; padding: 2rem 1rem 0 1rem; }
    .app-header h1 { margin: 0; font-size: 3.5rem; color: #f8f8f2; text-transform: uppercase; }
    .app-header p { margin: 0; color: #8be9fd; font-size: 1.1rem; letter-spacing: 2px; text-transform: uppercase; font-weight: bold; }
    
    .tab-nav { display: flex; justify-content: center; gap: 1rem; margin-top: 2rem; border-bottom: 2px solid #44475a; max-width: 600px; margin-left: auto; margin-right: auto; padding-bottom: 0.5rem; }
    .tab-nav button { background: none; border: none; color: #6272a4; font-family: 'Rajdhani', sans-serif; font-size: 1.2rem; font-weight: bold; text-transform: uppercase; cursor: pointer; transition: 0.2s; padding: 0.5rem 1rem; border-radius: 4px; }
    .tab-nav button:hover { color: #f8f8f2; background: rgba(68, 71, 90, 0.5); }
    .tab-nav button.active { color: #50fa7b; background: rgba(80, 250, 123, 0.1); }

    /* Dashboard Grid */
    .dashboard { max-width: 1300px; margin: 0 auto; padding: 2rem; }
    .tab-content { display: grid; grid-template-columns: repeat(auto-fit, minmax(350px, 1fr)); gap: 2rem; }
    .tab-content.single-column { grid-template-columns: 1fr; max-width: 900px; margin: 0 auto; }
    
    .optimizations-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 3rem; }

    /* Cards */
    .card { background: #282a36; padding: 2rem; border-radius: 8px; box-shadow: 0 4px 15px rgba(0,0,0,0.3); display: flex; flex-direction: column; justify-content: space-between; }
    .wide-card { padding: 3rem; }
    h2 { color: #ffb86c; margin-top: 0; margin-bottom: 0.5rem; font-size: 1.8rem; }
    .description { color: #8be9fd; font-size: 0.9rem; margin-top: 0; margin-bottom: 1.5rem; }

    /* Forms */
    .form-group { margin-bottom: 1.5rem; display: flex; flex-direction: column; }
    label { margin-bottom: 0.5rem; font-weight: bold; color: #f8f8f2; font-size: 0.9rem; }
    input { padding: 0.75rem; border-radius: 4px; border: 1px solid #6272a4; background: #44475a; color: white; width: 100%; box-sizing: border-box; transition: 0.2s; font-family: 'Fira Code', monospace; }
    input:focus { border-color: #bd93f9; outline: none; }
    input:disabled { opacity: 0.5; cursor: not-allowed; }

    /* Buttons */
    .btn-create { padding: 1rem; background: #50fa7b; color: #282a36; font-family: 'Rajdhani', sans-serif; font-weight: 700; font-size: 1.2rem; text-transform: uppercase; letter-spacing: 1px; border: none; border-radius: 4px; cursor: pointer; transition: 0.2s; box-shadow: 0 0 12px rgba(80, 250, 123, 0.3); width: 100%; }
    .btn-create:hover { background: #40c963; transform: translateY(-1px); box-shadow: 0 0 15px rgba(80, 250, 123, 0.5); }
    .warning-btn { background: #ff5555; color: white; width: 100%; padding: 1rem; font-family: 'Rajdhani', sans-serif; font-weight: 700; font-size: 1.2rem; text-transform: uppercase; letter-spacing: 1px; border: none; border-radius: 4px; cursor: pointer; transition: 0.2s; box-shadow: 0 0 12px rgba(255, 85, 85, 0.3); }
    .warning-btn:hover { background: #ff6e6e; transform: translateY(-1px); box-shadow: 0 0 15px rgba(255, 85, 85, 0.5);}
    .warning-btn:disabled { background: #6272a4; cursor: not-allowed; box-shadow: none; transform: none; }
    .logout-btn { padding: 1rem; background: #bd93f9; color: #282a36; font-weight: bold; font-size: 1rem; border: none; border-radius: 4px; cursor: pointer; transition: 0.2s; width: 100%; }
    .logout-btn:hover { background: #d6b4fc; }
    .success-prompt { padding: 1rem; background: #1e1e24; border: 1px solid #50fa7b; border-radius: 4px; text-align: center; }
    .success-prompt p { color: #50fa7b; font-weight: bold; margin-bottom: 1rem; margin-top: 0; }

    /* Launchers */
    .launcher-grid-wrapper { background: #1e1e24; padding: 1rem; border-radius: 8px; border: 1px solid #44475a; margin-bottom: 1.5rem; min-height: 140px; position: relative; overflow: hidden; }
    .launcher-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 1rem; }
    .launcher-item { display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 0.5rem; will-change: transform, opacity; backface-visibility: hidden; }
    .launcher-icon { width: 28px; height: 28px; opacity: 0.8; }
    .launcher-item span { font-size: 0.7rem; color: #f8f8f2; text-align: center; }
    .launcher-item.idle { opacity: 1; transform: scale(1) translate(0, 0); }
    .launcher-item.exploding { animation: explodeAnim 0.6s cubic-bezier(0.1, 0.9, 0.2, 1) forwards; }
    .launcher-item.returning { opacity: 0; animation: heavenReturn 0.8s cubic-bezier(0.175, 0.885, 0.32, 1.275) forwards; animation-delay: var(--delay); }
    @keyframes explodeAnim { 0% { transform: scale(1) translate(0, 0) rotate(0deg); opacity: 1; } 100% { transform: scale(2.5) translate(calc(var(--x) * 60px), calc(var(--y) * 60px)) rotate(calc(var(--x) * 45deg)); opacity: 0; } }
    @keyframes heavenReturn { 0% { transform: translateY(-80px) scale(0.8); opacity: 0; } 100% { transform: translateY(0) scale(1); opacity: 1; } }

    /* Optimizations */
    .button-stack { display: flex; flex-direction: column; gap: 1rem; }
    .button-stack h3 { color: #ffb86c; font-size: 1.3rem; margin: 0 0 0.5rem 0; border-bottom: 1px solid #44475a; padding-bottom: 0.5rem; }
    .launcher-btn { padding: 1rem; font-family: 'Rajdhani', sans-serif; font-weight: bold; font-size: 1.1rem; border-radius: 4px; cursor: pointer; transition: 0.2s; width: 100%; margin: 0; border: 1px solid transparent; text-transform: uppercase; }
    .nvidia-btn { background: #1e1e24; color: #50fa7b; border-color: #50fa7b; }
    .nvidia-btn:hover { background: rgba(80, 250, 123, 0.1); }
    .default-btn { background: #1e1e24; color: #f8f8f2; border-color: #6272a4; }
    .default-btn:hover { background: rgba(98, 114, 164, 0.2); }
    .divider-small { height: 1px; background: #44475a; margin: 1rem 0; }

    /* SLIDING PILLS */
    .sliding-toggle { position: relative; display: flex; background: #1e1e24; border-radius: 50px; padding: 4px; margin: 0; box-shadow: inset 0 2px 5px rgba(0,0,0,0.4); border: 1px solid #44475a; }
    .slider-bg { position: absolute; top: 4px; bottom: 4px; width: calc(50% - 4px); border-radius: 50px; transition: 0.4s cubic-bezier(0.25, 1, 0.5, 1); z-index: 1; }
    .slider-bg.left { transform: translateX(0); background: #8be9fd; box-shadow: 0 0 10px rgba(139, 233, 253, 0.3); }
    .slider-bg.right { transform: translateX(100%); background: #50fa7b; box-shadow: 0 0 10px rgba(80, 250, 123, 0.3); }
    .sliding-toggle button { position: relative; z-index: 2; flex: 1; background: transparent; border: none; color: #f8f8f2; font-size: 0.85rem; font-family: 'Rajdhani', sans-serif; text-transform: uppercase; font-weight: 700; padding: 0.75rem 0.25rem; cursor: pointer; transition: color 0.3s; }
    .sliding-toggle button.active { color: #1e1e24; }

    .support-links { display: flex; flex-direction: column; gap: 0.75rem; }
    .link-btn { background: #1e1e24; color: #bd93f9; border: 1px solid #bd93f9; padding: 1rem; border-radius: 4px; font-weight: bold; cursor: pointer; transition: 0.2s; text-align: left; font-size: 0.95rem; }
    .link-btn:hover { background: rgba(189, 147, 249, 0.1); transform: translateX(5px); }

    /* TERMINAL */
    .terminal-wrapper { max-width: 1300px; margin: 0 auto 2rem auto; width: calc(100% - 4rem); background: #000000; border-radius: 8px; box-shadow: 0 4px 15px rgba(0,0,0,0.5); border: 1px solid #44475a; overflow: hidden; }
    .terminal-header { background: #282a36; padding: 0.5rem 1rem; display: flex; align-items: center; border-bottom: 1px solid #44475a; }
    .terminal-header .dot { height: 12px; width: 12px; border-radius: 50%; display: inline-block; margin-right: 6px; }
    .dot.red { background-color: #ff5555; }
    .dot.yellow { background-color: #f1fa8c; }
    .dot.green { background-color: #50fa7b; }
    .terminal-title { color: #6272a4; font-family: 'Fira Code', monospace; font-size: 0.85rem; margin-left: auto; margin-right: auto; }
    .terminal-body { height: 150px; padding: 1rem; font-family: 'Fira Code', 'Courier New', monospace; font-size: 0.85rem; overflow-y: auto; color: #f8f8f2; }
    .log-line { margin-bottom: 0.4rem; line-height: 1.4; }
    .timestamp { color: #6272a4; margin-right: 0.5rem; }
    .arrow { color: #ffb86c; margin-right: 0.5rem; }
    .log-line.info { color: #8be9fd; }
    .log-line.success { color: #50fa7b; }
    .log-line.warning { color: #f1fa8c; }
    .log-line.error { color: #ff5555; }

    /* Footer */
    .app-footer { text-align: center; padding: 2rem 1rem; margin-top: 0; color: #6272a4; font-family: 'Rajdhani', sans-serif; font-size: 1rem; font-weight: 600; display: flex; flex-direction: column; align-items: center; gap: 0.75rem; letter-spacing: 0.5px; }
    .app-footer p { margin: 0; }
    .github-link { display: inline-block; opacity: 0.7; transition: 0.2s; }
    .github-link:hover { opacity: 1; transform: scale(1.1); }
    .github-icon { width: 24px; height: 24px; filter: invert(100%) sepia(0%) saturate(0%) hue-rotate(93deg) brightness(103%) contrast(103%); }
</style>