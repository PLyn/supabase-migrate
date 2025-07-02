<!-- src/routes/migrate/+page.svelte -->
<script lang="ts">
    let isLoading = $state(false);
    let previewData = $state(null);
    let error = $state<string | null>(null);

    async function fetchPreview() {
        console.log("Fetching preview...");
        isLoading = true;
        error = null;

        try {
            const response = await fetch("http://0.0.0.0:10000/preview", {
                method: "GET",
                headers: {
                    "Content-Type": "application/json",
                    // Add any auth headers if needed
                    // 'Authorization': `Bearer ${token}`
                },
                // Add CORS mode if needed
                mode: "cors",
            });

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }

            const data = await response.json();
            previewData = data;
            console.log("Preview data:", data);
        } catch (err) {
            console.error("Error fetching preview:", err);
            error =
                err instanceof Error
                    ? err.message
                    : "An unknown error occurred";
        } finally {
            isLoading = false;
        }
    }
</script>

<div class="container">
    <div class="form-group">
        <label for="preview-button" class="label">Generate Preview</label>
        <button
            id="preview-button"
            onclick={fetchPreview}
            disabled={isLoading}
            class="btn btn-primary"
            aria-describedby={error ? "error-message" : undefined}
        >
            {isLoading ? "Generating..." : "Generate"}
        </button>
    </div>

    {#if error}
        <div class="alert alert-error" role="alert" id="error-message">
            <strong>Error:</strong>
            {error}
        </div>
    {/if}

    {#if previewData}
        <div
            class="alert alert-success"
            role="region"
            aria-labelledby="preview-heading"
        >
            <h3 id="preview-heading" class="alert-title">Preview Data:</h3>
            <pre class="code-block">{JSON.stringify(previewData, null, 2)}</pre>
        </div>
    {/if}
</div>

<style>
    .container {
        max-width: 800px;
        margin: 0 auto;
        padding: 2rem 1rem;
        font-family:
            system-ui,
            -apple-system,
            BlinkMacSystemFont,
            "Segoe UI",
            Roboto,
            sans-serif;
        line-height: 1.6;
        color: #2d3748;
    }

    .form-group {
        margin-bottom: 1.5rem;
        display: flex;
        align-items: center;
        gap: 0.75rem;
        flex-wrap: wrap;
        flex-direction: column;
    }

    .label {
        font-weight: 600;
        font-size: 1rem;
        color: #4a5568;
        margin: 0;
    }

    .btn {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        padding: 0.75rem 1.5rem;
        border: 2px solid transparent;
        border-radius: 0.5rem;
        font-size: 1rem;
        font-weight: 500;
        text-decoration: none;
        cursor: pointer;
        transition: all 0.2s ease-in-out;
        min-width: 120px;
        position: relative;
    }

    .btn:focus {
        outline: none;
        box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.3);
    }

    .btn-primary {
        background-color: #3182ce;
        color: white;
        border-color: #3182ce;
    }

    .btn-primary:hover:not(:disabled) {
        background-color: #2c5282;
        border-color: #2c5282;
        transform: translateY(-1px);
        box-shadow: 0 4px 12px rgba(49, 130, 206, 0.3);
    }

    .btn-primary:active:not(:disabled) {
        transform: translateY(0);
        box-shadow: 0 2px 4px rgba(49, 130, 206, 0.3);
    }

    .btn:disabled {
        opacity: 0.6;
        cursor: not-allowed;
        transform: none;
        box-shadow: none;
    }

    .alert {
        margin-top: 1.5rem;
        padding: 1rem 1.25rem;
        border-radius: 0.5rem;
        border: 1px solid;
        font-size: 0.95rem;
    }

    .alert-error {
        background-color: #fed7d7;
        border-color: #feb2b2;
        color: #c53030;
    }

    .alert-success {
        background-color: #c6f6d5;
        border-color: #9ae6b4;
        color: #2f855a;
    }

    .alert-title {
        font-weight: 600;
        font-size: 1.1rem;
        margin: 0 0 0.75rem 0;
        color: inherit;
    }

    .code-block {
        background-color: #f7fafc;
        border: 1px solid #e2e8f0;
        border-radius: 0.375rem;
        padding: 1rem;
        font-family: "Fira Code", "Monaco", "Consolas", monospace;
        font-size: 0.875rem;
        line-height: 1.5;
        overflow-x: auto;
        margin: 0;
        color: #2d3748;
        white-space: pre-wrap;
        word-break: break-word;
    }

    /* Responsive design */
    @media (max-width: 640px) {
        .container {
            padding: 1rem 0.75rem;
        }

        .form-group {
            flex-direction: column;
            align-items: flex-start;
            gap: 0.5rem;
        }

        .btn {
            width: 100%;
            min-width: auto;
        }

        .code-block {
            font-size: 0.8rem;
            padding: 0.75rem;
        }
    }

    /* High contrast mode support */
    @media (prefers-contrast: high) {
        .btn-primary {
            border-width: 3px;
        }

        .alert {
            border-width: 2px;
        }
    }

    /* Reduced motion support */
    @media (prefers-reduced-motion: reduce) {
        .btn {
            transition: none;
        }

        .btn-primary:hover:not(:disabled) {
            transform: none;
        }

        .btn-primary:active:not(:disabled) {
            transform: none;
        }
    }

    /* Print styles */
    @media print {
        .btn {
            display: none;
        }

        .alert {
            border: 2px solid #000;
            background: white !important;
            color: #000 !important;
        }

        .code-block {
            border: 1px solid #000;
            background: white !important;
        }
    }
</style>
