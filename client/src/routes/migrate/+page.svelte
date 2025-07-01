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
            const response = await fetch('http://localhost:3000/api/preview', {
                method: 'GET',
                headers: {
                    'Content-Type': 'application/json',
                    // Add any auth headers if needed
                    // 'Authorization': `Bearer ${token}`
                },
                // Add CORS mode if needed
                mode: 'cors'
            });

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }

            const data = await response.json();
            previewData = data;
            console.log('Preview data:', data);
            
        } catch (err) {
            console.error('Error fetching preview:', err);
            error = err instanceof Error ? err.message : 'An unknown error occurred';
        } finally {
            isLoading = false;
        }
    }
</script>

<div class="container mx-auto px-4">
    <label for="button">Generate Preview</label>
    <button 
        id="button" 
        onclick={fetchPreview}
        disabled={isLoading}
        class="ml-2 px-4 py-2 bg-blue-500 text-white rounded disabled:opacity-50"
    >
        {isLoading ? 'Generating...' : 'Generate'}
    </button>

    {#if error}
        <div class="mt-4 p-4 bg-red-100 border border-red-400 text-red-700 rounded">
            Error: {error}
        </div>
    {/if}

    {#if previewData}
        <div class="mt-4 p-4 bg-green-100 border border-green-400 text-green-700 rounded">
            <h3 class="font-bold">Preview Data:</h3>
            <pre class="mt-2 text-sm">{JSON.stringify(previewData, null, 2)}</pre>
        </div>
    {/if}
</div>