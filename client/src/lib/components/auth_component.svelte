<script>
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";

    onMount(async () => {
        await check_auth_status();
    });

    const auth_state = $state({
        is_authenticated: false,
        is_loading: false,
    });

    async function check_auth_status() {
        auth_state.is_loading = true;
        try {
            const response = await fetch("http://localhost:10000/auth", {
                method: "GET",
                credentials: "include",
                headers: {
                    "Content-Type": "application/json",
                },
            });
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            let response_text = await response.json();
            auth_state.is_authenticated = response_text;
        } catch (err) {
            console.error("Auth check error:", err);
            auth_state.is_authenticated = false;
        } finally {
            auth_state.is_loading = false;
        }
    }

    let message = $state("");
    let messageType = $state("");

    async function logout() {
        auth_state.is_loading = true;
        try {
            const response = await fetch("http://localhost:10000/signout", {
                method: "POST",
                credentials: "include",
                headers: {
                    "Content-Type": "application/json",
                },
            });
            const responseText = await response.text();
            if (response.ok) {
                auth_state.is_authenticated = false;
                goto("/");
            } else if (response.status === 400) {
                message = "You were not signed in.";
                messageType = "warning";
            } else if (response.status === 500) {
                message = "Server error occurred. Please try again.";
                messageType = "error";
            } else if (response.status !== 200) {
                message = `Unexpected error: ${responseText}`;
                messageType = "error";
            } else {
                message = "Successfully signed out";
                messageType = "success";
            }
        } catch (error) {
            console.error("Signout error:", error);
            return 500;
        } finally {
            auth_state.is_loading = false;
        }
    }

    function login() {
        window.location.href = "http://localhost:10000/connect-supabase/login";
    }
</script>

{#if !auth_state.is_authenticated && !auth_state.is_loading}
    <div class="signout-container">
        <button
            onclick={login}
            disabled={auth_state.is_loading}
            class="signout-btn"
        >
            {#if auth_state.is_loading}
                Signing in...
            {:else}
                Sign In with Supabase
            {/if}
        </button>
    </div>
{:else if auth_state.is_authenticated && !auth_state.is_loading}
    <div class="signout-container">
        <button
            onclick={logout}
            disabled={auth_state.is_loading}
            class="signout-btn"
        >
            {#if auth_state.is_loading}
                Signing out...
            {:else}
                Sign Out
            {/if}
        </button>
        {#if message}
            <div class="message {messageType}">
                {message}
            </div>
        {/if}
    </div>
{/if}

<style>
</style>
