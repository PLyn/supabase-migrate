<!-- src/lib/components/migrate/ConfigSelectView.svelte -->
<script lang="ts">
	interface ConfigItems {
		Auth: boolean;
		Postgrest: boolean;
		EdgeFunctions: boolean;
		Secrets: boolean;
		Storage: boolean;
		Postgres: boolean;
		Branches: boolean;
	}

	let {
		configItems = $bindable(),
		sourceId,
		destId,
		onBack,
		onPreview
	}: {
		configItems: ConfigItems;
		sourceId: string;
		destId: string;
		onBack: () => void;
		onPreview: () => void;
	} = $props();

	const hasSelection = $derived(() => {
		return Object.values(configItems).some((value) => value);
	});
</script>

<div class="mx-auto flex w-full max-w-screen-sm flex-col items-center p-4">
	<button class="btn btn-secondary my-4" onclick={onBack}>Back</button>

	<div class="mx-auto w-full max-w-xs">
		<label class="flex w-full cursor-pointer items-center justify-between py-2">
			<span>Auth</span>
			<input class="checkbox checkbox-success" type="checkbox" bind:checked={configItems.Auth} />
		</label>
		<div class="divider"></div>

		<label class="flex w-full cursor-pointer items-center justify-between py-2">
			<span>Postgrest</span>
			<input
				class="checkbox checkbox-success"
				type="checkbox"
				bind:checked={configItems.Postgrest}
			/>
		</label>
		<div class="divider"></div>

		<label class="flex w-full cursor-pointer items-center justify-between py-2">
			<span>EdgeFunctions</span>
			<input
				class="checkbox checkbox-success"
				type="checkbox"
				bind:checked={configItems.EdgeFunctions}
			/>
		</label>
		<div class="divider"></div>

		<label class="flex w-full cursor-pointer items-center justify-between py-2">
			<span>Secrets</span>
			<input class="checkbox checkbox-success" type="checkbox" bind:checked={configItems.Secrets} />
		</label>
		<div class="divider"></div>

		<label class="flex w-full cursor-pointer items-center justify-between py-2">
			<span>Storage</span>
			<input class="checkbox checkbox-success" type="checkbox" bind:checked={configItems.Storage} />
		</label>
		<div class="divider"></div>

		<label class="flex w-full cursor-pointer items-center justify-between py-2">
			<span>Postgres</span>
			<input
				class="checkbox checkbox-success"
				type="checkbox"
				bind:checked={configItems.Postgres}
			/>
		</label>
		<div class="divider"></div>

		<label class="flex w-full cursor-pointer items-center justify-between py-2">
			<span>Branches</span>
			<input
				class="checkbox checkbox-success"
				type="checkbox"
				bind:checked={configItems.Branches}
			/>
		</label>
	</div>

	{#if hasSelection()}
		<button class="btn btn-primary mt-4" onclick={onPreview}> Preview Changes </button>
	{/if}
</div>
