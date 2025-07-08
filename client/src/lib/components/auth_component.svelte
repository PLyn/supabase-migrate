<script>
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";

    onMount(async () => {
        await check_auth_status();
    });

    const auth_state = $state({
        is_authenticated: false,
        is_loading: true, // Start with loading true to prevent flash
    });

    async function check_auth_status() {
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

<div class="auth-container">
    {#if auth_state.is_loading}
        <button
            class="auth-button auth-button-skeleton"
            aria-label="Loading authentication status"
            disabled
        >
            <span class="skeleton-text"></span>
        </button>
    {:else if !auth_state.is_authenticated}
        <button
            onclick={login}
            disabled={auth_state.is_loading}
            class="auth-button auth-button-primary"
        >
            Sign In with Supabase
        </button>
    {:else}
        <button
            onclick={logout}
            disabled={auth_state.is_loading}
            class="auth-button auth-button-secondary"
        >
            Sign Out
        </button>
        {#if message}
            <div class="auth-message auth-message-{messageType}" role="alert">
                {message}
            </div>
        {/if}
    {/if}
</div>

<style>
    .auth-container {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        min-width: 120px;
        justify-content: flex-end;
    }

    .auth-button {
        padding: 0.5rem 1rem;
        border: 1px solid transparent;
        border-radius: 6px;
        font-size: 0.9rem;
        font-weight: 600;
        cursor: pointer;
        transition:
            background-color 0.2s ease-in-out,
            color 0.2s ease-in-out,
            border-color 0.2s ease-in-out,
            box-shadow 0.2s ease-in-out;
        white-space: nowrap;
        min-width: 170px; /* Ensure consistent button width */
    }

    .auth-button-primary {
        background-color: #3182ce;
        color: #ffffff;
        border-color: #3182ce;
    }

    .auth-button-primary:hover:not(:disabled) {
        background-color: #2b6cb0;
        border-color: #2b6cb0;
    }

    .auth-button-secondary {
        background-color: #e53e3e;
        color: #ffffff;
        border-color: #e53e3e;
    }

    .auth-button-secondary:hover:not(:disabled) {
        background-color: #c53030;
        border-color: #c53030;
    }

    .auth-button:focus {
        outline: none;
        box-shadow: 0 0 0 3px rgba(49, 130, 206, 0.4);
    }

    .auth-button:disabled {
        background-color: #cbd5e0;
        color: #718096;
        border-color: #cbd5e0;
        cursor: not-allowed;
        opacity: 0.8;
    }

    /* Skeleton loading styles */
    .auth-button-skeleton {
        background-color: #f7fafc;
        border-color: #e2e8f0;
        cursor: default;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .skeleton-text {
        display: inline-block;
        height: 1em;
        width: 163px;
        background: linear-gradient(
            90deg,
            #e2e8f0 25%,
            #f7fafc 50%,
            #e2e8f0 75%
        );
        background-size: 200% 100%;
        animation: skeleton-shimmer 1.5s infinite;
        border-radius: 4px;
    }

    @keyframes skeleton-shimmer {
        0% {
            background-position: -200% 0;
        }
        100% {
            background-position: 200% 0;
        }
    }

    .auth-message {
        position: absolute;
        right: 0;
        top: calc(100% + 0.5rem);
        padding: 0.5rem 0.75rem;
        border-radius: 4px;
        font-size: 0.8rem;
        line-height: 1.3;
        white-space: normal;
        max-width: 250px;
        z-index: 101;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    }

    .auth-message-success {
        background-color: #e6fffa;
        color: #38a169;
        border: 1px solid #9ae6b4;
    }

    .auth-message-warning {
        background-color: #fffaf0;
        color: #dd6b20;
        border: 1px solid #fbd38d;
    }

    .auth-message-error {
        background-color: #fff5f5;
        color: #e53e3e;
        border: 1px solid #feb2b2;
    }

    @media (max-width: 768px) {
        .auth-container {
            width: 100%;
            justify-content: center;
            margin-top: 0.5rem;
        }

        .auth-message {
            position: relative;
            top: auto;
            right: auto;
            margin-top: 0.5rem;
            width: 100%;
            max-width: none;
            text-align: center;
        }

        .auth-button {
            padding: 0.6rem 1.2rem;
            font-size: 0.95rem;
        }

        .auth-button-skeleton {
            padding: 0.6rem 1.2rem;
            font-size: 0.95rem;
        }

        .skeleton-text {
            /* Slightly wider for mobile */
            width: 152px;
        }
    }

    @media (max-width: 480px) {
        .auth-button {
            font-size: 0.85rem;
            padding: 0.5rem 1rem;
        }

        .auth-button-skeleton {
            font-size: 0.85rem;
            padding: 0.5rem 1rem;
        }

        .skeleton-text {
            /* Smaller width for mobile */
            width: 140px;
        }
    }
</style>
