<!-- src/routes/migrate/+page.svelte -->
<script lang="ts">
	import { onMount } from 'svelte';
	import { serverFunctions, type Project, type ProjectConfig } from '$lib/api';
	import {
		projects,
		selectedSourceId,
		selectedDestId,
		migrationResults,
		migrationStatus,
		configItems
	} from '$lib/stores';
	import AnimatedArrow from '$lib/components/AnimatedArrow.svelte';
	import ProjectSelectView from '$lib/components/migrate/ProjectSelectView.svelte';
	import ConfigSelectView from '$lib/components/migrate/ConfigSelectView.svelte';
	import ResultsView from '$lib/components/migrate/ResultsView.svelte';
	import LoadingView from '$lib/components/migrate/LoadingView.svelte';

	let currentStep = $state('Projects');
	let projectsList = $state<Project[]>([]);
	let sourceId = $state('');
	let destId = $state('');
	let results = $state<ProjectConfig[]>([]);
	let status = $state('Migration Status: Migration has not been run');
	let configItemsState = $state({
		Auth: false,
		Postgrest: false,
		EdgeFunctions: false,
		Secrets: false,
		Storage: false,
		Postgres: false,
		Branches: false
	});

	onMount(async () => {
		try {
			const projectsData = await serverFunctions.getProjects();
			projectsList = projectsData;
			if (projectsData.length > 0) {
				sourceId = projectsData[0].id;
				destId = projectsData[0].id;
			}
		} catch (error) {
			console.error('Failed to load projects:', error);
		}
	});

	function nextStep() {
		currentStep = 'Config';
	}

	async function generatePreview() {
		currentStep = 'Loading';
		try {
			const configArray = [
				configItemsState.Auth,
				configItemsState.Postgrest,
				configItemsState.EdgeFunctions,
				configItemsState.Secrets,
				configItemsState.Storage,
				configItemsState.Postgres,
				configItemsState.Branches
			];

			const previewResults = await serverFunctions.generatePreview(sourceId, destId, configArray);
			results = previewResults;
			currentStep = 'Preview';
		} catch (error) {
			console.error('Failed to generate preview:', error);
			results = [];
			currentStep = 'Config';
		}
	}

	async function runMigration() {
		currentStep = 'Loading';
		try {
			const migrationResults = await serverFunctions.migrateConfig(results, sourceId, destId);
			results = migrationResults;
			status = 'Success';
			currentStep = 'Results';
		} catch (error) {
			console.error('Migration failed:', error);
			results = [];
			status = error instanceof Error ? error.message : 'Migration failed';
			currentStep = 'Results';
		}
	}
</script>

<div class="container mx-auto px-4">
	{#if currentStep === 'Projects'}
		<ProjectSelectView bind:sourceId bind:destId {projectsList} onNext={nextStep} />
	{:else if currentStep === 'Config'}
		<ConfigSelectView
			bind:configItems={configItemsState}
			{sourceId}
			{destId}
			onBack={() => (currentStep = 'Projects')}
			onPreview={generatePreview}
		/>
	{:else if currentStep === 'Loading'}
		<LoadingView />
	{:else if currentStep === 'Preview'}
		<div class="mt-4 flex flex-col p-4">
			<button
				class="btn btn-secondary mx-auto mb-4 max-w-sm"
				onclick={() => (currentStep = 'Config')}
			>
				Back
			</button>
			<h3 class="mb-4 text-2xl font-bold">Preview Results</h3>
			<ResultsView {results} sourceHeading="Source" destHeading="Destination" />
			<button class="btn btn-primary mx-auto mt-4 max-w-sm" onclick={runMigration}>
				Migrate Configuration!
			</button>
		</div>
	{:else if currentStep === 'Results'}
		<div class="flex min-h-screen flex-col items-center justify-center p-4">
			<h3 class="text-1xl my-4 font-bold">{status}</h3>
			<h3 class="mb-4 text-2xl font-bold">Migration Results</h3>
			<ResultsView
				{results}
				sourceHeading="Config change to migrate"
				destHeading="Current config after migration"
			/>
		</div>
	{/if}
</div>
