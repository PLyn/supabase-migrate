<script>
    import { goto } from "$app/navigation";

    let isLoading = false;
    let message = "";
    let messageType = "";

    async function handleSignout() {
        isLoading = true;
        message = "";

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
                message = "Successfully signed out!";
                messageType = "success";

                setTimeout(() => {
                    goto("/");
                }, 1500);
            } else if (response.status === 400) {
                message = "You were not signed in.";
                messageType = "warning";
            } else if (response.status === 500) {
                message = "Server error occurred. Please try again.";
                messageType = "error";
            } else {
                message = `Unexpected error: ${responseText}`;
                messageType = "error";
            }
        } catch (error) {
            console.error("Signout error:", error);
            message =
                "Network error. Please check your connection and try again.";
            messageType = "error";
        } finally {
            isLoading = false;
        }
    }
</script>

<div class="signout-container">
    <button on:click={handleSignout} disabled={isLoading} class="signout-btn">
        {#if isLoading}
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

<style>
    .signout-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
    }

    .signout-btn {
        padding: 0.5rem 1rem;
        background-color: #dc2626;
        color: white;
        border: none;
        border-radius: 0.375rem;
        cursor: pointer;
        font-weight: 500;
        transition: background-color 0.2s;
    }

    .signout-btn:hover:not(:disabled) {
        background-color: #b91c1c;
    }

    .signout-btn:disabled {
        background-color: #9ca3af;
        cursor: not-allowed;
    }

    .message {
        padding: 0.75rem 1rem;
        border-radius: 0.375rem;
        font-weight: 500;
        text-align: center;
        min-width: 200px;
    }

    .message.success {
        background-color: #d1fae5;
        color: #065f46;
        border: 1px solid #a7f3d0;
    }

    .message.error {
        background-color: #fee2e2;
        color: #991b1b;
        border: 1px solid #fca5a5;
    }

    .message.warning {
        background-color: #fef3c7;
        color: #92400e;
        border: 1px solid #fde68a;
    }
</style>
