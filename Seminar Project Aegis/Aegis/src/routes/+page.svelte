<script>
    import { invoke } from '@tauri-apps/api/core';
    import { onMount } from 'svelte';

    // Account Creation Variables
    let username = '';
    let password = '';
    let confirmPassword = '';
    let statusMessage = '';
    let isError = false;
    
    // Sanitization Variables
    /**
     * @type {string | any[] | null | undefined}
     */
    let sanitizationLogs = [];
    let isSanitizing = false;

// Optimization Variables
    let optimizationStatus = '';


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

        try {
            const result = await invoke('create_windows_account', { username, password });
            statusMessage = result;
            
            // Clear fields on success
            username = '';
            password = '';
            confirmPassword = '';
        } catch (error) {
            // @ts-ignore
            statusMessage = error;
            isError = true;
        }
    }

    async function handleSanitize() {
        isSanitizing = true;
        // @ts-ignore
        sanitizationLogs = ["Initializing sanitization protocol..."];
        
        try {
            // @ts-ignore
            const results = await invoke('sanitize_credentials');
            // @ts-ignore
            sanitizationLogs = results;
        } catch (error) {
            // @ts-ignore
            sanitizationLogs = [`Critical failure: ${error}`];
        } finally {
            isSanitizing = false;
        }
    }

    async function handleMouseFix() {
        optimizationStatus = "Applying mouse fix...";
        try {
            // @ts-ignore
            optimizationStatus = await invoke('disable_mouse_acceleration');
        } catch (error) {
            optimizationStatus = `Error: ${error}`;
        }
    }

</script>

<main class="container">
    <h1>Project Aegis</h1>
    <p>Gaming Den Utility</p>

    <div class="form-group">
        <label for="username">New Username</label>
        <input id="username" type="text" bind:value={username} placeholder="e.g., PlayerOne" />
    </div>

    <div class="form-group">
        <label for="password">Password</label>
        <input id="password" type="password" bind:value={password} />
    </div>

    <div class="form-group">
        <label for="confirmPassword">Re-type Password</label>
        <input id="confirmPassword" type="password" bind:value={confirmPassword} />
    </div>

    <button on:click={handleCreateAccount}>Create Account & Logout</button>

    {#if statusMessage}
        <div class="status {isError ? 'error' : 'success'}">
            {statusMessage}
        </div>
    {/if}

    <hr class="divider" />

    <div class="section">
        <h2>System Sanitization</h2>
        <p class="description">Force closes game launchers and wipes local login tokens.</p>
        
        <button class="warning-btn" on:click={handleSanitize} disabled={isSanitizing}>
            {isSanitizing ? 'Sanitizing...' : 'Nuke Credentials'}
        </button>

        {#if sanitizationLogs.length > 0}
            <div class="log-box">
                {#each sanitizationLogs as log}
                    <div class="log-entry">{log}</div>
                {/each}
            </div>
        {/if}
    </div>

    <hr class="divider" />

    <div class="section">
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
    :global(body) {
        background-color: #1e1e24;
        color: #f8f8f2;
        font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    }
    .container { max-width: 400px; margin: 4rem auto; padding: 2rem; background: #282a36; border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.3); }
    .form-group { margin-bottom: 1.5rem; display: flex; flex-direction: column; }
    label { margin-bottom: 0.5rem; font-weight: bold; color: #8be9fd; }
    
    /* Input and Select styling combined */
    input, select { padding: 0.75rem; border-radius: 4px; border: 1px solid #6272a4; background: #44475a; color: white; width: 100%; box-sizing: border-box; }
    
    button { padding: 1rem; background: #50fa7b; color: #282a36; font-weight: bold; font-size: 1rem; border: none; border-radius: 4px; cursor: pointer; transition: 0.2s; }
    button:hover { background: #40c963; }
    .status { margin-top: 1rem; padding: 1rem; border-radius: 4px; }
    .error { background: #ff5555; color: white; }
    .success { background: #50fa7b; color: #282a36; }

    .divider { border: 0; height: 1px; background: #6272a4; margin: 2rem 0; }
    .section { margin-top: 1rem; }
    h2 { color: #ffb86c; margin-bottom: 0.5rem; font-size: 1.5rem; }
    .description { color: #8be9fd; font-size: 0.9rem; margin-bottom: 1rem; }
    
    .warning-btn { background: #ff5555; color: white; width: 100%; }
    .warning-btn:hover { background: #ff6e6e; }
    .warning-btn:disabled { background: #6272a4; cursor: not-allowed; }
    
    .log-box { margin-top: 1.5rem; padding: 1rem; background: #1e1e24; border-radius: 4px; border: 1px solid #44475a; font-family: monospace; font-size: 0.9rem; }
    .log-entry { margin-bottom: 0.5rem; }
    .log-entry:last-child { margin-bottom: 0; }

    .opt-btn { background: #8be9fd; color: #282a36; width: 100%; margin-bottom: 1rem; }
    .opt-btn:hover { background: #6fe1f9; }
    
    /* 3-Column Grid for Display Settings */
    .display-grid { display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 1rem; }
</style>