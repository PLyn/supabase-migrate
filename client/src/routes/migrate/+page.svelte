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

    // Track which steps have been completed
    let step1Completed = $state(false);
    let step2Completed = $state(false);
    let step3Completed = $state(false);

    // Derived state using $derived
    let sourceProject = $derived(projects.find((p) => p.id === sourceId));
    let destProject = $derived(projects.find((p) => p.id === destId));
    let sourceValid = $derived(
        sourceId !== "" && sourceProject?.status !== "INACTIVE",
    );
    let destValid = $derived(
        destId !== "" && destProject?.status !== "INACTIVE",
    );
    let sameProjectSelected = $derived(sourceId === destId && sourceId !== "");
    let projectsValid = $derived(
        sourceValid && destValid && !sameProjectSelected,
    );
    let canPreview = $derived(projectsValid && authConfigEnabled);

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
                let error_text = await response.text();
                throw new Error(`HTTP error: ${error_text}`);
            } else {
                let response_json = await response.json();
                return response_json;
            }
        } catch (err) {
            console.error("Get projects error:", err);
            return [];
        } finally {
            loading = false;
        }
    }

    async function handlePreview() {
        if (!canPreview) return;

        try {
            results = await generatePreview(
                sourceId,
                destId,
                authConfigEnabled,
            );
            step3Completed = true;
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

    function toggleAccordion(section: number) {
        // Only allow navigation to sections that have been completed or the current section
        if (
            section === 1 ||
            (section === 2 && step1Completed) ||
            (section === 3 && step2Completed) ||
            (section === 4 && step3Completed)
        ) {
            activeAccordion = section;
        }
    }

    function handleNextStep() {
        if (projectsValid) {
            step1Completed = true;
            toggleAccordion(2);
        }
    }

    function handleConfigNext() {
        if (authConfigEnabled) {
            step2Completed = true;
            handlePreview();
        }
    }

    // Load projects on mount using $effect
    $effect(() => {
        getProjects().then((data) => {
            projects = data;
            if (data.length > 0) {
                sourceId = data[0].id;
                destId = data.length > 1 ? data[1].id : data[0].id;
            }
        });
    });
</script>

<div class="migration-container">
    <header class="header-container">
        <h1>Auth Configuration Migration</h1>
    </header>

    <div class="accordion-wrapper" role="region" aria-label="Migration steps">
        <!-- Project Selection -->
        <div class="accordion-item">
            {#if activeAccordion === 1}
                <div
                    class="accordion-header active"
                    role="heading"
                    aria-level="2"
                >
                    <div class="accordion-header-content">
                        <span class="step-number" aria-hidden="true">1</span>
                        <span class="step-title">Project Selection</span>
                    </div>
                </div>
            {:else}
                <button
                    class="accordion-header"
                    onclick={() => toggleAccordion(1)}
                    aria-expanded="false"
                    aria-controls="panel-1"
                >
                    <div class="accordion-header-content">
                        <span class="step-number" aria-hidden="true">1</span>
                        <div class="step-info">
                            <span class="step-title">Project Selection</span>
                            {#if step1Completed}
                                <span class="step-summary"
                                    >{sourceId} ({sourceId}) --> {destId} ({destId})</span
                                >
                            {/if}
                        </div>
                        <span class="accordion-icon" aria-hidden="true">›</span>
                    </div>
                </button>
            {/if}

            {#if activeAccordion === 1}
                <div class="accordion-content" id="panel-1" role="tabpanel">
                    <div class="content-inner">
                        <div class="content-container narrow">
                            <div class="form-group">
                                <div class="label-row">
                                    <label for="source">Source Project</label>
                                    {#if sourceId && !sourceValid}
                                        <span
                                            class="status-message error"
                                            role="alert"
                                        >
                                            <span aria-hidden="true">×</span> Inactive
                                            project
                                        </span>
                                    {/if}
                                </div>
                                <select
                                    id="source"
                                    bind:value={sourceId}
                                    aria-describedby={sourceId && !sourceValid
                                        ? "source-error"
                                        : undefined}
                                    aria-invalid={sourceId && !sourceValid
                                        ? true
                                        : undefined}
                                >
                                    {#each projects as project}
                                        <option value={project.id}>
                                            {project.name} ({project.id}) - {project.region}
                                            - {project.status}
                                        </option>
                                    {/each}
                                </select>
                            </div>

                            <div class="arrow-container" aria-hidden="true">
                                <svg
                                    class="arrow-icon"
                                    width="24"
                                    height="24"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                >
                                    <path d="M12 5v14M19 12l-7 7-7-7" />
                                </svg>
                            </div>

                            <div class="form-group">
                                <div class="label-row">
                                    <label for="dest">Destination Project</label
                                    >
                                    {#if destId && !destValid}
                                        <span
                                            class="status-message error"
                                            role="alert"
                                        >
                                            <span aria-hidden="true">×</span> Inactive
                                            project
                                        </span>
                                    {:else if sameProjectSelected}
                                        <span
                                            class="status-message error"
                                            role="alert"
                                        >
                                            <span aria-hidden="true">×</span> Same
                                            as source
                                        </span>
                                    {:else if projectsValid}
                                        <span
                                            class="status-message success"
                                            role="status"
                                        >
                                            <span aria-hidden="true">✓</span> Valid
                                            selection
                                        </span>
                                    {/if}
                                </div>
                                <select
                                    id="dest"
                                    bind:value={destId}
                                    aria-describedby={destId &&
                                    (!destValid || sameProjectSelected)
                                        ? "dest-error"
                                        : undefined}
                                    aria-invalid={destId &&
                                    (!destValid || sameProjectSelected)
                                        ? true
                                        : undefined}
                                >
                                    {#each projects as project}
                                        <option value={project.id}>
                                            {project.name} ({project.id}) - {project.region}
                                            - {project.status}
                                        </option>
                                    {/each}
                                </select>
                            </div>

                            <button
                                class="btn-primary"
                                disabled={!projectsValid}
                                onclick={handleNextStep}
                                aria-label="Continue to configuration selection"
                            >
                                Continue
                            </button>
                        </div>
                    </div>
                </div>
            {/if}
        </div>

        <!-- Configuration Selection -->
        <div class="accordion-item">
            {#if activeAccordion === 2}
                <div
                    class="accordion-header active"
                    role="heading"
                    aria-level="2"
                >
                    <div class="accordion-header-content">
                        <span class="step-number" aria-hidden="true">2</span>
                        <span class="step-title"
                            >Select Configuration Items</span
                        >
                    </div>
                </div>
            {:else}
                <button
                    class="accordion-header"
                    disabled={!step1Completed}
                    onclick={() => step1Completed && toggleAccordion(2)}
                    aria-expanded="false"
                    aria-controls="panel-2"
                >
                    <div class="accordion-header-content">
                        <span class="step-number" aria-hidden="true">2</span>
                        <div class="step-info">
                            <span class="step-title"
                                >Select Configuration Items</span
                            >
                            {#if step2Completed}
                                <span class="step-summary">"Auth selected"</span
                                >
                            {/if}
                        </div>
                        <span class="accordion-icon" aria-hidden="true">›</span>
                    </div>
                </button>
            {/if}

            {#if activeAccordion === 2}
                <div class="accordion-content" id="panel-2" role="tabpanel">
                    <div class="content-inner">
                        <div class="content-container narrow">
                            <fieldset class="config-options">
                                <legend class="sr-only"
                                    >Configuration options</legend
                                >
                                <label class="checkbox-label">
                                    <input
                                        type="checkbox"
                                        bind:checked={authConfigEnabled}
                                        aria-describedby="auth-config-desc"
                                    />
                                    <span>Auth Configuration</span>
                                </label>
                                <p
                                    id="auth-config-desc"
                                    class="field-description"
                                >
                                    Migrate authentication settings including
                                    JWT configuration and signup options
                                </p>
                            </fieldset>

                            <button
                                class="btn-primary"
                                onclick={handleConfigNext}
                                disabled={!authConfigEnabled}
                                aria-label="Preview configuration changes"
                            >
                                Preview Changes
                            </button>
                        </div>
                    </div>
                </div>
            {/if}
        </div>

        <!-- Preview Results -->
        <div class="accordion-item">
            {#if activeAccordion === 3}
                <div
                    class="accordion-header active"
                    role="heading"
                    aria-level="2"
                >
                    <div class="accordion-header-content">
                        <span class="step-number" aria-hidden="true">3</span>
                        <span class="step-title">Preview Changes</span>
                    </div>
                </div>
            {:else}
                <button
                    class="accordion-header"
                    disabled={!step2Completed}
                    onclick={() => step2Completed && toggleAccordion(3)}
                    aria-expanded="false"
                    aria-controls="panel-3"
                >
                    <div class="accordion-header-content">
                        <span class="step-number" aria-hidden="true">3</span>
                        <div class="step-info">
                            <span class="step-title">Preview Changes</span>
                            {#if step3Completed && results.length > 0}
                                <span class="step-summary"
                                    >{results.length} configuration{results.length >
                                    1
                                        ? "s"
                                        : ""} reviewed</span
                                >
                            {/if}
                        </div>
                        <span class="accordion-icon" aria-hidden="true">›</span>
                    </div>
                </button>
            {/if}

            {#if activeAccordion === 3}
                <div class="accordion-content" id="panel-3" role="tabpanel">
                    <div class="content-inner">
                        {#if loading}
                            <div
                                class="loading"
                                role="status"
                                aria-live="polite"
                            >
                                <div class="spinner" aria-hidden="true"></div>
                                <p>Loading preview...</p>
                            </div>
                        {:else if results.length > 0}
                            <div class="content-container wide">
                                <div class="results-container">
                                    {#each results as config}
                                        <section class="config-section">
                                            <h3>{config.name}</h3>
                                            <div class="table-wrapper">
                                                <table class="results-table">
                                                    <caption class="sr-only"
                                                        >Configuration
                                                        differences between
                                                        source and destination</caption
                                                    >
                                                    <thead>
                                                        <tr>
                                                            <th
                                                                scope="col"
                                                                class="config-column"
                                                                >Configuration</th
                                                            >
                                                            <th
                                                                scope="col"
                                                                class="value-column"
                                                                >Source Value</th
                                                            >
                                                            <th
                                                                scope="col"
                                                                class="value-column"
                                                                >Destination
                                                                Value</th
                                                            >
                                                        </tr>
                                                    </thead>
                                                    <tbody>
                                                        {#each config.diffs as diff}
                                                            <tr>
                                                                <th
                                                                    scope="row"
                                                                    class="config-column"
                                                                    >{diff.key}</th
                                                                >
                                                                <td
                                                                    class="source-value value-column"
                                                                    >{diff.source_value}</td
                                                                >
                                                                <td
                                                                    class="dest-value value-column"
                                                                    >{diff.dest_value}</td
                                                                >
                                                            </tr>
                                                        {/each}
                                                    </tbody>
                                                </table>
                                            </div>
                                        </section>
                                    {/each}

                                    <button
                                        class="btn-primary btn-migrate"
                                        aria-label="Start migration process"
                                    >
                                        Migrate Configuration
                                    </button>
                                </div>
                            </div>
                        {/if}
                    </div>
                </div>
            {/if}
        </div>

        <!-- Migration Results -->
        <div class="accordion-item">
            {#if activeAccordion === 4}
                <div
                    class="accordion-header active"
                    role="heading"
                    aria-level="2"
                >
                    <div class="accordion-header-content">
                        <span class="step-number" aria-hidden="true">4</span>
                        <span class="step-title">Migration Results</span>
                    </div>
                </div>
            {:else}
                <button
                    class="accordion-header"
                    disabled={!migrationStatus}
                    onclick={() => migrationStatus && toggleAccordion(4)}
                    aria-expanded="false"
                    aria-controls="panel-4"
                >
                    <div class="accordion-header-content">
                        <span class="step-number" aria-hidden="true">4</span>
                        <div class="step-info">
                            <span class="step-title">Migration Results</span>
                            {#if migrationStatus}
                                <span class="step-summary">Completed</span>
                            {/if}
                        </div>
                        <span class="accordion-icon" aria-hidden="true">›</span>
                    </div>
                </button>
            {/if}

            {#if activeAccordion === 4}
                <div class="accordion-content" id="panel-4" role="tabpanel">
                    <div class="content-inner">
                        {#if loading}
                            <div
                                class="loading"
                                role="status"
                                aria-live="polite"
                            >
                                <div class="spinner" aria-hidden="true"></div>
                                <p>Migrating configuration...</p>
                            </div>
                        {:else if migrationStatus}
                            <div class="content-container wide">
                                <div
                                    class="migration-status"
                                    role="alert"
                                    aria-live="polite"
                                >
                                    <h3>Migration Status: {migrationStatus}</h3>
                                </div>

                                {#if results.length > 0}
                                    <div class="results-container">
                                        {#each results as config}
                                            <section class="config-section">
                                                <h3>{config.name}</h3>
                                                <div class="table-wrapper">
                                                    <table
                                                        class="results-table"
                                                    >
                                                        <caption class="sr-only"
                                                            >Migration results</caption
                                                        >
                                                        <thead>
                                                            <tr>
                                                                <th
                                                                    scope="col"
                                                                    class="config-column"
                                                                    >Configuration</th
                                                                >
                                                                <th
                                                                    scope="col"
                                                                    class="value-column"
                                                                    >Status</th
                                                                >
                                                                <th
                                                                    scope="col"
                                                                    class="value-column"
                                                                    >Current
                                                                    Value</th
                                                                >
                                                            </tr>
                                                        </thead>
                                                        <tbody>
                                                            {#each config.diffs as diff}
                                                                <tr>
                                                                    <th
                                                                        scope="row"
                                                                        class="config-column"
                                                                        >{diff.key}</th
                                                                    >
                                                                    <td
                                                                        class="value-column"
                                                                        >{diff.source_value}</td
                                                                    >
                                                                    <td
                                                                        class="value-column"
                                                                        >{diff.dest_value}</td
                                                                    >
                                                                </tr>
                                                            {/each}
                                                        </tbody>
                                                    </table>
                                                </div>
                                            </section>
                                        {/each}
                                    </div>
                                {/if}
                            </div>
                        {/if}
                    </div>
                </div>
            {/if}
        </div>
    </div>
</div>

<style>
    /* Global styles */
    * {
        box-sizing: border-box;
    }

    /* Screen reader only text */
    .sr-only {
        position: absolute;
        width: 1px;
        height: 1px;
        padding: 0;
        margin: -1px;
        overflow: hidden;
        clip: rect(0, 0, 0, 0);
        white-space: nowrap;
        border: 0;
    }

    /* Main container - with max-width constraint */
    .migration-container {
        width: 100%;
        max-width: 1900px;
        margin: 0 auto;
        padding: 0;
        font-family:
            -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
        color: #1a202c;
        line-height: 1.6;
    }

    /* Header container for centered title */
    .header-container {
        max-width: 1900px;
        margin: 0 auto;
        padding: 1rem;
    }

    /* Typography */
    h1 {
        text-align: center;
        color: #1a202c;
        margin: 0 0 2rem;
        font-size: clamp(1.5rem, 4vw, 2rem);
        font-weight: 600;
    }

    h3 {
        margin: 0 0 1rem;
        color: #2d3748;
        font-size: 1.125rem;
        font-weight: 600;
    }

    /* Accordion wrapper - full width */
    .accordion-wrapper {
        width: 100%;
    }

    /* Accordion items - full width within container with subtle borders */
    .accordion-item {
        margin: 0 0 0.75rem 0;
        background: white;
        border-radius: 0.5rem;
        box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
        overflow: hidden;
        width: 100%;
    }

    /* Accordion header - full width */
    .accordion-header {
        width: 100%;
        padding: 0;
        background: #f7fafc;
        border: none;
        text-align: left;
        font-size: 1rem;
        font-weight: 500;
        transition: all 0.2s ease;
        cursor: pointer;
        display: block;
    }

    /* Header content with max-width */
    .accordion-header-content {
        max-width: 1900px;
        margin: 0 auto;
        padding: 1rem 1.5rem;
        display: flex;
        align-items: center;
        gap: 0.75rem;
        position: relative;
    }

    .step-info {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        flex: 1;
    }

    .step-title {
        font-weight: 500;
        line-height: 1.4;
    }

    .step-summary {
        font-size: 0.875rem;
        color: #718096;
        font-weight: 400;
        line-height: 1.4;
    }

    .accordion-header.active .step-summary {
        color: rgba(255, 255, 255, 0.9);
    }

    .step-number {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        width: 1.75rem;
        height: 1.75rem;
        background: #e2e8f0;
        color: #4a5568;
        border-radius: 50%;
        font-size: 0.875rem;
        font-weight: 600;
        flex-shrink: 0;
    }

    button.accordion-header:hover:not(:disabled) {
        background: #edf2f7;
    }

    button.accordion-header:focus-visible {
        outline: 2px solid #4299e1;
        outline-offset: -2px;
    }

    .accordion-header.active {
        background: #4299e1;
        color: white;
    }

    .accordion-header.active .step-number {
        background: white;
        color: #4299e1;
    }

    .accordion-header:disabled {
        color: #a0aec0;
        cursor: not-allowed;
    }

    .accordion-header:disabled .step-number {
        background: #e2e8f0;
        color: #a0aec0;
    }

    .accordion-header:disabled .step-summary {
        color: #cbd5e0;
    }

    .accordion-icon {
        margin-left: auto;
        font-size: 1.25rem;
        transition: transform 0.2s ease;
        flex-shrink: 0;
    }

    /* Content - full width background */
    .accordion-content {
        background: white;
        border-top: 1px solid #e2e8f0;
        width: 100%;
    }

    /* Content inner - contains max-width constraint */
    .content-inner {
        max-width: 1900px;
        margin: 0 auto;
        padding: 1.5rem;
        width: 100%;
    }

    /* Content containers for different widths */
    .content-container {
        margin: 0 auto;
    }

    .content-container.narrow {
        max-width: 32rem;
    }

    .content-container.wide {
        max-width: 100%;
    }

    /* Forms */
    .form-group {
        margin-bottom: 2rem;
        position: relative;
    }

    .form-group::after {
        content: "";
        position: absolute;
        bottom: -1rem;
        left: 50%;
        transform: translateX(-50%);
        width: 40px;
        height: 1px;
        background: linear-gradient(
            to right,
            transparent,
            #cbd5e0,
            transparent
        );
    }

    .form-group:last-of-type::after {
        display: none;
    }

    .label-row {
        display: flex;
        align-items: center;
        justify-content: space-between;
        margin-bottom: 0.5rem;
        min-height: 1.5rem;
    }

    label {
        font-weight: 600;
        color: #2d3748;
        font-size: 0.9375rem;
        letter-spacing: -0.01em;
    }

    .status-message {
        font-size: 0.8125rem;
        font-weight: 600;
        display: flex;
        align-items: center;
        gap: 0.375rem;
        padding: 0.25rem 0.5rem;
        border-radius: 0.25rem;
        letter-spacing: -0.01em;
    }

    .status-message.error {
        color: #e53e3e;
        background-color: rgba(229, 62, 62, 0.08);
    }

    .status-message.success {
        color: #38a169;
        background-color: rgba(56, 161, 105, 0.08);
    }

    select {
        width: 100%;
        padding: 0.75rem 1rem;
        padding-right: 2.5rem;
        border: 2px solid #e2e8f0;
        border-radius: 0.5rem;
        font-size: 0.9375rem;
        font-weight: 500;
        background: white;
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='24' height='24' viewBox='0 0 24 24' fill='none' stroke='%234a5568' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
        background-repeat: no-repeat;
        background-position: right 0.75rem center;
        background-size: 1.25rem;
        appearance: none;
        transition: all 0.2s ease;
        cursor: pointer;
        line-height: 1.5;
    }

    select:hover:not(:disabled) {
        border-color: #cbd5e0;
        background-color: #f7fafc;
    }

    select:focus {
        outline: none;
        border-color: #4299e1;
        box-shadow:
            0 0 0 3px rgba(66, 153, 225, 0.1),
            0 1px 2px rgba(0, 0, 0, 0.05);
        background-color: white;
    }

    select[aria-invalid="true"] {
        border-color: #e53e3e;
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='24' height='24' viewBox='0 0 24 24' fill='none' stroke='%23e53e3e' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
    }

    /* Option styling - limited browser support */
    select option {
        padding: 1rem 1.25rem;
        line-height: 1.8;
        font-size: 0.9375rem;
        font-weight: 500;
        color: #2d3748;
        background-color: white;
    }

    /* These may not work in all browsers */
    select option:hover,
    select option:focus,
    select option:checked {
        background-color: #edf2f7;
        color: #1a202c;
        box-shadow: inset 0 0 0 2px #4299e1;
    }

    select option:disabled {
        color: #a0aec0;
        font-style: italic;
    }

    /* For webkit browsers (Chrome, Safari) - custom scrollbar */
    select::-webkit-scrollbar {
        width: 12px;
    }

    select::-webkit-scrollbar-track {
        background: #f7fafc;
        border-radius: 0 0.5rem 0.5rem 0;
    }

    select::-webkit-scrollbar-thumb {
        background: #cbd5e0;
        border-radius: 6px;
        border: 2px solid #f7fafc;
    }

    select::-webkit-scrollbar-thumb:hover {
        background: #a0aec0;
    }

    /* Firefox specific */
    @-moz-document url-prefix() {
        select option {
            padding: 0.75rem 1rem;
        }
    }

    /* Arrow */
    .arrow-container {
        text-align: center;
        margin: 1.5rem 0;
    }

    .arrow-icon {
        color: #4299e1;
        animation: bounce 2s infinite;
    }

    @keyframes bounce {
        0%,
        100% {
            transform: translateY(0);
        }
        50% {
            transform: translateY(0.5rem);
        }
    }

    /* Checkbox */
    fieldset {
        border: none;
        margin: 0;
        padding: 0;
    }

    .config-options {
        margin-bottom: 1.5rem;
    }

    .checkbox-label {
        display: flex;
        align-items: center;
        padding: 0.75rem;
        border: 1px solid #cbd5e0;
        border-radius: 0.375rem;
        cursor: pointer;
        transition: all 0.2s ease;
    }

    .checkbox-label:hover {
        background: #f7fafc;
        border-color: #4299e1;
    }

    .checkbox-label input[type="checkbox"] {
        margin-right: 0.75rem;
        width: 1.125rem;
        height: 1.125rem;
        cursor: pointer;
    }

    .field-description {
        font-size: 0.813rem;
        color: #718096;
        margin: 0.5rem 0 0 2.625rem;
    }

    /* Buttons */
    .btn-primary {
        background: #4299e1;
        color: white;
        border: none;
        padding: 0.75rem 2rem;
        border-radius: 0.375rem;
        cursor: pointer;
        font-size: 0.875rem;
        font-weight: 500;
        transition: all 0.2s ease;
        display: block;
        margin: 1.5rem auto 0;
        min-width: 12rem;
    }

    .btn-primary:hover:not(:disabled) {
        background: #3182ce;
        transform: translateY(-1px);
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    }

    .btn-primary:focus-visible {
        outline: 2px solid #4299e1;
        outline-offset: 2px;
    }

    .btn-primary:disabled {
        background: #cbd5e0;
        color: #718096;
        cursor: not-allowed;
        transform: none;
        box-shadow: none;
    }

    .btn-migrate {
        width: 100%;
        max-width: 20rem;
        margin: 2rem auto 0;
    }

    /* Loading */
    .loading {
        text-align: center;
        padding: 3rem;
    }

    .spinner {
        width: 2.5rem;
        height: 2.5rem;
        border: 3px solid #e2e8f0;
        border-top-color: #4299e1;
        border-radius: 50%;
        animation: spin 0.8s linear infinite;
        margin: 0 auto 1rem;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }

    /* Results */
    .results-container {
        margin-top: 1.5rem;
    }

    .config-section {
        margin-bottom: 2rem;
    }

    .table-wrapper {
        overflow-x: auto;
        border-radius: 0.5rem;
        box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    }

    .results-table {
        width: 100%;
        border-collapse: collapse;
        font-size: 0.875rem;
        table-layout: fixed;
    }

    .results-table th,
    .results-table td {
        padding: 0.75rem 1rem;
        text-align: left;
        border-bottom: 1px solid #e2e8f0;
        vertical-align: top;
    }

    /* Column width control */
    .config-column {
        width: 20%;
        word-wrap: break-word;
        word-break: break-word;
        hyphens: auto;
        min-width: 120px;
    }

    .value-column {
        width: 40%;
        word-wrap: break-word;
        word-break: break-word;
    }

    .results-table thead th {
        background: #f7fafc;
        font-weight: 600;
        color: #2d3748;
        border-bottom: 2px solid #e2e8f0;
    }

    .results-table tbody th {
        font-weight: 500;
        color: #4a5568;
    }

    .results-table tbody tr:nth-child(even) {
        background: #f7fafc;
    }

    .results-table tbody tr:hover {
        background: #edf2f7;
    }

    .results-table tbody tr:last-child td,
    .results-table tbody tr:last-child th {
        border-bottom: none;
    }

    .source-value {
        color: #38a169;
    }

    .dest-value {
        color: #e53e3e;
    }

    /* Status */
    .migration-status {
        text-align: center;
        padding: 2rem;
        background: #f0fff4;
        border-radius: 0.5rem;
        margin-bottom: 2rem;
    }

    .migration-status h3 {
        color: #38a169;
        margin: 0;
    }

    /* Responsive */
    @media (max-width: 768px) {
        .step-info {
            max-width: calc(100vw - 8rem);
        }

        .step-summary {
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }
    }

    @media (max-width: 640px) {
        .header-container {
            padding: 0.5rem;
        }

        .accordion-header-content {
            padding: 0.875rem 1rem;
            font-size: 0.875rem;
        }

        .content-inner {
            padding: 1rem;
        }

        .btn-primary {
            width: 100%;
            max-width: none;
        }

        .results-table {
            font-size: 0.75rem;
        }

        .results-table th,
        .results-table td {
            padding: 0.5rem 0.75rem;
        }

        /* Adjust column widths for mobile */
        .config-column {
            width: 25%;
            min-width: 100px;
        }

        .value-column {
            width: 37.5%;
        }

        .step-info {
            max-width: calc(100vw - 6rem);
        }
    }

    /* High contrast mode support */
    @media (prefers-contrast: high) {
        .accordion-item {
            border: 1px solid currentColor;
        }

        .btn-primary:focus-visible {
            outline-width: 3px;
        }
    }

    /* Reduced motion support */
    @media (prefers-reduced-motion: reduce) {
        * {
            animation-duration: 0.01ms !important;
            animation-iteration-count: 1 !important;
            transition-duration: 0.01ms !important;
        }
    }
</style>
