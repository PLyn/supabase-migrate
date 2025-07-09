<script lang="ts">
    import { type ProjectList } from "$lib/bindings/ProjectList";
    import { type ProjectDiffs } from "$lib/bindings/ProjectDiffs";
    import { type ProjectDiffEntry } from "$lib/bindings/ProjectDiffEntry";

    // State using runes
    let projects = $state<ProjectList[]>([]);
    let sourceId = $state("");
    let destId = $state("");
    let authConfigEnabled = $state(false);
    let loading = $state(false);
    let results = $state<ProjectDiffs[]>([]);
    let migrationStatus = $state("");
    let activeAccordion = $state(1);

    // Derived state using $derived
    let sourceProject = $derived(projects.find((p) => p.id === sourceId));
    let destProject = $derived(projects.find((p) => p.id === destId));
    let sourceValid = $derived(
        sourceId && sourceProject?.status !== "INACTIVE",
    );
    let destValid = $derived(destId && destProject?.status !== "INACTIVE");
    let projectsValid = $derived(
        sourceValid && destValid && sourceId !== destId,
    );
    let canPreview = $derived(projectsValid && authConfigEnabled);

    // Mock functions with proper types
    async function getProjects(): Promise<ProjectList[]> {
        try {
            loading = true;
            const response = await fetch("http://localhost:10000/projects", {
                method: "GET",
                credentials: "include",
                headers: {
                    "Content-Type": "application/json",
                },
            });
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            let response_json = await response.json();
            return response_json;
        } catch (err) {
            console.error("Get projects error:", err);
            return [];
        } finally {
            loading = false;
        }
    }

    async function handlePreview() {
        try {
            results = await generatePreview(
                sourceId,
                destId,
                authConfigEnabled,
            );
            activeAccordion = 3;
        } catch (error) {
            console.error("Preview failed:", error);
        }
    }

    async function generatePreview(
        sourceId: string,
        destId: string,
        authConfigEnabled: boolean,
    ): Promise<ProjectDiffs[]> {
        try {
            loading = true;

            // Set default values for boolean parameters
            const params = new URLSearchParams({
                source_id: sourceId,
                dest_id: destId,
                auth: authConfigEnabled.toString(),
                postgrest: "false",
                edge_functions: "false",
                secrets: "false",
                postgres: "false",
            });

            const response = await fetch(
                `http://localhost:10000/preview?${params}`,
                {
                    method: "GET",
                    credentials: "include",
                    headers: {
                        "Content-Type": "application/json",
                    },
                },
            );

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }

            let response_json = await response.json();
            return response_json;
        } catch (err) {
            console.error("Get projects error:", err);
            return [];
        } finally {
            loading = false;
        }
    }

    /*
    async function migrateConfig(
        results: { name: string; diffs: Config["diffs"] }[],
        sourceId: string,
        destId: string,
    ): Promise<{ name: string; diffs: Config["diffs"] }[]> {
        loading = true;
        await new Promise((resolve) => setTimeout(resolve, 1500));
        loading = false;

        return [
            {
                name: "Auth Configuration",
                diffs: [
                    {
                        key: "jwt_secret",
                        source_value: "Successfully migrated",
                        dest_value: "new_secret_456",
                    },
                    {
                        key: "jwt_expiry",
                        source_value: "Successfully migrated",
                        dest_value: "7200",
                    },
                    {
                        key: "enable_signup",
                        source_value: "Successfully migrated",
                        dest_value: "false",
                    },
                ],
            },
        ];
    }

    async function handleMigrate() {
        try {
            const migrationResults = await migrateConfig(
                results,
                sourceId,
                destId,
            );
            results = migrationResults;
            migrationStatus = "Success";
            activeAccordion = 4;
        } catch (error) {
            migrationStatus =
                error instanceof Error ? error.message : "Unknown error";
            activeAccordion = 4;
        }
    }
    */

    function toggleAccordion(section: number) {
        activeAccordion = activeAccordion === section ? 0 : section;
    }

    // Load projects on mount using $effect
    $effect(() => {
        getProjects().then((data) => {
            projects = data;
            if (data.length > 0) {
                sourceId = data[0].id;
                destId = data[0].id;
            }
        });
    });
</script>

