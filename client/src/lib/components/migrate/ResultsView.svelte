<!-- src/lib/components/migrate/ResultsView.svelte -->
<script lang="ts">
	import type { ProjectConfig } from '$lib/api';

	let {
		results = [],
		sourceHeading,
		destHeading
	}: {
		results: ProjectConfig[];
		sourceHeading: string;
		destHeading: string;
	} = $props();
</script>

<div class="max-w-8xl w-full space-y-6">
	{#each results as projectConfig}
		<div class="overflow-x-auto">
			<h2 class="mb-2 text-lg font-bold">{projectConfig.name}</h2>
			<table class="table w-full border-collapse border border-black">
				<thead>
					<tr>
						<th class="border border-black bg-gray-300 p-2 text-center">Config Item</th>
						<th class="border border-black bg-gray-300 p-2 text-center">{sourceHeading}</th>
						<th class="border border-black bg-gray-300 p-2 text-center">{destHeading}</th>
					</tr>
				</thead>
				<tbody>
					{#each projectConfig.diffs as diff}
						<tr class="hover:bg-gray-200">
							<td class="border border-black p-2 text-left">{diff.key}</td>
							<td class="border border-black p-2">{diff.source_value}</td>
							<td class="border border-black p-2">{diff.dest_value}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/each}
</div>
