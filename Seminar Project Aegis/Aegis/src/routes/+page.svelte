<script>
    import { invoke } from '@tauri-apps/api/core';

    let username = '';
    let password = '';
    let confirmPassword = '';
    let statusMessage = '';
    let isError = false;

    async function handleCreateAccount() {
        // Basic frontend validation
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
            // Call the Rust backend
            const result = await invoke('create_windows_account', { username, password });
            statusMessage = result;
            
            // TODO: We will add the "Force Logout" logic here later
            
            // Clear fields on success
            username = '';
            password = '';
            confirmPassword = '';
        } catch (error) {
            statusMessage = error;
            isError = true;
        }
    }
</script>

<main class="container">
    <h1>Project Aegis</h1>
    <p>Gaming Den Provisioning</p>

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
</main>

<style>
    /* Quick styling to keep it clean */
    :global(body) {
        background-color: #1e1e24;
        color: #f8f8f2;
        font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    }
    .container { max-width: 400px; margin: 4rem auto; padding: 2rem; background: #282a36; border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.3); }
    .form-group { margin-bottom: 1.5rem; display: flex; flex-direction: column; }
    label { margin-bottom: 0.5rem; font-weight: bold; color: #8be9fd; }
    input { padding: 0.75rem; border-radius: 4px; border: 1px solid #6272a4; background: #44475a; color: white; }
    button { padding: 1rem; background: #50fa7b; color: #282a36; font-weight: bold; font-size: 1rem; border: none; border-radius: 4px; cursor: pointer; transition: 0.2s; }
    button:hover { background: #40c963; }
    .status { margin-top: 1rem; padding: 1rem; border-radius: 4px; }
    .error { background: #ff5555; color: white; }
    .success { background: #50fa7b; color: #282a36; }
</style>