<div class="migration-container">
    <h1>Auth Configuration Migration</h1>

    <!-- Project Selection Accordion -->
    <div class="accordion-item">
        <button
            class="accordion-header"
            class:active={activeAccordion === 1}
            onclick={() => toggleAccordion(1)}
        >
            1. Select Projects
            <span class="accordion-icon"
                >{activeAccordion === 1 ? "−" : "+"}</span
            >
        </button>

        {#if activeAccordion === 1}
            <div class="accordion-content">
                <div class="form-group">
                    <label for="source">Source Project:</label>
                    <select id="source" bind:value={sourceId}>
                        {#each projects as project}
                            <option value={project.id}>
                                {project.id} - {project.name} - {project.region}
                                - {project.status}
                            </option>
                        {/each}
                    </select>
                    {#if sourceId && !sourceValid}
                        <p class="error">
                            Selected source project is INACTIVE.
                        </p>
                    {/if}
                </div>

                <div class="arrow-container">
                    <div class="animated-arrow">↓</div>
                </div>

                <div class="form-group">
                    <label for="dest">Destination Project:</label>
                    <select id="dest" bind:value={destId}>
                        {#each projects as project}
                            <option value={project.id}>
                                {project.id} - {project.name} - {project.region}
                                - {project.status}
                            </option>
                        {/each}
                    </select>
                    {#if destId && !destValid}
                        <p class="error">
                            Selected destination project is INACTIVE.
                        </p>
                    {/if}
                </div>

                {#if sourceId === destId}
                    <p class="error">
                        Source and destination projects cannot be the same.
                    </p>
                {/if}

                {#if projectsValid}
                    <p class="success">Projects validated successfully!</p>
                    <button
                        class="btn-primary"
                        onclick={() => toggleAccordion(2)}
                    >
                        Next: Configure Items
                    </button>
                {/if}
            </div>
        {/if}
    </div>

    <!-- Configuration Selection Accordion -->
    <div class="accordion-item">
        <button
            class="accordion-header"
            class:active={activeAccordion === 2}
            class:disabled={!projectsValid}
            onclick={() => projectsValid && toggleAccordion(2)}
        >
            2. Select Configuration Items
            <span class="accordion-icon"
                >{activeAccordion === 2 ? "−" : "+"}</span
            >
        </button>

        {#if activeAccordion === 2}
            <div class="accordion-content">
                <div class="config-options">
                    <label class="checkbox-label">
                        <input
                            type="checkbox"
                            bind:checked={authConfigEnabled}
                        />
                        <span>Auth Configuration</span>
                    </label>
                </div>

                {#if authConfigEnabled}
                    <button class="btn-primary" onclick={handlePreview}>
                        Preview Changes
                    </button>
                {/if}
            </div>
        {/if}
    </div>

    <!-- Preview Results Accordion -->
    <div class="accordion-item">
        <button
            class="accordion-header"
            class:active={activeAccordion === 3}
            class:disabled={!canPreview}
            onclick={() => canPreview && toggleAccordion(3)}
        >
            3. Preview Changes
            <span class="accordion-icon"
                >{activeAccordion === 3 ? "−" : "+"}</span
            >
        </button>

        {#if activeAccordion === 3}
            <div class="accordion-content">
                {#if loading}
                    <div class="loading">
                        <div class="spinner"></div>
                        <p>Loading preview...</p>
                    </div>
                {:else if results.length > 0}
                    <div class="results-container">
                        {#each results as config}
                            <div class="config-section">
                                <h3>{config.name}</h3>
                                <table class="results-table">
                                    <thead>
                                        <tr>
                                            <th>Config Item</th>
                                            <th>Source</th>
                                            <th>Destination</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {#each config.diffs as diff}
                                            <tr>
                                                <td>{diff.key}</td>
                                                <td>{diff.source_value}</td>
                                                <td>{diff.dest_value}</td>
                                            </tr>
                                        {/each}
                                    </tbody>
                                </table>
                            </div>
                        {/each}

                        <button class="btn-primary migrate-btn">
                            <!-- onclick={handleMigrate} -->
                            Migrate Configuration!
                        </button>
                    </div>
                {/if}
            </div>
        {/if}
    </div>

    <!-- Migration Results Accordion -->
    <div class="accordion-item">
        <button
            class="accordion-header"
            class:active={activeAccordion === 4}
            class:disabled={!migrationStatus}
            onclick={() => migrationStatus && toggleAccordion(4)}
        >
            4. Migration Results
            <span class="accordion-icon"
                >{activeAccordion === 4 ? "−" : "+"}</span
            >
        </button>

        {#if activeAccordion === 4}
            <div class="accordion-content">
                {#if loading}
                    <div class="loading">
                        <div class="spinner"></div>
                        <p>Migrating configuration...</p>
                    </div>
                {:else}
                    <div class="migration-status">
                        <h3>Migration Status: {migrationStatus}</h3>
                    </div>

                    {#if results.length > 0}
                        <div class="results-container">
                            {#each results as config}
                                <div class="config-section">
                                    <h3>{config.name}</h3>
                                    <table class="results-table">
                                        <thead>
                                            <tr>
                                                <th>Config Item</th>
                                                <th>Migration Status</th>
                                                <th>Current Value</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {#each config.diffs as diff}
                                                <tr>
                                                    <td>{diff.key}</td>
                                                    <td>{diff.source_value}</td>
                                                    <td>{diff.dest_value}</td>
                                                </tr>
                                            {/each}
                                        </tbody>
                                    </table>
                                </div>
                            {/each}
                        </div>
                    {/if}
                {/if}
            </div>
        {/if}
    </div>
</div>

<style>
    .migration-container {
        max-width: 800px;
        margin: 0 auto;
        padding: 20px;
        font-family: Arial, sans-serif;
    }

    h1 {
        text-align: center;
        color: #333;
        margin-bottom: 30px;
    }

    .accordion-item {
        margin-bottom: 10px;
        border: 1px solid #ddd;
        border-radius: 8px;
        overflow: hidden;
    }

    .accordion-header {
        width: 100%;
        padding: 15px 20px;
        background: #f8f9fa;
        border: none;
        text-align: left;
        cursor: pointer;
        font-size: 16px;
        font-weight: 600;
        display: flex;
        justify-content: space-between;
        align-items: center;
        transition: background-color 0.3s;
    }

    .accordion-header:hover:not(.disabled) {
        background: #e9ecef;
    }

    .accordion-header.active {
        background: #007bff;
        color: white;
    }

    .accordion-header.disabled {
        background: #f8f9fa;
        color: #6c757d;
        cursor: not-allowed;
    }

    .accordion-icon {
        font-size: 18px;
        font-weight: bold;
    }

    .accordion-content {
        padding: 20px;
        background: white;
        border-top: 1px solid #ddd;
    }

    .form-group {
        margin-bottom: 20px;
    }

    label {
        display: block;
        margin-bottom: 5px;
        font-weight: 600;
        color: #333;
    }

    select {
        width: 100%;
        padding: 10px;
        border: 2px solid #ddd;
        border-radius: 4px;
        font-size: 14px;
    }

    select:focus {
        outline: none;
        border-color: #007bff;
    }

    .arrow-container {
        text-align: center;
        margin: 20px 0;
    }

    .animated-arrow {
        font-size: 24px;
        color: #007bff;
        animation: bounce 2s infinite;
    }

    @keyframes bounce {
        0%,
        20%,
        50%,
        80%,
        100% {
            transform: translateY(0);
        }
        40% {
            transform: translateY(-10px);
        }
        60% {
            transform: translateY(-5px);
        }
    }

    .config-options {
        margin-bottom: 20px;
    }

    .checkbox-label {
        display: flex;
        align-items: center;
        padding: 10px;
        border: 1px solid #ddd;
        border-radius: 4px;
        cursor: pointer;
        transition: background-color 0.3s;
    }

    .checkbox-label:hover {
        background: #f8f9fa;
    }

    .checkbox-label input[type="checkbox"] {
        margin-right: 10px;
        width: 16px;
        height: 16px;
    }

    .btn-primary {
        background: #007bff;
        color: white;
        border: none;
        padding: 12px 24px;
        border-radius: 4px;
        cursor: pointer;
        font-size: 16px;
        font-weight: 600;
        transition: background-color 0.3s;
    }

    .btn-primary:hover {
        background: #0056b3;
    }

    .migrate-btn {
        margin-top: 20px;
        width: 100%;
    }

    .error {
        color: #dc3545;
        font-size: 14px;
        margin-top: 5px;
    }

    .success {
        color: #28a745;
        font-size: 14px;
        margin-top: 5px;
    }

    .loading {
        text-align: center;
        padding: 40px;
    }

    .spinner {
        width: 40px;
        height: 40px;
        border: 4px solid #f3f3f3;
        border-top: 4px solid #007bff;
        border-radius: 50%;
        animation: spin 1s linear infinite;
        margin: 0 auto 20px;
    }

    @keyframes spin {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }

    .results-container {
        margin-top: 20px;
    }

    .config-section {
        margin-bottom: 30px;
    }

    .config-section h3 {
        margin-bottom: 15px;
        color: #333;
    }

    .results-table {
        width: 100%;
        border-collapse: collapse;
        margin-bottom: 20px;
    }

    .results-table th,
    .results-table td {
        padding: 12px;
        text-align: left;
        border: 1px solid #ddd;
    }

    .results-table th {
        background: #f8f9fa;
        font-weight: 600;
    }

    .results-table tr:nth-child(even) {
        background: #f8f9fa;
    }

    .migration-status {
        margin-bottom: 20px;
    }

    .migration-status h3 {
        color: #28a745;
        text-align: center;
    }
</style>
