<!-- src/lib/components/migrate/ProjectSelectView.svelte -->
<script>
	import AnimatedArrow from '../AnimatedArrow.svelte';

	let { sourceId = $bindable(), destId = $bindable(), projectsList = [], onNext } = $props();

	$effect(() => {
		if (projectsList.length > 0 && !sourceId) {
			sourceId = projectsList[0].id;
			destId = projectsList[0].id;
		}
	});

	const validateSource = $derived(() => {
		if (!sourceId) {
			return 'Please select a source project.';
		}
		const project = projectsList.find((p) => p.id === sourceId);
		if (project?.status === 'INACTIVE') {
			return 'Selected source project is INACTIVE.';
		}
		return '';
	});

	const validateDestination = $derived(() => {
		if (!destId) {
			return 'Please select a destination project.';
		}
		const project = projectsList.find((p) => p.id === destId);
		if (project?.status === 'INACTIVE') {
			return 'Selected destination project is INACTIVE.';
		}
		return '';
	});

	const validateForm = $derived(() => {
		if (sourceId && destId && sourceId === destId) {
			return 'Source and destination projects cannot be the same.';
		}
		return '';
	});

	const isValidated = $derived(() => {
		return !validateSource && !validateDestination && !validateForm;
	});
</script>

<div class="mx-auto mt-4 flex w-full max-w-screen-lg flex-col items-center">
	<h1 class="mb-4 text-xl font-bold">Migrate Project configuration</h1>

	<label for="source" class="mb-4 text-lg">Source Project</label>
	<select id="source" class="select select-info mb-4 w-full" bind:value={sourceId}>
		{#each projectsList as project}
			<option value={project.id}>
				{project.id} - {project.name} - {project.region} - {project.status}
			</option>
		{/each}
	</select>

	<AnimatedArrow />

	<label for="dest" class="my-4 text-lg">Destination Project</label>
	<select id="dest" class="select select-info mb-4 w-full" bind:value={destId}>
		{#each projectsList as project}
			<option value={project.id}>
				{project.id} - {project.name} - {project.region} - {project.status}
			</option>
		{/each}
	</select>

	{#if !isValidated}
		<ul>
			{#if validateForm}<li><p style="color: red;">{validateForm}</p></li>{/if}
			{#if validateSource}<li><p style="color: red;">{validateSource}</p></li>{/if}
			{#if validateDestination}<li><p style="color: red;">{validateDestination}</p></li>{/if}
		</ul>
	{:else}
		<p style="color: green;">No Validation Errors Found. You may proceed!</p>
		<button class="btn btn-primary mt-4" onclick={onNext}>Next</button>
	{/if}
</div>
