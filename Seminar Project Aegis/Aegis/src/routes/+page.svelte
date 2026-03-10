<script>
// @ts-nocheck
    import { invoke } from '@tauri-apps/api/core';
    
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

    // Optimization Variables
    let optimizationStatus = '';

    // Array for the Launcher Logos (Using SimpleIcons for instant clean SVGs)
    const launchers = [
        { name: 'Discord', icon: '/icons/discord.svg' },
        { name: 'Epic Games', icon: '/icons/epicgames.svg' },
        { name: 'Steam', icon: '/icons/steam.svg' },
        { name: 'Spotify', icon: '/icons/spotify.svg' },
        { name: 'Battle.net', icon: '/icons/battlenet.svg' },
        { name: 'Riot Games', icon: '/icons/riot.svg' }
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

    async function handleSanitize() {
        isSanitizing = true;
        sanitizationLogs = ["Initializing sanitization protocol..."];
        
        try {
            const results = await invoke('sanitize_credentials');
            sanitizationLogs = results;
        } catch (error) {
            sanitizationLogs = [`Critical failure: ${error}`];
        } finally {
            isSanitizing = false;
        }
    }

    async function handleMouseFix() {
        optimizationStatus = "Applying mouse fix...";
        try {
            optimizationStatus = await invoke('disable_mouse_acceleration');
        } catch (error) {
            optimizationStatus = `Error: ${error}`;
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
        <p class="description">Create a temporary 6-month local user.</p>

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
            <button on:click={handleCreateAccount}>Create Account</button>
        {/if}

        {#if statusMessage && !accountCreated}
            <div class="status {isError ? 'error' : 'success'}">
                {statusMessage}
            </div>
        {/if}
    </div>

    <div class="card">
        <h2>System Sanitization</h2>
        <p class="description">Closes the displayed apps/launchers and anything that is currently logged in.</p>
        
        <div class="launcher-grid">
            {#each launchers as launcher}
                <div class="launcher-item">
                    <img src={launcher.icon} alt="{launcher.name} logo" class="launcher-icon" />
                    <span>{launcher.name}</span>
                </div>
            {/each}
        </div>
        
        <button class="warning-btn" on:click={handleSanitize} disabled={isSanitizing}>
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
        <h2>Hardware Optimizations</h2>
        <p class="description">Configure competitive gaming standards.</p>

        <button class="opt-btn" on:click={handleMouseFix}>Disable Enhanced Mouse Precision</button>

        {#if optimizationStatus}
            <div class="status success" style="margin-top: 1rem;">
                {optimizationStatus}
            </div>
        {/if}
    </div>

</main>

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

    /* Dashboard Grid (This makes it responsive!) */
    .dashboard {
        display: grid;
        /* Will create as many 350px columns as fit, otherwise stack them */
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
    input, select { padding: 0.75rem; border-radius: 4px; border: 1px solid #6272a4; background: #44475a; color: white; width: 100%; box-sizing: border-box; transition: 0.2s; }
    input:focus { border-color: #bd93f9; outline: none; }
    input:disabled { opacity: 0.5; cursor: not-allowed; }

    /* Buttons */
    button { padding: 1rem; background: #50fa7b; color: #282a36; font-weight: bold; font-size: 1rem; border: none; border-radius: 4px; cursor: pointer; transition: 0.2s; margin-top: auto; }
    button:hover { background: #40c963; }
    
    .warning-btn { background: #ff5555; color: white; width: 100%; margin-top: 1rem;}
    .warning-btn:hover { background: #ff6e6e; }
    .warning-btn:disabled { background: #6272a4; cursor: not-allowed; }
    
    .logout-btn { background: #bd93f9; color: #282a36; width: 100%; }
    .logout-btn:hover { background: #d6b4fc; }

    .opt-btn { background: #8be9fd; color: #282a36; width: 100%; margin-bottom: 1rem; }
    .opt-btn:hover { background: #6fe1f9; }

    /* Status Messages */
    .status { margin-top: 1rem; padding: 1rem; border-radius: 4px; font-weight: 500;}
    .error { background: rgba(255, 85, 85, 0.2); color: #ff5555; border: 1px solid #ff5555; }
    .success { background: rgba(80, 250, 123, 0.2); color: #50fa7b; border: 1px solid #50fa7b; }
    
    .success-prompt { margin-top: 1rem; padding: 1rem; background: #1e1e24; border: 1px solid #50fa7b; border-radius: 4px; text-align: center; }
    .success-prompt p { color: #50fa7b; font-weight: bold; margin-bottom: 1rem; margin-top: 0; }

    /* Launcher Visuals */
    .launcher-grid {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        gap: 1rem;
        margin-bottom: 1.5rem;
        background: #1e1e24;
        padding: 1rem;
        border-radius: 8px;
        border: 1px solid #44475a;
    }
    .launcher-item {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
    }
    .launcher-icon {
        width: 32px;
        height: 32px;
        opacity: 0.8;
    }
    .launcher-item span {
        font-size: 0.75rem;
        color: #f8f8f2;
        text-align: center;
    }

    /* Logs */
    .log-box { margin-top: 1.5rem; padding: 1rem; background: #1e1e24; border-radius: 4px; border: 1px solid #44475a; font-family: 'Consolas', monospace; font-size: 0.85rem; line-height: 1.4;}
    .log-entry { margin-bottom: 0.25rem; }
    .log-entry:last-child { margin-bottom: 0; }
</style>